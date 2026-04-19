use async_trait::async_trait;
use log::{error, info};
use serde_json::Value;
use crate::{handler::handler_trait::HandlerTrait, server::connection_context::ConnectionContext};
use shared::server::message::{Message, Status};
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub struct RemoveAgentHandler {
    pub pool: Arc<PgPool>,
}

impl RemoveAgentHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for RemoveAgentHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Received request removing agent");

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
                    "Invalid remove-agent request: missing id",
                );
            }
        };

        let result = sqlx::query!(
            r#"
            DELETE FROM agents
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pool)
        .await;

        match result {
            Ok(result) => {
                if result.rows_affected() == 1 {
                    Message::new_response(
                        Status::Ok,
                        None,
                        200,
                        format!("Successfully deleted agent {}", id),
                    )
                } else {
                    Message::new_response(Status::Error, None, 404, "Agent not found")
                }
            }
            Err(e) => {
                error!("Failed to delete agent: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to delete agent")
            }
        }
    }
}