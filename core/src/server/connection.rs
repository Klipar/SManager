use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::{Instant, Duration}};
use futures_util::{SinkExt, StreamExt};
use log::{info, error, debug};
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use shared::server::message::{Message, Status};
use tokio_tungstenite::tungstenite::Message as TMessage;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::{connection_context::ConnectionContext, connection_registry::ConnectionRegistry, message_handler::process_message},
};

pub async fn handle_connection(
    stream: tokio::net::TcpStream,
    addr: SocketAddr,
    handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    registry: Arc<ConnectionRegistry>,
    conn_id: u64,
) {
    let ws_stream = match accept_async(stream).await {
        Ok(s) => s,
        Err(e) => { error!("Handshake failed for {}: {}", addr, e); return; }
    };

    info!("New WebSocket connection from {} (id={})", addr, conn_id);
    let ctx = ConnectionContext::new(addr.ip().to_string(), conn_id);
    run_message_loop(ws_stream, addr, handlers, registry, conn_id, ctx).await;
    info!("Connection closed: {} (id={})", addr, conn_id);
}

pub async fn run_message_loop(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    addr: SocketAddr,
    handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    registry: Arc<ConnectionRegistry>,
    conn_id: u64,
    mut ctx: ConnectionContext,
) {
    const TIMEOUT_DURATION: Duration = Duration::from_secs(60);
    let mut last_message_time = Instant::now();

    let (mut sink, mut stream) = ws_stream.split();
    let (tx, mut rx) = mpsc::channel::<TMessage>(32);

    registry.register(conn_id, tx.clone()).await;

    let writer = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = sink.send(msg).await {
                debug!("WebSocket writer send error to {}: {}", addr, e);
                break;
            }
        }
        debug!("WebSocket writer exiting for {}", addr);
    });

    // reader loop: process incoming messages and push responses to tx
    loop {
        tokio::select! {
            _ = tokio::time::sleep(TIMEOUT_DURATION) => {
                if last_message_time.elapsed() > TIMEOUT_DURATION {
                    debug!("Connection timeout for {}: no data for 60 seconds", addr);
                    break;
                }
            }

            result = stream.next() => {
                let Some(result) = result else {
                    debug!("WebSocket stream ended for {}", addr);
                    break;
                };

                let msg = match result {
                    Ok(m) => m,
                    Err(e) => {
                        error!("WebSocket read error from {}: {}", addr, e);
                        break;
                    }
                };

                last_message_time = Instant::now();

                match &msg {
                    TMessage::Close(_) => {
                        let _ = tx.send(TMessage::Close(None)).await;
                        break;
                    }
                    TMessage::Ping(_) | TMessage::Pong(_) => {
                        continue;
                    }
                    _ => {}
                }

                let raw_text = match parse_message_from_ws(msg).await {
                    Some(text) => text,
                    None => {
                        let error_response = Message::new_response(
                            Status::Error,
                            None,
                            400,
                            "Invalid message payload",
                        );
                        let _ = tx.send(response_to_ws(error_response)).await;
                        continue;
                    }
                };

                let message = match serde_json::from_str::<Message>(&raw_text) {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("Failed to parse JSON from {}: {}", addr, e);
                        let error_response = Message::new_response(
                            Status::Error,
                            None,
                            400,
                            "Invalid JSON format",
                        );
                        let _ = tx.send(response_to_ws(error_response)).await;
                        continue;
                    }
                };

                let response = process_message(message, &handlers, &mut ctx, addr).await;

                if tx.send(response_to_ws(response)).await.is_err() {
                    debug!("Outgoing channel closed for {}, stopping reader", addr);
                    break;
                }
            }
        }
    }

    drop(tx);
    registry.unregister(conn_id).await;
    let _ = writer.await;
}

pub async fn parse_message_from_ws(msg: TMessage) -> Option<String> {
    match msg {
        TMessage::Text(text) => Some(text),
        TMessage::Binary(data) => {
            String::from_utf8(data).ok()
        }
        _ => None,
    }
}

pub fn response_to_ws(response: Message) -> TMessage {
    let json = serde_json::to_string(&response).unwrap_or_else(|_| {
        "{\"type\":\"response\",\"id\":0,\"status\":\"error\",\"code\":500,\"message\":\"Failed to serialize response\"}".to_string()
    });

    TMessage::Text(json)
}