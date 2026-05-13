use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::{bail, Result};
use log::{error, info, warn};
use sqlx::postgres::PgPool;
use tokio::sync::broadcast;
use futures::future::join_all;
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
        &self,
        agent_id: i64,
        ip: &str,
        port: u16,
    ) -> Result<()> {
        let manager = Arc::new(ConnectionManager::connect(agent_id, ip, port,  self.run_tx.clone()).await?);
        let previous = {
            let mut conns = self.connections.lock().await;
            conns.insert(agent_id, manager)
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
    pub async fn connect_all(&self) -> Vec<(i64, anyhow::Error)> {
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