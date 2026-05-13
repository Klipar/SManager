use std::{collections::HashMap, sync::Arc, time::{Duration, Instant}};
use tokio::{sync::{mpsc, oneshot, Mutex, watch}, time};
use shared::server::message::{Message, Status};
use log::{debug, error, info, warn};
use anyhow::Result;
use tokio::sync::broadcast;
use crate::state::RunEvent;
use shared::db::models::run::Run;
use crate::tls_client::client::AgentClient;
use crate::tls_client::connection::connect;

const PING_INTERVAL: Duration = Duration::from_secs(20);

struct PendingRequest {
    tx: oneshot::Sender<Message>,
}

struct Inner {
    pending: HashMap<u64, PendingRequest>,
    next_id: u64,
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
    stream_tx: Arc<Mutex<Option<mpsc::Sender<Message>>>>,
    disconnect_rx: watch::Receiver<bool>,
    _disconnect_tx: Arc<watch::Sender<bool>>,
}

impl ConnectionManager {
    pub async fn connect(
        agent_id: i64,
        agent_ip: &str,
        agent_port: u16,
        run_tx: broadcast::Sender<RunEvent>,
    ) -> Result<Self> {
        let framed = connect(agent_ip, agent_port).await?;
        let (client, inbound_rx) = AgentClient::new(framed);
        let inner = Arc::new(Mutex::new(Inner::new()));
        let stream_tx: Arc<Mutex<Option<mpsc::Sender<Message>>>> = Arc::new(Mutex::new(None));

        let (disconnect_tx, disconnect_rx) = watch::channel(false);
        let disconnect_tx = Arc::new(disconnect_tx);

        {
            let inner_clone = Arc::clone(&inner);
            let stream_tx_clone = Arc::clone(&stream_tx);
            let run_tx_clone = run_tx.clone();
            let agent_id_clone = agent_id;
            let disconnect_tx = Arc::clone(&disconnect_tx);
            tokio::spawn(Self::reader_task(
                inbound_rx,
                inner_clone,
                stream_tx_clone,
                client.clone(),
                run_tx_clone,
                agent_id_clone,
                disconnect_tx,
            ));
        }

        {
            let client_clone = client.clone();
            let inner_clone = Arc::clone(&inner);
            let disconnect_tx = Arc::clone(&disconnect_tx);
            let disconnect_rx_clone = disconnect_rx.clone();
            tokio::spawn(Self::keepalive_task(client_clone, inner_clone, disconnect_tx, disconnect_rx_clone));
        }

        Ok(Self {
            client,
            inner,
            stream_tx,
            disconnect_rx,
            _disconnect_tx: disconnect_tx,
        })
    }

    pub async fn wait_for_disconnect(&self) {
        let mut rx = self.disconnect_rx.clone();
        loop {
            if rx.changed().await.is_err() {
                return;
            }
            if *rx.borrow() {
                return;
            }
        }
    }

    pub async fn subscribe_push(&self) -> mpsc::Receiver<Message> {
        let (tx, rx) = mpsc::channel(64);
        *self.stream_tx.lock().await = Some(tx);
        rx
    }

    async fn reader_task(
        mut inbound_rx: mpsc::Receiver<Message>,
        inner: Arc<Mutex<Inner>>,
        stream_tx: Arc<Mutex<Option<mpsc::Sender<Message>>>>,
        client: AgentClient,
        run_tx: broadcast::Sender<RunEvent>,
        agent_id: i64,
        disconnect_tx: Arc<watch::Sender<bool>>,
    ) {
        while let Some(msg) = inbound_rx.recv().await {
            match &msg {
                Message::Request { action, id, data } if action == "execution_stream" => {
                    let msg_id = *id;
                    if let Some(data) = data {
                        match serde_json::from_value::<Vec<Run>>(data.clone()) {
                            Ok(runs) => {
                                for run in runs {
                                    let _ = run_tx.send(RunEvent { agent_id, run });
                                }
                            }
                            Err(e) => {
                                log::error!("[ConnectionManager] Failed to parse run data: {}", e);
                            }
                        }
                    }
                    let guard = stream_tx.lock().await;
                    if let Some(tx) = guard.as_ref() {
                        if tx.send(msg).await.is_err() {
                            warn!("[ConnectionManager] Stream receiver dropped");
                        }
                    } else {
                        warn!("[ConnectionManager] Received execution_stream but no subscriber");
                    }
                    drop(guard);
                    let ack = Message::Response {
                        id: msg_id,
                        status: Status::Ok,
                        code: 200,
                        message: "Received execution_stream".to_string(),
                        data: None,
                    };
                    if let Err(e) = client.send(ack).await {
                        error!("[ConnectionManager] Failed to send ACK: {}", e);
                    }
                }
                Message::Response { id, .. } => {
                    let id = *id;
                    let mut state = inner.lock().await;
                    state.last_activity = Instant::now();
                    if let Some(pending) = state.pending.remove(&id) {
                        let _ = pending.tx.send(msg);
                    } else {
                        debug!("[ConnectionManager] Ping/unsolicited response for id={}", id);
                    }
                }
                Message::Request { action, .. } => {
                    warn!("[ConnectionManager] Unknown push action: {}", action);
                }
            }
        }
        info!("[ConnectionManager] Inbound channel closed");
        let _ = disconnect_tx.send(true);
    }

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

    async fn keepalive_task(
        client: AgentClient,
        inner: Arc<Mutex<Inner>>,
        disconnect_tx: Arc<watch::Sender<bool>>,
        mut disconnect_rx: watch::Receiver<bool>,
    ) {
        let mut ticker = time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                _ = ticker.tick() => {
                }
                _ = disconnect_rx.changed() => {
                    info!("[ConnectionManager] Keepalive received disconnect signal");
                    break;
                }
            }
            if *disconnect_rx.borrow() {
                break;
            }
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
                    let _ = disconnect_tx.send(true);
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
                        let _ = disconnect_tx.send(true);
                        break;
                    }
                }
            }
        }
    }
}