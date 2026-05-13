use async_trait::async_trait;
use log::error;
use serde_json::Value;
use std::sync::Arc;

use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    state::{AppState, RunEvent},
    tls_client::{
        orchestrator::AgentOrchestrator,
        requests::task::{run_task, start_stream},
    },
};

use shared::{
    enums::script_types::ScriptType,
    server::{
        dto::run_task_dto::RunTaskDTO,
        message::{Message, Status},
    },
};

pub struct RunTaskHandler {
    pub orchestrator: Arc<AgentOrchestrator>,
    pub state: Arc<AppState>,
}

impl RunTaskHandler {
    pub fn new(
        orchestrator: Arc<AgentOrchestrator>,
        state: Arc<AppState>,
    ) -> Self {
        Self { orchestrator, state }
    }
}

#[async_trait]
impl HandlerTrait for RunTaskHandler {
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

        let script_type = match data.get("script_type") {
            Some(value) => match serde_json::from_value::<ScriptType>(value.clone()) {
                Ok(script_type) => script_type,
                Err(_) => {
                    return Message::new_response(
                        Status::Error,
                        None,
                        400,
                        "Invalid script_type",
                    )
                }
            },
            None => {
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Missing script_type",
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

        match run_task(
            &manager,
            RunTaskDTO {
                task_id,
                script_type,
            },
        )
            .await
        {
            Ok(_) => {
                let manager_clone = Arc::clone(&manager);
                let run_tx = self.state.run_tx.clone();

                tokio::spawn(async move {
                    match start_stream(&manager_clone).await {
                        Ok(mut rx) => {
                            while let Some(run) = rx.recv().await {
                                let _ = run_tx.send(RunEvent {
                                    agent_id,
                                    run,
                                });
                            }
                        }
                        Err(e) => {
                            error!(
                                "Failed to start run stream for agent {}: {}",
                                agent_id,
                                e
                            );
                        }
                    }
                });

                Message::new_response(
                    Status::Ok,
                    None,
                    200,
                    "Task started",
                )
            }

            Err(e) => {
                error!("Failed to run task: {}", e);

                Message::new_response(
                    Status::Error,
                    None,
                    500,
                    "Failed to run task",
                )
            }
        }
    }
}