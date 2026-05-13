use async_trait::async_trait;
use log::{error, info};
use serde_json::Value;
use std::sync::Arc;
use sqlx::postgres::PgPool;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::orchestrator::AgentOrchestrator,
};
use shared::server::message::{Message, Status};

pub struct ConnectAgentHandler {
    pub pool: Arc<PgPool>,
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl ConnectAgentHandler {
    pub fn new(pool: Arc<PgPool>, orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { pool, orchestrator }
    }
}

#[async_trait]
impl HandlerTrait for ConnectAgentHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let agent_id = match data.get("agent_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing agent_id"),
        };

        let agent = sqlx::query!("SELECT ip, port FROM agents WHERE id = $1", agent_id as i32)
            .fetch_optional(&*self.pool)
            .await;

        let agent = match agent {
            Ok(Some(a)) => a,
            Ok(None) => return Message::new_response(Status::Error, None, 404, "Agent not found"),
            Err(e) => {
                error!("DB error: {}", e);
                return Message::new_response(Status::Error, None, 500, "Database error");
            }
        };

        match self.orchestrator.connect(
            agent_id,
            &agent.ip,
            agent.port as u16,
        ).await {
            Ok(_) => {
                info!("Connected to agent {}", agent_id);
                Message::new_response(Status::Ok, None, 200, "Connected to agent")
            }
            Err(e) => {
                error!("Failed to connect to agent {}: {}", agent_id, e);
                Message::new_response(Status::Error, None, 503, "Failed to connect to agent")
            }
        }
    }
}