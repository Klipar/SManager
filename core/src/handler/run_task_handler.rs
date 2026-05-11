use async_trait::async_trait;
use log::error;
use serde_json::Value;
use std::sync::Arc;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::{
        orchestrator::AgentOrchestrator,
        requests::task::run_task,
    },
};
use shared::server::{dto::run_task_dto::RunTaskDTO, message::{Message, Status}};
use shared::enums::script_types::ScriptType;
pub struct RunTaskHandler {
    orchestrator: Arc<AgentOrchestrator>,
}

impl RunTaskHandler {
    pub fn new(orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}
#[async_trait]
impl HandlerTrait for RunTaskHandler {
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
            Err(_) => return Message::new_response(
                Status::Error, None, 503,
                format!("Agent {} not connected", agent_id)
            ),
        };

        match run_task(&manager, RunTaskDTO {
            task_id,
            script_type: ScriptType::Run,
        }).await {
            Ok(_) => Message::new_response(Status::Ok, None, 200, "Task started"),
            Err(e) => {
                error!("Failed to run task: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to run task")
            }
        }
    }
}