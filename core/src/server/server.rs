use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};

pub struct Server {
    pub ip: String,
    pub port: u16,
    pub is_active: bool,
}

impl Server {
    pub fn new(ip: String, port: u16) -> Self {
        Self {
            ip,
            port,
            is_active: false,
        }
    }

    pub async fn start_server(&mut self) {
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port)).await.unwrap();
        println!("Server running on ws://{}:{}", self.ip, self.port);

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(Self::handle_connection(stream));
        }
    }

    async fn handle_connection(stream: tokio::net::TcpStream) {
        let mut ws_stream = accept_async(stream)
            .await
            .expect("Error during WebSocket handshake");

        println!("New WebSocket connection");

        while let Some(msg) = ws_stream.next().await {
            let msg = msg.unwrap();

            if msg.is_text() || msg.is_binary() {
                println!("Received: {:?}", msg);

                // Echo the message back to the client
                ws_stream.send(msg).await.unwrap();
            }
        }
    }
}