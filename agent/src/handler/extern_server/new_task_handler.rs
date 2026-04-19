use async_trait::async_trait;
use serde_json::Value;
use shared::{db::models::{Task, TaskStatus}, server::{connection_context::ConnectionContext, dto::new_task_request_dto::NewTaskRequestDTO, handler_trait::HandlerTrait, message::{Message, Status}}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use serde_json::json;

use log::{info, error};

pub struct NewTaskHandler {
    pub pool: Arc<PgPool>,
}

impl NewTaskHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for NewTaskHandler {
    async fn handle(&self, data: Option<Value>, ctx: &mut ConnectionContext)-> Message {
        info!("Creating new task");

        let data = match data {
            Some(v) => v,
            None => {
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Missing data"
                );
            }
        };

        let dto: NewTaskRequestDTO = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse create new task request: {}", e);
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid new-task request"
                );
            }
        };

        let inserted = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (
                core_id, name, description,
                install_script, run_script, delete_script,
                restart_policy, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                id, core_id, name, description,
                install_script, run_script, delete_script,
                restart_policy, status
            "#
        )
        .bind(ctx.id)
        .bind(dto.name)
        .bind(dto.description)
        .bind(dto.install_script)
        .bind(dto.run_script)
        .bind(dto.delete_script)
        .bind(dto.restart_policy)
        .bind(TaskStatus::Stopped)
        .fetch_one(&*self.pool)
        .await;

        match inserted {
            Ok(task) => {
                return Message::new_response (
                    Status::Ok,
                    Some(json!({"task" : task})),
                    200,
                    "Successfully created new task."
                );
            }
            Err(e) => {
                error!("Failed to create new task: {}", e);
                return Message::new_response (
                    Status::Error,
                    None,
                    400,
                    "Failed to create new task"
                );
            }
        }
    }
}