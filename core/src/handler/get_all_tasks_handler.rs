use async_trait::async_trait;
use log::{error, info};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::{orchestrator::AgentOrchestrator, requests::task::get_all_tasks},
};
use shared::server::message::{Message, Status};
use sqlx::postgres::PgPool;

pub struct GetAllTasksHandler {
    pub orchestrator: Arc<AgentOrchestrator>,
    pub pool: Arc<PgPool>,
}

impl GetAllTasksHandler {
    pub fn new(orchestrator: Arc<AgentOrchestrator>, pool: Arc<PgPool>) -> Self {
        Self { orchestrator, pool }
    }
}

#[async_trait]
impl HandlerTrait for GetAllTasksHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Received request for extracting all tasks");

        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let agent_id = match data.get("agent_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing agent_id"),
        };

        let manager = match self.orchestrator.get(agent_id).await {
            Ok(m) => m,
            Err(_) => return Message::new_response(
                Status::Error, None, 503,
                format!("Agent {} not connected", agent_id),
            ),
        };

        match get_all_tasks(&manager).await {
            Ok(tasks) => {
                for task in &tasks {
                    let insert_result = sqlx::query!(
                r#"
                INSERT INTO tasks (id, agent_id)
                VALUES ($1, $2)
                ON CONFLICT (id) DO NOTHING
                "#,
                task.id,
                agent_id as i32
            )
                        .execute(self.pool.as_ref())
                        .await;

                    match insert_result {
                        Ok(result) => {
                            if result.rows_affected() > 0 {
                                info!(
                            "Inserted missing task {} for agent {}",
                            task.id,
                            agent_id
                        );
                            }
                        }
                        Err(e) => {
                            error!("Failed to sync task {}: {}", task.id, e);
                        }
                    }
                }

                Message::new_response(
                    Status::Ok,
                    Some(json!({ "tasks": tasks })),
                    200,
                    "Successfully retrieved all tasks",
                )
            }
            Err(e) => {
                error!("Failed to retrieve tasks: {}", e);
                Message::new_response(
                    Status::Error,
                    None,
                    500,
                    "Failed to retrieve tasks",
                )
            }
        }
    }
}