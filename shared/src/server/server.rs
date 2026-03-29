use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};
use futures::StreamExt;
use serde_json::Value;
use anyhow::Result;
use futures::SinkExt;

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
            let (socket, addr) = listener.accept().await?;
            println!("New connection from {}", addr);

            let handlers = self.handlers.clone();

            tokio::spawn(async move {
                let mut framed = Framed::new(socket, LinesCodec::new_with_max_length(65536)); //Set to 64 kb data per json. if need can be extended

                while let Some(result) = framed.next().await {
                    match result {
                        Ok(line) => {
                            if let Ok(json) = serde_json::from_str::<Value>(&line) {
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

                            if let Err(e) = framed.send(line).await {
                                eprintln!("Failed to write to {}: {}", addr, e);
                                return;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading from {}: {}", addr, e);
                            return;
                        }
                    }
                }
                println!("Unconnected {}", addr);
            });
        }
    }
}