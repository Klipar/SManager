use async_trait::async_trait;
use log::{error, info};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::{orchestrator::AgentOrchestrator, requests::task::update_task},
};
use shared::server::message::{Message, Status};
use shared::db::models::task::Task;

pub struct UpdateTaskHandler {
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl UpdateTaskHandler {
    pub fn new(orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl HandlerTrait for UpdateTaskHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Received request for updating task");

        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let agent_id = match data.get("agent_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing agent_id"),
        };

        let dto: Task = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse update-task request: {}", e);
                return Message::new_response(Status::Error, None, 400, "Invalid update-task request");
            }
        };

        let manager = match self.orchestrator.get(agent_id).await {
            Ok(m) => m,
            Err(_) => return Message::new_response(
                Status::Error, None, 503,
                format!("Agent {} not connected", agent_id),
            ),
        };

        match update_task(&manager, dto).await {
            Ok(task) => Message::new_response(
                Status::Ok,
                Some(json!({ "task": task })),
                200,
                "Task updated successfully",
            ),
            Err(e) => {
                error!("Failed to update task: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to update task")
            }
        }
    }
}