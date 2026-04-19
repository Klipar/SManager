use async_trait::async_trait;
use log::{error, info};
use serde_json::{json, Value};
use crate::{handler::handler_trait::HandlerTrait, server::connection_context::ConnectionContext};
use shared::server::{dto::update_agent_dto::UpdateAgentDto, message::{Message, Status}};
use shared::db::models::Agent;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub struct UpdateAgentHandler {
    pub pool: Arc<PgPool>,
}

impl UpdateAgentHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for UpdateAgentHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Received request for updating agent");

        let data = match data {
            Some(v) => v,
            None => {
                return Message::new_response(Status::Error, None, 400, "Missing data");
            }
        };

        let dto: UpdateAgentDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse update-agent request: {}", e);
                return Message::new_response(Status::Error, None, 400, "Invalid update-agent request");
            }
        };

        if dto.ip.is_none() && dto.port.is_none() && dto.name.is_none() && dto.description.is_none() {
            return Message::new_response(Status::Error, None, 400, "No fields to update");
        }

        let updated_agent = sqlx::query_as::<_, Agent>(
            r#"
            UPDATE agents
            SET
                ip = COALESCE($1, ip),
                port = COALESCE($2, port),
                name = COALESCE($3, name),
                description = COALESCE($4, description)
            WHERE id = $5
            RETURNING *
            "#
        )
        .bind(dto.ip)
        .bind(dto.port)
        .bind(dto.name)
        .bind(dto.description)
        .bind(dto.id)
        .fetch_optional(&*self.pool)
        .await;

        match updated_agent {
            Ok(Some(agent)) => Message::new_response(
                Status::Ok,
                Some(json!({ "agent": agent })),
                200,
                "Successfully updated agent",
            ),
            Ok(None) => Message::new_response(Status::Error, None, 404, "Agent not found"),
            Err(e) => {
                error!("Failed to update agent: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to update agent")
            }
        }
    }
}