use async_trait::async_trait;
use serde_json::Value;
use serde_json::json;
use shared::{
    db::models::Task,
    server::{
        connection_context::ConnectionContext,
        handler_trait::HandlerTrait,
        message::{Message, Status},
    },
};
use sqlx::SqlitePool;
use std::sync::Arc;

use log::{error, info};

pub struct GetAllTasksHandler {
    pub pool: Arc<SqlitePool>,
}

impl GetAllTasksHandler {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for GetAllTasksHandler {
    async fn handle(&self, _data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Extracting all tasks");

        let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
            .fetch_all(&*self.pool)
            .await;

        match tasks {
            Ok(tasks) => {
                return Message::new_response(
                    Status::Ok,
                    Some(json!({"tasks" : tasks})),
                    200,
                    "Successfully extracted all tasks.",
                );
            }
            Err(e) => {
                error!("Failed to extract tasks: {}", e);
                return Message::new_response(Status::Error, None, 400, "Failed to extract tasks");
            }
        }
    }
}
