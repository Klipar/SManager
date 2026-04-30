use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use futures_util::{SinkExt, StreamExt};
use log::{info, error};
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
    mut ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    addr: SocketAddr,
    handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    mut ctx: ConnectionContext,
) {
    while let Some(result) = ws_stream.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!("WebSocket read error from {}: {}", addr, e);
                break;
            }
        };

        let raw_text = match parse_message_from_ws(msg).await {
            Some(text) => text,
            None => {
                let error_response = Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid message payload",
                );
                if let Err(e) = ws_stream.send(response_to_ws(error_response)).await {
                    error!("Failed to send error response to {}: {}", addr, e);
                }
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
                if let Err(e) = ws_stream.send(response_to_ws(error_response)).await {
                    error!("Failed to send error response to {}: {}", addr, e);
                }
                continue;
            }
        };

        let response = process_message(message, &handlers, &mut ctx, addr).await;

        if let Err(e) = ws_stream.send(response_to_ws(response)).await {
            error!("WebSocket send error to {}: {}", addr, e);
            break;
        }
    }
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