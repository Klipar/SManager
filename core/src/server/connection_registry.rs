use std::collections::HashMap;
use std::sync::Arc;
use shared::server::message::Message;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::tungstenite::Message as WsMessage;

pub type ConnTx = mpsc::Sender<WsMessage>;

pub struct ConnectionRegistry {
    connections: RwLock<HashMap<u64, ConnTx>>,
}

impl ConnectionRegistry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            connections: RwLock::new(HashMap::new()),
        })
    }

    pub async fn register(&self, id: u64, tx: ConnTx) {
        self.connections.write().await.insert(id, tx);
    }

    pub async fn unregister(&self, id: u64) {
        self.connections.write().await.remove(&id);
    }

pub async fn broadcast(&self, msg: &Message) {
    let json = serde_json::to_string(msg).unwrap_or_default();
    let ws_msg = WsMessage::Text(json);
    let conns = self.connections.read().await;
    for (_, tx) in conns.iter() {
        let _ = tx.try_send(ws_msg.clone());
    }
}

pub async fn broadcast_except(&self, exclude_id: u64, msg: &Message) {
    let json = serde_json::to_string(msg).unwrap_or_default();
    let ws_msg = WsMessage::Text(json);
    let conns = self.connections.read().await;
    for (id, tx) in conns.iter() {
        if *id != exclude_id {
            let _ = tx.try_send(ws_msg.clone());
        }
    }
}

pub async fn send_to(&self, id: u64, msg: &Message) -> bool {
    let json = serde_json::to_string(msg).unwrap_or_default();
    let ws_msg = WsMessage::Text(json);
    let conns = self.connections.read().await;
    if let Some(tx) = conns.get(&id) {
        tx.try_send(ws_msg).is_ok()
    } else {
        false
    }
}
}