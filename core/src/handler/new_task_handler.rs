use async_trait::async_trait;
use log::{error, info};
use serde_json::{json, Value};
use std::sync::Arc;
use sqlx::postgres::PgPool;



use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::{
        orchestrator::AgentOrchestrator,
        requests::task::new_task,
    },
};
use shared::server::{
    dto::{ new_task_request_dto::NewTaskRequestDTO},
    message::{Message, Status},
};
use shared::db::models::TaskCore;


use serde::Deserialize;
use shared::db::models::enums::RestartPolicy;

#[derive(Deserialize)]
pub struct CreateTaskCoreDto {
    pub agent_id: i32,
    pub name: String,
    pub description: String,
    pub install_script: String,
    pub run_script: String,
    pub delete_script: String,
    pub restart_policy: RestartPolicy,
}
pub struct NewTaskHandler {
    pub pool: Arc<PgPool>,
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl NewTaskHandler {
    pub fn new(pool: Arc<PgPool>, orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { pool, orchestrator }
    }
}

#[async_trait]
impl HandlerTrait for NewTaskHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Creating new task");

        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let dto: CreateTaskCoreDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse new-task request: {}", e);
                return Message::new_response(Status::Error, None, 400, "Invalid new-task request");
            }
        };
        let manager = match self.orchestrator.get(dto.agent_id as i64).await {
            Ok(m) => m,
            Err(_) => return Message::new_response(
                Status::Error, None, 503,
                format!("Agent {} not connected", dto.agent_id),
            ),
        };
        
        let agent_task = match new_task(&manager, NewTaskRequestDTO {
            name: dto.name.clone(),
            description: dto.description.clone(),
            install_script: dto.install_script.clone(),
            run_script: dto.run_script.clone(),
            delete_script: dto.delete_script.clone(),
            restart_policy: dto.restart_policy.clone(),
        }).await {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to create task on agent: {}", e);
                return Message::new_response(Status::Error, None, 500, "Failed to create task on agent");
            }
        };

        let inserted = sqlx::query_as::<_, TaskCore>(
            r#"
            INSERT INTO tasks (id, agent_id)
            VALUES ($1, $2)
            RETURNING *
            "#
        )
            .bind(agent_task.id as i32)
            .bind(dto.agent_id)
            .fetch_one(&*self.pool)
            .await;

        match inserted {
            Ok(task) => Message::new_response(
                Status::Ok,
                Some(json!({ "task": task })),
                200,
                "Task created successfully",
            ),
            Err(e) => {
                error!("Failed to save task to DB: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to save task to database")
            }
        }
    }
}