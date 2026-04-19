use async_trait::async_trait;
use log::{error, info};
use serde_json::{json, Value};
use crate::{handler::handler_trait::HandlerTrait, server::connection_context::ConnectionContext};
use shared::server::{dto::create_task_core_dto::CreateTaskCoreDto, message::{Message, Status}};
use shared::db::models::TaskCore;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub struct NewTaskHandler {
    pub pool: Arc<PgPool>,
}

impl NewTaskHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for NewTaskHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Creating new task");

        let data = match data {
            Some(v) => v,
            None => {
                return Message::new_response(Status::Error, None, 400, "Missing data");
            }
        };

        let dto: CreateTaskCoreDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse new-task request: {}", e);
                return Message::new_response(Status::Error, None, 400, "Invalid new-task request");
            }
        };

        let inserted = sqlx::query_as::<_, TaskCore>(
            r#"
            INSERT INTO tasks (id, agent_id)
            VALUES ($1, $2)
            RETURNING *
            "#
        )
        .bind(dto.id)
        .bind(dto.agent_id)
        .fetch_one(&*self.pool)
        .await;

        match inserted {
            Ok(task) => Message::new_response(
                Status::Ok,
                Some(json!({ "task": task })),
                200,
                "Task created successfully",
            ),
            Err(e) => {
                error!("Failed to create task: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to create task")
            }
        }
    }
}