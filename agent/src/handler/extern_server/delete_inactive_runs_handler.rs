use async_trait::async_trait;
use serde_json::Value;
use shared::server::{connection_context::ConnectionContext, handler_trait::HandlerTrait, message::{Message, Status}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use serde_json::json;

use log::{info, error};

pub struct DeleteInactiveRunsHandler {
    pub pool: Arc<PgPool>,
}

impl DeleteInactiveRunsHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for DeleteInactiveRunsHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Deleting all inactive runs");

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
            let delete_result = sqlx::query(
                "DELETE FROM runs WHERE return_code IS NOT NULL AND end_time IS NOT NULL AND task_id = $1"
            )
            .bind(id as i32)
            .execute(&*self.pool)
            .await;
            match delete_result {
            Ok(result) => {
                let rows_affected = result.rows_affected();
                info!("Successfully deleted {} inactive runs", rows_affected);

                Message::new_response(
                    Status::Ok,
                    Some(json!({
                        "deleted_count": rows_affected,
                        "message": format!("Successfully deleted {} inactive runs", rows_affected)
                    })),
                    200,
                    &format!("Successfully deleted {} inactive runs", rows_affected)
                )
            }
            Err(e) => {
                error!("Failed to delete inactive runs: {}", e);
                Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Failed to delete inactive runs"
                )
            }
        }
        } else {
            error!("Failed to delete inactive runs, bad request");
            Message::new_response (
                Status::Error,
                None,
                400,
                "Failed to delete runs, bad request"
            )
        }
    }
}