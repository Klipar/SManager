use async_trait::async_trait;
use log::{error, info};
use serde_json::Value;
use shared::server::{
    connection_context::ConnectionContext,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub struct RemoveTaskHandler {
    pub pool: Arc<PgPool>,
}

impl RemoveTaskHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for RemoveTaskHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Received request removing task");

        let data = match data {
            Some(v) => v,
            None => {
                return Message::new_response(Status::Error, None, 400, "Missing data");
            }
        };

        let id = match data.get("id").and_then(|v| v.as_i64()) {
            Some(id) => id as i32,
            None => {
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid remove-task request: missing id",
                );
            }
        };

        let result = sqlx::query!(
            r#"
            DELETE FROM tasks
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await;

        match result {
            Ok(res) => {
                if res.rows_affected() > 0 {
                    Message::new_response(
                        Status::Ok,
                        None,
                        200,
                        "Task removed successfully",
                    )
                } else {
                    Message::new_response(Status::Error, None, 404, "Task not found")
                }
            }
            Err(e) => {
                error!("Failed to delete task: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to delete task")
            }
        }
    }
}