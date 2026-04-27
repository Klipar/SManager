use anyhow::Result;
use futures::{StreamExt, SinkExt};
use log::{error, info};
use tokio::sync::mpsc;
use std::sync::Arc;
use shared::server::message::Message;

use super::connection::AgentFramed;

// tx — pošli správu
// rx — prijatá správa

#[derive(Clone)]
pub struct AgentClient {
    tx: mpsc::Sender<Message>,
}

impl AgentClient {
    pub fn new(framed: AgentFramed) -> (Self, mpsc::Receiver<Message>) {
        let (mut writer, mut reader) = framed.split();

        // outbound: Core -> Agent
        let (outbound_tx, mut outbound_rx) = mpsc::channel::<Message>(32);

        // inbound: Agent -> Core
        let (inbound_tx, inbound_rx) = mpsc::channel::<Message>(32);

        // --- WRITER TASK ---
        tokio::spawn(async move {
            while let Some(msg) = outbound_rx.recv().await {
                match serde_json::to_string(&msg) {
                    Ok(json) => {
                        if let Err(e) = writer.send(json).await {
                            error!("[AgentClient] Write error: {}", e);
                            break;
                        }
                    }
                    Err(e) => error!("[AgentClient] Serialize error: {}", e),
                }
            }
        });

        // --- READER TASK ---
        tokio::spawn(async move {
            while let Some(result) = reader.next().await {
                match result {
                    Ok(line) => {
                        match serde_json::from_str::<Message>(&line) {
                            Ok(msg) => {
                                if inbound_tx.send(msg).await.is_err() {
                                    break;
                                }
                            }
                            Err(e) => error!("[AgentClient] Parse error: {}", e),
                        }
                    }
                    Err(e) => {
                        error!("[AgentClient] Read error: {}", e);
                        break;
                    }
                }
            }
            info!("[AgentClient] Connection closed");
        });

        (Self { tx: outbound_tx }, inbound_rx)
    }

    pub async fn send(&self, msg: Message) -> Result<()> {
        self.tx.send(msg).await?;
        Ok(())
    }
}