use async_trait::async_trait;
use log::{error, info};
use serde_json::Value;
use std::sync::Arc;
use sqlx::postgres::PgPool;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::{orchestrator::AgentOrchestrator, requests::task::remove_task},
};
use shared::server::message::{Message, Status};

pub struct RemoveTaskHandler {
    pub pool: Arc<PgPool>,
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl RemoveTaskHandler {
    pub fn new(pool: Arc<PgPool>, orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { pool, orchestrator }
    }
}

#[async_trait]
impl HandlerTrait for RemoveTaskHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Received request removing task");

        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let agent_id = match data.get("agent_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing agent_id"),
        };

        let task_id = match data.get("id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing id"),
        };

        let manager = match self.orchestrator.get(agent_id).await {
            Ok(m) => m,
            Err(_) => return Message::new_response(
                Status::Error, None, 503,
                format!("Agent {} not connected", agent_id),
            ),
        };


        if let Err(e) = remove_task(&manager, task_id).await {
            error!("Failed to remove task on agent: {}", e);
            return Message::new_response(Status::Error, None, 500, "Failed to remove task on agent");
        }


        let result = sqlx::query!(
            "DELETE FROM tasks WHERE id = $1",
            task_id as i32
        )
            .execute(self.pool.as_ref())
            .await;

        match result {
            Ok(res) if res.rows_affected() > 0 => Message::new_response(
                Status::Ok, None, 200, "Task removed successfully",
            ),
            Ok(_) => Message::new_response(Status::Error, None, 404, "Task not found in core DB"),
            Err(e) => {
                error!("Failed to delete task from core DB: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to delete task from core DB")
            }
        }
    }
}