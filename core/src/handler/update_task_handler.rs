use async_trait::async_trait;
use log::{error, info};
use serde_json::{json, Value};
use shared::server::{
    connection_context::ConnectionContext,
    dto::update_task_core_dto::UpdateTaskCoreDto,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use shared::db::models::TaskCore;
use sqlx::postgres::PgPool;
use std::sync::Arc;

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
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Received request for updating task");

        let data = match data {
            Some(v) => v,
            None => {
                return Message::new_response(Status::Error, None, 400, "Missing data");
            }
        };

        let dto: UpdateTaskCoreDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse update-task request: {}", e);
                return Message::new_response(Status::Error, None, 400, "Invalid update-task request");
            }
        };

        if dto.agent_id.is_none() {
            return Message::new_response(Status::Error, None, 400, "No fields to update");
        }

        let updated_task = sqlx::query_as::<_, TaskCore>(
            r#"
            UPDATE tasks
            SET agent_id = COALESCE($1, agent_id)
            WHERE id = $2
            RETURNING *
            "#
        )
        .bind(dto.agent_id)
        .bind(dto.id)
        .fetch_optional(&*self.pool)
        .await;

        match updated_task {
            Ok(Some(task)) => Message::new_response(
                Status::Ok,
                Some(json!({ "task": task })),
                200,
                "Task updated successfully",
            ),
            Ok(None) => Message::new_response(Status::Error, None, 404, "Task not found"),
            Err(e) => {
                error!("Failed to update task: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to update task")
            }
        }
    }
}