use async_trait::async_trait;
use serde_json::Value;
use shared::{db::models::Task, server::{connection_context::ConnectionContext, dto::update_task_dto::UpdateTaskDTO, handler_trait::HandlerTrait, message::{Message, Status}}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use serde_json::json;
use log::{info, error};

pub struct UpdateTaskHandler {
    pub pool: Arc<PgPool>,
}

impl UpdateTaskHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for UpdateTaskHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext)-> Message {
        info!("Received request for updating task");

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

        let task: UpdateTaskDTO = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse update-task request: {}", e);
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid update-task request"
                );
            }
        };

        let updated = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET
                name = $1,
                description = $2,
                install_script = $3,
                run_script = $4,
                delete_script = $5,
                restart_policy = $6
            WHERE id = $7
            RETURNING
                id, core_id, name, description,
                install_script, run_script, delete_script,
                restart_policy, status
            "#
        )
        .bind(&task.name)
        .bind(&task.description)
        .bind(&task.install_script)
        .bind(&task.run_script)
        .bind(&task.delete_script)
        .bind(task.restart_policy)
        .bind(task.id)
        .fetch_optional(&*self.pool)
        .await;

        match updated{
            Ok(task) => {
                return Message::new_response (
                    Status::Ok,
                    Some(json!({"task" : task})),
                    200,
                    "Successfully updated task."
                );
            }
            Err(e) => {
                error!("Failed to update task: {}", e);
                return Message::new_response (
                    Status::Error,
                    None,
                    400,
                    "Failed to update task"
                );
            }
        }
    }
}