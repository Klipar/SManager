use std::{ collections::HashMap, sync::Arc, time::{Duration, Instant}};
use tokio::{sync::{mpsc, oneshot, Mutex}, time};
use shared::server::message::Message;
use log::{error, info, warn};
use anyhow::Result;

use crate::tls_client::client::AgentClient;
use crate::tls_client::connection::connect;

const PING_INTERVAL: Duration = Duration::from_secs(20);

/// A pending request waiting for its response.
struct PendingRequest {
    tx: oneshot::Sender<Message>,
}

struct Inner {
    pending: HashMap<u64, PendingRequest>,
    next_id: u64,
    /// IDs that have been used and released - reused before incrementing next_id.
    free_ids: Vec<u64>,
    last_activity: Instant,
}

impl Inner {
    fn new() -> Self {
        Self {
            pending: HashMap::new(),
            next_id: 1,
            free_ids: Vec::new(),
            last_activity: Instant::now(),
        }
    }

    fn alloc_id(&mut self) -> u64 {
        if let Some(id) = self.free_ids.pop() {
            id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            id
        }
    }

    fn free_id(&mut self, id: u64) {
        self.free_ids.push(id);
    }
}

pub struct ConnectionManager {
    client: AgentClient,
    inner: Arc<Mutex<Inner>>,
}

impl ConnectionManager {
    /// Connect to agent and spawn background tasks (reader + keepalive).
    pub async fn connect(
        agent_ip: &str,
        agent_port: u16,
        agent_cn: &str,
    ) -> Result<Self> {
        let framed = connect(agent_ip, agent_port, agent_cn).await?;
        let (client, inbound_rx) = AgentClient::new(framed);

        let inner = Arc::new(Mutex::new(Inner::new()));
        {
            let inner_clone = Arc::clone(&inner);
            tokio::spawn(Self::reader_task(inbound_rx, inner_clone));
        }
        {
            let client_clone = client.clone();
            let inner_clone = Arc::clone(&inner);
            tokio::spawn(Self::keepalive_task(client_clone, inner_clone));
        }

        Ok(Self { client, inner })
    }

    /// Send a request and await its correlated response.
    pub async fn request(
        &self,
        action: impl Into<String>,
        data: Option<serde_json::Value>,
    ) -> Result<Message> {
        let (id, rx) = {
            let mut state = self.inner.lock().await;
            let id = state.alloc_id();
            let (tx, rx) = oneshot::channel();
            state.pending.insert(id, PendingRequest { tx });
            state.last_activity = Instant::now();
            (id, rx)
        };

        let msg = Message::Request {
            id,
            action: action.into(),
            data,
        };

        self.client.send(msg).await?;
        match rx.await {
            Ok(response) => {
                let mut state = self.inner.lock().await;
                state.free_id(id);
                Ok(response)
            }
            Err(_) => {
                let mut state = self.inner.lock().await;
                state.pending.remove(&id);
                state.free_id(id);
                Err(anyhow::anyhow!("Agent connection closed before response for id={}", id))
            }
        }
    }

    async fn reader_task(
        mut inbound_rx: mpsc::Receiver<Message>,
        inner: Arc<Mutex<Inner>>,
    ) {
        while let Some(msg) = inbound_rx.recv().await {
            match &msg {
                Message::Response { id, .. } => {
                    let id = *id;
                    let mut state = inner.lock().await;
                    state.last_activity = Instant::now();
                    if let Some(pending) = state.pending.remove(&id) {
                        // Don't free the id here - the caller's request() does it
                        // after the oneshot resolves, keeping the slot live until then.
                        let _ = pending.tx.send(msg);
                    } else {
                        warn!("[ConnectionManager] Unsolicited response for id={}", id);
                    }
                }
                Message::Request { .. } => {
                    error!("[ConnectionManager] Unexpected Request from agent - ignoring");
                }
            }
        }
        info!("[ConnectionManager] Inbound channel closed - agent disconnected");
    }

    async fn keepalive_task(client: AgentClient, inner: Arc<Mutex<Inner>>) {
        let mut ticker = time::interval(Duration::from_secs(1));
        loop {
            ticker.tick().await;
            let elapsed = { inner.lock().await.last_activity.elapsed() };

            if elapsed >= PING_INTERVAL {
                let (id, rx) = {
                    let mut state = inner.lock().await;
                    let id = state.alloc_id();
                    let (tx, rx) = oneshot::channel();
                    state.pending.insert(id, PendingRequest { tx });
                    state.last_activity = Instant::now();
                    (id, rx)
                };

                let ping = Message::Request {
                    id,
                    action: "ping".to_string(),
                    data: Some(serde_json::json!({})),
                };

                if let Err(e) = client.send(ping).await {
                    error!("[ConnectionManager] Failed to send ping: {}", e);
                    let mut state = inner.lock().await;
                    state.pending.remove(&id);
                    state.free_id(id);
                    break;
                }

                info!("[ConnectionManager] Keepalive ping sent (id={})", id);

                match time::timeout(Duration::from_secs(5), rx).await {
                    Ok(Ok(_)) => {
                        let mut state = inner.lock().await;
                        state.free_id(id);
                        info!("[ConnectionManager] Pong received (id={})", id);
                    }
                    _ => {
                        error!("[ConnectionManager] Ping timeout or connection lost (id={})", id);
                        let mut state = inner.lock().await;
                        state.pending.remove(&id);
                        state.free_id(id);
                        break;
                    }
                }
            }
        }
    }
}