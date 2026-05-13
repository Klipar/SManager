use async_trait::async_trait;
use log::error;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::{orchestrator::AgentOrchestrator, requests::run::delete_inactive_runs},
};
use shared::server::message::{Message, Status};

pub struct DeleteInactiveRunsHandler {
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl DeleteInactiveRunsHandler {
    pub fn new(orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl HandlerTrait for DeleteInactiveRunsHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let agent_id = match data.get("agent_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing agent_id"),
        };

        let task_id = match data.get("task_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing task_id"),
        };

        let manager = match self.orchestrator.get(agent_id).await {
            Ok(m) => m,
            Err(_) => return Message::new_response(Status::Error, None, 503, format!("Agent {} not connected", agent_id)),
        };

        match delete_inactive_runs(&manager, task_id).await {
            Ok(count) => Message::new_response(
                Status::Ok,
                Some(json!({ "deleted_count": count })),
                200,
                "Inactive runs deleted",
            ),
            Err(e) => {
                error!("Failed to delete inactive runs: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to delete inactive runs")
            }
        }
    }
}