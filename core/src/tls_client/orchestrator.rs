use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::{bail, Result};
use log::{error, info, warn};
use sqlx::postgres::PgPool;
use tokio::sync::broadcast;
use futures::future::join_all;
use tokio::time::{interval, Duration};
use crate::tls_client::connection_manager::ConnectionManager;
use crate::state::RunEvent;

#[derive(Clone)]
pub struct AgentOrchestrator {
    connections: Arc<Mutex<HashMap<i64, Arc<ConnectionManager>>>>,
    run_tx: broadcast::Sender<RunEvent>,
    pool: Arc<PgPool>,
}

impl AgentOrchestrator {
    pub fn new(run_tx: broadcast::Sender<RunEvent>, pool: Arc<PgPool>) -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            run_tx,
            pool,
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

    async fn mark_online(&self, agent_id: i64) -> Result<()> {
        sqlx::query(
            "UPDATE agents SET status = 'online', last_connection = NOW() WHERE id = $1",
        )
        .bind(agent_id as i32)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    async fn mark_offline(&self, agent_id: i64) -> Result<()> {
        sqlx::query("UPDATE agents SET status = 'offline' WHERE id = $1")
            .bind(agent_id as i32)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn connect(
        self: &Arc<Self>,
        agent_id: i64,
        ip: &str,
        port: u16,
    ) -> Result<()> {
        let manager = Arc::new(ConnectionManager::connect(agent_id, ip, port, self.run_tx.clone()).await?);
        let previous = {
            let mut conns = self.connections.lock().await;
            conns.insert(agent_id, Arc::clone(&manager))
        };

        if let Err(e) = self.mark_online(agent_id).await {
            let mut conns = self.connections.lock().await;
            if let Some(prev) = previous {
                conns.insert(agent_id, prev);
            } else {
                conns.remove(&agent_id);
            }
            return Err(e);
        }

        spawn_disconnect_watcher(self, agent_id, Arc::clone(&manager));
        info!("[Orchestrator] Connected to agent {}", agent_id);
        Ok(())
    }

    pub async fn disconnect(&self, agent_id: i64) -> Result<()> {
        let removed = {
            let mut conns = self.connections.lock().await;
            conns.remove(&agent_id).is_some()
        };

        if removed {
            info!("[Orchestrator] Disconnected agent {}", agent_id);
        } else {
            warn!("[Orchestrator] Agent {} not found", agent_id);
        }

        self.mark_offline(agent_id).await?;
        Ok(())
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

    pub async fn connect_all(self: &Arc<Self>) -> Vec<(i64, anyhow::Error)> {
        let agents = match sqlx::query!("SELECT id, ip, port FROM agents")
            .fetch_all(self.pool.as_ref())
            .await
        {
            Ok(a) => a,
            Err(e) => {
                error!("[Orchestrator] Failed to fetch agents: {}", e);
                return vec![];
            }
        };

        let futures = agents.into_iter().map(|agent| {
            let orchestrator = self.clone();
            async move {
                orchestrator
                    .connect(agent.id as i64, &agent.ip, agent.port as u16)
                    .await
                    .map_err(|e| (agent.id as i64, e))
            }
        });

        join_all(futures)
            .await
            .into_iter()
            .filter_map(|result| result.err())
            .collect()
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
        match orch.connect(agent_id, &agent.ip, agent.port as u16).await {
            Ok(_) => {
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
            orch.disconnect(agent_id).await.ok();
            info!("[Orchestrator] Agent {} disconnected (detected)", agent_id);
        }
    });
}