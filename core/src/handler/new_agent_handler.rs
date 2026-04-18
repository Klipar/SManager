use async_trait::async_trait;
use log::{error, info};
use serde_json::{json, Value};
use shared::server::{
    connection_context::ConnectionContext,
    dto::create_agent_dto::CreateAgentDto,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use shared::db::models::Agent;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub struct NewAgentHandler {
    pub pool: Arc<PgPool>,
}

impl NewAgentHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for NewAgentHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Creating new agent");

        let data = match data {
            Some(v) => v,
            None => {
                return Message::new_response(Status::Error, None, 400, "Missing data");
            }
        };

        let dto: CreateAgentDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse new-agent request: {}", e);
                return Message::new_response(Status::Error, None, 400, "Invalid new-agent request");
            }
        };

        let inserted = sqlx::query_as::<_, Agent>(
            r#"
            INSERT INTO agents (ip, port, name, description)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(dto.ip)
        .bind(dto.port)
        .bind(dto.name)
        .bind(dto.description)
        .fetch_one(&*self.pool)
        .await;

        match inserted {
            Ok(agent) => Message::new_response(
                Status::Ok,
                Some(json!({ "agent": agent })),
                200,
                "Agent created successfully",
            ),
            Err(e) => {
                error!("Failed to create agent: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to create agent")
            }
        }
    }
}