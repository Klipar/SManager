use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};
use futures::StreamExt;
use anyhow::Result;
use futures::sink::SinkExt;
use serde_json::json;
use crate::server::{connection_context::ConnectionContext, handler_trait::HandlerTrait, message::{Message, Status}};
use log::{info, error};

pub struct Server {
    pub ip: String,
    pub port: u16,
    pub is_active: bool,
    pub handlers: HashMap<String, Arc<dyn HandlerTrait>>,
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

    pub async fn start_server(mut self) -> Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port)).await?;
        self.is_active = true;

        info!("Server listening on {}:{}", self.ip, self.port);

        loop {
            let (socket, addr) = listener.accept().await?;
            info!("New connection from {}", addr);

            let handlers = self.handlers.clone();

            tokio::spawn(async move {
                let mut framed =  Framed::new(socket, LinesCodec::new_with_max_length(65536)); //Set to 64 kb data per json. if need can be extended

                let mut ctx = ConnectionContext::new();

                while let Some(result) = framed.next().await {
                    match result {
                        Ok(line) => {
                            if let Ok(msg) = serde_json::from_str::<Message>(&line) {
                                match msg {
                                    Message::Request {  id, action, data } => {
                                        if ctx.authenticated || action == "authenticate" {
                                            if let Some(handler) = handlers.get(&action) {
                                                let mut response = handler.handle(data, &mut ctx).await;
                                                response.set_id(id);

                                                let json = serde_json::to_string(&response).unwrap();

                                                framed.send(json).await.unwrap();
                                            } else {
                                                error!("Unknown request: {}", action);
                                            }
                                        } else {
                                            let response = Message::Response {
                                                id,
                                                status: Status::Error,
                                                data: json!({ "message": "Unauthorized" }),
                                                code: 401,
                                            };

                                            let json = serde_json::to_string(&response).unwrap();
                                            if let Err(e) = framed.send(json).await {
                                                error!("Failed to write to {}: {}", addr, e);
                                                return;
                                            }
                                        }
                                    },
                                    Message::Response { .. } => {
                                        println!("{:?}", msg); //TODO: process responses
                                    }
                                }
                            } else {
                                error!("Failed to parse from {}", addr);
                            }
                        }
                        Err(e) => {
                            error!("Error reading from {}: {}", addr, e);
                            return;
                        }
                    }
                }
                info!("Unconnected {}", addr);
            });
        }
    }
}