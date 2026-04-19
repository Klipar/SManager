use async_trait::async_trait;
use serde_json::Value;
use shared::server::{connection_context::ConnectionContext,
                    handler_trait::HandlerTrait, message::{Message, Status}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use log::{info, error};

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
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext)-> Message {
        info!("Received request removing task");

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

        if let Some(id) = data.get("id").and_then(|v| v.as_i64()) {
            let result = sqlx::query!(
                r#"
                DELETE FROM tasks
                WHERE id = $1
                "#,
                id as i32
            )
            .execute(&*self.pool)
            .await;

            if let Ok(result) = result {
                if result.rows_affected() == 1 {
                    return Message::new_response (
                        Status::Ok,
                        None,
                        200,
                        format!("Successfully deleted task {}", id)
                    );
                }
            }
        }

        error!("Failed to delete task, bad request");
        return Message::new_response (
            Status::Error,
            None,
            400,
            "Failed to delete task, bad request"
        );
    }
}