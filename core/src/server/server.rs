use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use serde_json::Value;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

use shared::server::{
    connection_context::ConnectionContext,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};

pub struct Server {
    pub ip: String,
    pub port: u16,
    pub is_active: bool,
    pub handlers: HashMap<String, Arc<dyn HandlerTrait>>,
}

#[derive(Debug, Deserialize)]
struct AuthClaims {
    sub: i32,
    is_admin: bool,
}

impl Server {
    pub fn new(ip: String, port: u16) -> Self {
        Self {
            ip,
            port,
            is_active: false,
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler(&mut self, name: &str, handler: Arc<dyn HandlerTrait>) {
        self.handlers.insert(name.to_string(), handler);
    }

    pub async fn start_server(&mut self) {
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port)).await.unwrap();
        println!("Server running on ws://{}:{}", self.ip, self.port);

        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(Self::handle_connection(stream, addr, self.handlers.clone()));
        }
    }

    async fn handle_connection(
        stream: tokio::net::TcpStream,
        addr: SocketAddr,
        handlers: HashMap<String, Arc<dyn HandlerTrait>>,
    ) {
        let mut ws_stream = accept_async(stream)
            .await
            .expect("Error during WebSocket handshake");

        println!("New WebSocket connection");
        let mut ctx = ConnectionContext::new(addr.ip().to_string());

        while let Some(result) = ws_stream.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("Read ws error {}: {}", addr, e);
                    return;
                }
            };

            if msg.is_text() || msg.is_binary() {
                let raw = if msg.is_text() {
                    match msg.to_text() {
                        Ok(text) => text.to_string(),
                        Err(_) => {
                            let response = Message::new_response(
                                Status::Error,
                                None,
                                400,
                                "Invalid text payload",
                            );
                            let _ = ws_stream.send(response_to_ws(response)).await;
                            continue;
                        }
                    }
                } else {
                    match String::from_utf8(msg.into_data().to_vec()) {
                        Ok(text) => text,
                        Err(_) => {
                            let response = Message::new_response(
                                Status::Error,
                                None,
                                400,
                                "Invalid binary payload",
                            );
                            let _ = ws_stream.send(response_to_ws(response)).await;
                            continue;
                        }
                    }
                };

                let parsed = match serde_json::from_str::<Message>(&raw) {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        let response = Message::new_response(
                            Status::Error,
                            None,
                            400,
                            "Invalid message format",
                        );
                        let _ = ws_stream.send(response_to_ws(response)).await;
                        continue;
                    }
                };

                let response = match parsed {
                    Message::Request { id, action, data } => {
                        if action != "login" {
                            let token = match extract_token(&data) {
                                Some(token) => token,
                                None => {
                                    let mut response = Message::new_response(
                                        Status::Error,
                                        None,
                                        401,
                                        "Missing token",
                                    );
                                    response.set_id(id);
                                    if let Err(e) = ws_stream.send(response_to_ws(response)).await {
                                        eprintln!("Write ws error {}: {}", addr, e);
                                    }
                                    continue;
                                }
                            };

                            match verify_token(&token) {
                                Ok(claims) => {
                                    ctx.user_id = Some(claims.sub);
                                    ctx.is_admin = claims.is_admin;
                                    ctx.id = Some(claims.sub);
                                }
                                Err(_) => {
                                    let mut response = Message::new_response(
                                        Status::Error,
                                        None,
                                        401,
                                        "Invalid token",
                                    );
                                    response.set_id(id);
                                    if let Err(e) = ws_stream.send(response_to_ws(response)).await {
                                        eprintln!("Write ws error {}: {}", addr, e);
                                    }
                                    continue;
                                }
                            }

                            if !is_action_allowed(&action, &data, &ctx) {
                                let mut response = Message::new_response(
                                    Status::Error,
                                    None,
                                    403,
                                    "Forbidden",
                                );
                                response.set_id(id);
                                if let Err(e) = ws_stream.send(response_to_ws(response)).await {
                                    eprintln!("Write ws error {}: {}", addr, e);
                                }
                                continue;
                            }
                        }

                        let mut response = if let Some(handler) = handlers.get(&action) {
                            handler.handle(data, &mut ctx).await
                        } else {
                            Message::new_response(
                                Status::Error,
                                None,
                                404,
                                format!("Unknown action: {}", action),
                            )
                        };

                        response.set_id(id);
                        response
                    }
                    Message::Response { .. } => Message::new_response(
                        Status::Error,
                        None,
                        400,
                        "Response messages are not accepted on this socket",
                    ),
                };

                if let Err(e) = ws_stream.send(response_to_ws(response)).await {
                    eprintln!("Write ws error {}: {}", addr, e);
                    return;
                }
            }
        }
    }
}

fn extract_token(data: &Option<Value>) -> Option<String> {
    data.as_ref()?
        .get("token")?
        .as_str()
        .map(|v| v.to_string())
}

fn get_target_user_id(data: &Option<Value>) -> Option<i32> {
    data.as_ref()?
        .get("id")?
        .as_i64()
        .map(|id| id as i32)
}

fn is_action_allowed(action: &str, data: &Option<Value>, ctx: &ConnectionContext) -> bool {
    if ctx.is_admin {
        return true;
    }

    match action {
        "update-user" | "remove-user" => match (ctx.user_id, get_target_user_id(data)) {
            (Some(current_user_id), Some(target_user_id)) => current_user_id == target_user_id,
            _ => false,
        },
        _ => false,
    }
}

fn verify_token(token: &str) -> Result<AuthClaims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    let mut validation = Validation::default();
    validation.required_spec_claims.remove("exp");

    decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
}

fn response_to_ws(response: Message) -> tokio_tungstenite::tungstenite::Message {
    let json = serde_json::to_string(&response).unwrap_or_else(|_| {
        "{\"type\":\"response\",\"id\":0,\"status\":\"error\",\"code\":500,\"message\":\"Failed to serialize response\"}".to_string()
    });

    tokio_tungstenite::tungstenite::Message::Text(json)
}