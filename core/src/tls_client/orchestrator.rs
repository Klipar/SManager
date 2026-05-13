use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::{bail, Result};
use log::{error, info, warn};
use sqlx::postgres::PgPool;
use tokio::sync::broadcast;
use tokio::time::{interval, Duration};
use crate::tls_client::connection_manager::ConnectionManager;
use crate::state::RunEvent;

pub struct AgentOrchestrator {
    connections: Arc<Mutex<HashMap<i64, Arc<ConnectionManager>>>>,
    run_tx: broadcast::Sender<RunEvent>,
}

impl AgentOrchestrator {
    pub fn new(run_tx: broadcast::Sender<RunEvent>) -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            run_tx,
        }
    }

    pub fn start_reconnect_loop(self: &Arc<Self>, pool: Arc<PgPool>) {
        let weak_self = Arc::downgrade(self);
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                let orch = match weak_self.upgrade() {
                    Some(orch) => orch,
                    None => break,
                };
                reconnect_disconnected_agents(orch, &pool).await;
            }
        });
    }

    pub async fn connect(
        self: &Arc<Self>,
        agent_id: i64,
        ip: &str,
        port: u16,
    ) -> Result<()> {
        let manager = ConnectionManager::connect(agent_id, ip, port, self.run_tx.clone()).await?;
        let manager = Arc::new(manager);

        let mut conns = self.connections.lock().await;
        if conns.contains_key(&agent_id) {
            warn!("[Orchestrator] Agent {} already connected", agent_id);
            return Ok(());
        }
        conns.insert(agent_id, Arc::clone(&manager));
        drop(conns);

        spawn_disconnect_watcher(self, agent_id, Arc::clone(&manager));
        info!("[Orchestrator] Connected to agent {}", agent_id);
        Ok(())
    }

    pub async fn disconnect(&self, agent_id: i64) {
        let mut conns = self.connections.lock().await;
        if conns.remove(&agent_id).is_some() {
            info!("[Orchestrator] Disconnected agent {}", agent_id);
        } else {
            warn!("[Orchestrator] Agent {} not found", agent_id);
        }
    }

    pub async fn get(&self, agent_id: i64) -> Result<Arc<ConnectionManager>> {
        let conns = self.connections.lock().await;
        match conns.get(&agent_id) {
            Some(manager) => Ok(Arc::clone(manager)),
            None => bail!("[Orchestrator] Agent {} not connected", agent_id),
        }
    }

    pub async fn connected_agents(&self) -> Vec<i64> {
        self.connections.lock().await.keys().cloned().collect()
    }

    pub async fn is_connected(&self, agent_id: i64) -> bool {
        self.connections.lock().await.contains_key(&agent_id)
    }

    pub async fn connect_all(self: &Arc<Self>, pool: &Arc<PgPool>) -> Vec<(i64, anyhow::Error)> {
        let agents = match sqlx::query!("SELECT id, ip, port FROM agents")
            .fetch_all(pool.as_ref())
            .await
        {
            Ok(a) => a,
            Err(e) => {
                error!("[Orchestrator] Failed to fetch agents: {}", e);
                return vec![];
            }
        };

        let weak_self = Arc::downgrade(self);
        let run_tx = self.run_tx.clone();
        let handles: Vec<_> = agents.into_iter().map(|agent| {
            let run_tx = run_tx.clone();
            let connections = Arc::clone(&self.connections);
            let agent_id = agent.id as i64;
            let weak_self = weak_self.clone();
            tokio::spawn(async move {
                let result = ConnectionManager::connect(
                    agent_id,
                    &agent.ip,
                    agent.port as u16,
                    run_tx,
                ).await;

                match result {
                    Ok(manager) => {
                        let manager = Arc::new(manager);
                        let mut conns = connections.lock().await;
                        if conns.contains_key(&agent_id) {
                            return None;
                        }
                        conns.insert(agent_id, Arc::clone(&manager));
                        drop(conns);
                        if let Some(orch) = weak_self.upgrade() {
                            spawn_disconnect_watcher(&orch, agent_id, manager);
                        }
                        info!("[Orchestrator] Connected to agent {}", agent_id);
                        None
                    }
                    Err(e) => {
                        error!("[Orchestrator] Failed to connect to agent {}: {}", agent_id, e);
                        Some((agent_id, e))
                    }
                }
            })
        }).collect();

        let mut errors = vec![];
        for handle in handles {
            if let Ok(Some(err)) = handle.await {
                errors.push(err);
            }
        }
        errors
    }
}

async fn reconnect_disconnected_agents(orch: Arc<AgentOrchestrator>, pool: &PgPool) {
    let agents = match sqlx::query!("SELECT id, ip, port FROM agents")
        .fetch_all(pool)
        .await
    {
        Ok(a) => a,
        Err(e) => {
            error!("[Reconnect loop] DB error: {}", e);
            return;
        }
    };

    for agent in agents {
        let agent_id = agent.id as i64;
        if orch.is_connected(agent_id).await {
            continue;
        }
        match ConnectionManager::connect(agent_id, &agent.ip, agent.port as u16, orch.run_tx.clone()).await {
            Ok(manager) => {
                let manager = Arc::new(manager);
                let mut conns = orch.connections.lock().await;
                if conns.contains_key(&agent_id) {
                    continue;
                }
                conns.insert(agent_id, Arc::clone(&manager));
                drop(conns);
                spawn_disconnect_watcher(&orch, agent_id, manager);
                info!("[Orchestrator] Reconnected to agent {}", agent_id);
            }
            Err(e) => {
                warn!("[Reconnect loop] Agent {} connection failed: {}", agent_id, e);
            }
        }
    }
}

fn spawn_disconnect_watcher(orch: &Arc<AgentOrchestrator>, agent_id: i64, manager: Arc<ConnectionManager>) {
    let weak_self = Arc::downgrade(orch);
    tokio::spawn(async move {
        manager.wait_for_disconnect().await;
        if let Some(orch) = weak_self.upgrade() {
            orch.disconnect(agent_id).await;
            info!("[Orchestrator] Agent {} disconnected (detected)", agent_id);
        }
    });
}