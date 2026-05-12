use async_trait::async_trait;
use log::error;
use serde_json::Value;
use std::sync::Arc;

use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    state::AppState,
    tls_client::{
        orchestrator::AgentOrchestrator,
        requests::task::{stop_task, stop_stream},
    },
};

use shared::server::message::{Message, Status};

pub struct StopTaskHandler {
    pub orchestrator: Arc<AgentOrchestrator>,
    pub state: Arc<AppState>,
}

impl StopTaskHandler {
    pub fn new(
        orchestrator: Arc<AgentOrchestrator>,
        state: Arc<AppState>,
    ) -> Self {
        Self { orchestrator, state }
    }
}

#[async_trait]
impl HandlerTrait for StopTaskHandler {
    async fn handle(
        &self,
        data: Option<Value>,
        _ctx: &mut ConnectionContext,
    ) -> Message {
        let data = match data {
            Some(v) => v,
            None => {
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Missing data",
                )
            }
        };

        let agent_id = match data.get("agent_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => {
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Missing agent_id",
                )
            }
        };

        let task_id = match data.get("task_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => {
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Missing task_id",
                )
            }
        };

        let manager = match self.orchestrator.get(agent_id).await {
            Ok(m) => m,
            Err(_) => {
                return Message::new_response(
                    Status::Error,
                    None,
                    503,
                    format!("Agent {} not connected", agent_id),
                )
            }
        };

        match stop_task(&manager, task_id).await {
            Ok(_) => {
                if let Err(e) = stop_stream(&manager).await {
                    error!(
                        "Failed to stop stream for agent {}: {}",
                        agent_id,
                        e
                    );
                }
                Message::new_response(
                    Status::Ok,
                    None,
                    200,
                    "Task stopped",
                )
            }

            Err(e) => {
                error!("Failed to stop task: {}", e);

                Message::new_response(
                    Status::Error,
                    None,
                    500,
                    "Failed to stop task",
                )
            }
        }
    }
}