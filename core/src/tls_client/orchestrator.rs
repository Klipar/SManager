use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::{bail, Result};
use log::{error, info, warn};
use sqlx::postgres::PgPool;
use crate::tls_client::connection_manager::ConnectionManager;

pub struct AgentOrchestrator {
    connections: Arc<Mutex<HashMap<i64, Arc<ConnectionManager>>>>,
}

impl AgentOrchestrator {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn connect(
        &self,
        agent_id: i64,
        ip: &str,
        port: u16,
        cn: &str,
    ) -> Result<()> {
        let manager = Arc::new(ConnectionManager::connect(ip, port, cn).await?);
        let mut conns = self.connections.lock().await;
        conns.insert(agent_id, manager);
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
    pub async fn connect_all(&self, pool: &Arc<PgPool>) -> Vec<(i64, anyhow::Error)> {
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
        let handles: Vec<_> = agents.into_iter().map(|agent| {
            let self_arc = Arc::clone(&self.connections);
            tokio::spawn(async move {
                let result = ConnectionManager::connect(
                    &agent.ip,
                    agent.port as u16,
                    &std::env::var("AGENT_SERVER_CN").unwrap_or("localhost".to_string()),
                ).await;

                match result {
                    Ok(manager) => {
                        self_arc.lock().await.insert(agent.id as i64, Arc::new(manager));
                        info!("[Orchestrator] Connected to agent {}", agent.id);
                        None
                    }
                    Err(e) => {
                        error!("[Orchestrator] Failed to connect to agent {}: {}", agent.id, e);
                        Some((agent.id as i64, e))
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


