use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::Value;
use anyhow::Result;

use crate::server::handler_trait::HandlerTrait;


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

        println!("Server listening on {}:{}", self.ip, self.port);

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("New connection from {}", addr);

            let handlers = self.handlers.clone(); // clone Arc handles

            tokio::spawn(async move {
                let mut buf = vec![0; 1024];
                loop {
                    match socket.read(&mut buf).await {
                        Ok(0) => {
                            println!("Connection closed: {}", addr);
                            return;
                        }
                        Ok(n) => {
                            let received = &buf[..n];
                            // to json
                            if let Ok(json) = serde_json::from_slice::<Value>(received) {
                                if let Some(request) = json.get("request").and_then(|r| r.as_str()) {
                                    if let Some(handler) = handlers.get(request) {
                                        let data = json.get("data").cloned().unwrap_or(Value::Null);
                                        handler.handle(data).await;
                                    } else {
                                        eprintln!("Unknown request: {}", request);
                                    }
                                } else {
                                    eprintln!("Missing 'request' field");
                                }
                            } else {
                                eprintln!("Failed to parse JSON from {}", addr);
                            }

                            if let Err(e) = socket.write_all(received).await {
                                eprintln!("Failed to write to {}: {}", addr, e);
                                return;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from {}: {}", addr, e);
                            return;
                        }
                    }
                }
            });
        }
    }
}