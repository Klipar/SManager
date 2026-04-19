use async_trait::async_trait;
use log::{error, info};
use serde_json::{json, Value};
use crate::{handler::handler_trait::HandlerTrait, server::connection_context::ConnectionContext};
use shared::server::message::{Message, Status};
use shared::db::models::TaskCore;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub struct GetAllTasksHandler {
    pub pool: Arc<PgPool>,
}

impl GetAllTasksHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for GetAllTasksHandler {
    async fn handle(&self, _data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Received request for extracting all tasks");

        let tasks = sqlx::query_as::<_, TaskCore>(
            "SELECT * FROM tasks ORDER BY id ASC"
        )
        .fetch_all(&*self.pool)
        .await;

        match tasks {
            Ok(tasks) => Message::new_response(
                Status::Ok,
                Some(json!({ "tasks": tasks })),
                200,
                "Successfully retrieved all tasks",
            ),
            Err(e) => {
                error!("Failed to retrieve tasks: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to retrieve tasks")
            }
        }
    }
}