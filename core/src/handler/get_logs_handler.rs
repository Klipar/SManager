use async_trait::async_trait;
use log::{error, info};
use serde_json::{json, Value};
use shared::server::{
    connection_context::ConnectionContext,
    dto::get_logs_dto::GetLogsDto,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use shared::db::models::Log;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub struct GetLogsHandler {
    pub pool: Arc<PgPool>,
}

impl GetLogsHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for GetLogsHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Received request for extracting logs");

        let dto: GetLogsDto = match data {
            Some(v) => match serde_json::from_value(v) {
                Ok(dto) => dto,
                Err(_) => GetLogsDto {
                    agent_id: None,
                    task_id: None,
                    user_id: None,
                    limit: None,
                    offset: None,
                },
            },
            None => GetLogsDto {
                agent_id: None,
                task_id: None,
                user_id: None,
                limit: None,
                offset: None,
            },
        };

        let limit = dto.limit.unwrap_or(100).min(1000);
        let offset = dto.offset.unwrap_or(0);

        let logs = sqlx::query_as::<_, Log>(
            r#"
            SELECT * FROM logs
            WHERE
                (agent_id = $1 OR $1 IS NULL) AND
                (task_id = $2 OR $2 IS NULL) AND
                (user_id = $3 OR $3 IS NULL)
            ORDER BY timestamp DESC
            LIMIT $4 OFFSET $5
            "#
        )
        .bind(dto.agent_id)
        .bind(dto.task_id)
        .bind(dto.user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&*self.pool)
        .await;

        match logs {
            Ok(logs) => Message::new_response(
                Status::Ok,
                Some(json!({ "logs": logs })),
                200,
                "Successfully retrieved logs",
            ),
            Err(e) => {
                error!("Failed to retrieve logs: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to retrieve logs")
            }
        }
    }
}