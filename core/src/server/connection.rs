use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::{Instant, Duration}};
use futures_util::{SinkExt, StreamExt};
use log::{info, error, debug};
use tokio::sync::mpsc;
use tokio_tungstenite::accept_async;
use shared::server::message::{Message, Status};
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    server::message_handler::process_message,
};

pub async fn handle_connection(
    stream: tokio::net::TcpStream,
    addr: SocketAddr,
    handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
) {
    let ws_stream = match accept_async(stream).await {
        Ok(stream) => stream,
        Err(e) => {
            error!("WebSocket handshake failed for {}: {}", addr, e);
            return;
        }
    };

    info!("New WebSocket connection from {}", addr);
    let ctx = ConnectionContext::new(addr.ip().to_string());
    run_message_loop(ws_stream, addr, handlers, ctx).await;
    info!("Connection closed: {}", addr);
}

pub async fn run_message_loop(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    addr: SocketAddr,
    handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    mut ctx: ConnectionContext,
) {
    const TIMEOUT_DURATION: Duration = Duration::from_secs(60);
    let mut last_message_time = Instant::now();

    // split sink/stream and create a channel for outgoing messages
    let (mut sink, mut stream) = ws_stream.split();
    let (tx, mut rx) = mpsc::channel::<tokio_tungstenite::tungstenite::Message>(32);

    // writer task: send outgoing messages from channel
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
                    tokio_tungstenite::tungstenite::Message::Close(_) => {
                        let _ = tx.send(tokio_tungstenite::tungstenite::Message::Close(None)).await;
                        break;
                    }
                    tokio_tungstenite::tungstenite::Message::Ping(_) | tokio_tungstenite::tungstenite::Message::Pong(_) => {
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
    let _ = writer.await;
}

pub async fn parse_message_from_ws(msg: tokio_tungstenite::tungstenite::Message) -> Option<String> {
    match msg {
        tokio_tungstenite::tungstenite::Message::Text(text) => Some(text),
        tokio_tungstenite::tungstenite::Message::Binary(data) => {
            String::from_utf8(data).ok()
        }
        _ => None,
    }
}

pub fn response_to_ws(response: Message) -> tokio_tungstenite::tungstenite::Message {
    let json = serde_json::to_string(&response).unwrap_or_else(|_| {
        "{\"type\":\"response\",\"id\":0,\"status\":\"error\",\"code\":500,\"message\":\"Failed to serialize response\"}".to_string()
    });

    tokio_tungstenite::tungstenite::Message::Text(json)
}