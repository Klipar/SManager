use async_trait::async_trait;
use serde_json::Value;
use shared::server::{connection_context::ConnectionContext,
                    dto::get_cores_dto::CoresDTO,
                    handler_trait::HandlerTrait, message::{Message, Status}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use serde_json::json;
use log::{info, error};

pub struct UpdateCoreHandler {
    pub pool: Arc<PgPool>,
}

impl UpdateCoreHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for UpdateCoreHandler {
    async fn handle(&self, data: Value, _ctx: &mut ConnectionContext)-> Message {
        info!("Received request for updating core");
        let core: CoresDTO = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse update-core request: {}", e);
                return Message::new_response(
                    Status::Error,
                    json!({ "message": "Invalid update-core request" }),
                    400,
                );
            }
        };

        let updated_core = sqlx::query_as!(
            CoresDTO,
            r#"
            UPDATE cores
            SET ip = $1, port = $2, name = $3
            WHERE id = $4
            RETURNING id, ip, port, name
            "#,
            core.ip,
            core.port,
            core.name,
            core.id,
        )
        .fetch_one(&*self.pool)
        .await;

        match updated_core{
            Ok(cores) => {
                let response = json!({"message" : "Successfully updated core.", "cores" : cores});
                return Message::new_response (
                    Status::Ok,
                    response,
                    200,
                );
            }
            Err(e) => {
                error!("Failed to update core: {}", e);
                return Message::new_response (
                    Status::Error,
                    json!({ "message": "Failed to update core" }),
                    400,
                );
            }
        }
    }
}