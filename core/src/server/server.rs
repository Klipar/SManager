use std::{collections::HashMap, sync::Arc};
use std::sync::atomic::{AtomicU64, Ordering};
use log::info;
use tokio::net::TcpListener;

use crate::handler::handler_trait::HandlerTrait;
use crate::server::connection::handle_connection;
use crate::server::connection_registry::ConnectionRegistry;

static CONN_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub struct Server {
    pub ip: String,
    pub port: u16,
    pub is_active: bool,
    pub handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    pub registry: Arc<ConnectionRegistry>,
}

impl Server {
    pub fn new(ip: String, port: u16) -> Self {
        Self {
            ip,
            port,
            is_active: false,
            handlers: Arc::new(HashMap::new()),
            registry: ConnectionRegistry::new(),
        }
    }

    pub fn add_handler(&mut self, name: &str, handler: Arc<dyn HandlerTrait>) {
        Arc::make_mut(&mut self.handlers).insert(name.to_string(), handler);
    }

    pub fn registry(&self) -> Arc<ConnectionRegistry> {
        Arc::clone(&self.registry)
    }

    pub async fn start_server(&mut self) {
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port))
            .await
            .expect("Failed to bind server");

        info!("WebSocket server listening on ws://{}:{}", self.ip, self.port);
        self.is_active = true;

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let conn_id = CONN_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
                    tokio::spawn(handle_connection(
                        stream,
                        addr,
                        Arc::clone(&self.handlers),
                        Arc::clone(&self.registry),
                        conn_id,
                    ));
                }
                Err(e) => {
                    log::error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}