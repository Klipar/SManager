use async_trait::async_trait;
use serde_json::Value;
use shared::server::{connection_context::ConnectionContext,
                    handler_trait::HandlerTrait, message::{Message, Status}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use serde_json::json;
use log::{info, error};

pub struct RemoveCoreHandler {
    pub pool: Arc<PgPool>,
}

impl RemoveCoreHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for RemoveCoreHandler {
    async fn handle(&self, data: Value, _ctx: &mut ConnectionContext)-> Message {
        info!("Received request removing core");

        if let Some(id) = data.get("id").and_then(|v| v.as_i64()) {
            let result = sqlx::query!(
                r#"
                DELETE FROM cores
                WHERE id = $1
                "#,
                id as i32
            )
            .execute(&*self.pool)
            .await;

            if let Ok(result) = result {
                if result.rows_affected() == 1 {
                    let response = json!({"message" : format!("Successfully deleted core {}", id)});
                    return Message::new_response (
                        Status::Ok,
                        response,
                        200,
                    );
                }
            }
        }

        error!("Failed to delete core, bad request");
        return Message::new_response (
            Status::Error,
            json!({ "message": "Failed to delete core, bad request" }),
            400,
        );
    }
}