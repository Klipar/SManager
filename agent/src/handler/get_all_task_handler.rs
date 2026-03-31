use async_trait::async_trait;
use serde_json::Value;
use shared::{db::models::Task, server::{connection_context::ConnectionContext, handler_trait::HandlerTrait, message::{Message, Status}}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use serde_json::json;

use log::{info, error};

pub struct GetAllTaskHandler {
    pub pool: Arc<PgPool>,
}

impl GetAllTaskHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for GetAllTaskHandler {
    async fn handle(&self, _data: Value, _ctx: &mut ConnectionContext)-> Message {
        info!("Extracting all tasks");

        let tasks = sqlx::query_as::<_, Task>(
            "SELECT * FROM tasks"
        )
        .fetch_all(&*self.pool)
        .await;

        match tasks {
            Ok(tasks) => {
                let response = json!({
                    "message" : "Successfully extracted all tasks.",
                    "task" : tasks
                });
                return Message::new_response (
                    Status::Ok,
                    response,
                    200,
                );
            }
            Err(e) => {
                error!("Failed to extract tasks: {}", e);
                return Message::new_response (
                    Status::Error,
                    json!({ "message": "Failed to extract tasks" }),
                    400,
                );
            }
        }
    }
}