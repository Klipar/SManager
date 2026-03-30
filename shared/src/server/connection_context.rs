use tokio_util::codec::Framed;
use tokio_util::codec::LinesCodec;
use tokio::net::TcpStream;
use futures::SinkExt;
use anyhow::Result;

pub struct ConnectionContext {
    pub authenticated: bool,
    pub sender_id: Option<String>,
    pub framed: Framed<TcpStream, LinesCodec>,
}

impl ConnectionContext {
    pub fn new(framed: Framed<TcpStream, LinesCodec>) -> Self {
        Self {
            authenticated: false,
            sender_id: None,
            framed,
        }
    }

    pub async fn send_response(&mut self, msg: &str) -> Result<()> {
        self.framed.send(msg.to_string()).await?;
        Ok(())
    }
}