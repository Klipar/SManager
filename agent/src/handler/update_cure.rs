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
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext)-> Message {
        info!("Received request for updating core");

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

        let core: CoresDTO = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse update-core request: {}", e);
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid update-core request"
                );
            }
        };

        let updated_core = sqlx::query_as!(
            CoresDTO,
            r#"
            UPDATE cores
            SET ip = $1, name = $2
            WHERE id = $3
            RETURNING id, ip, name
            "#,
            core.ip,
            core.name,
            core.id,
        )
        .fetch_one(&*self.pool)
        .await;

        match updated_core{
            Ok(core) => {
                return Message::new_response (
                    Status::Ok,
                    Some(json!({"core" : core})),
                    200,
                    "Successfully updated core."
                );
            }
            Err(e) => {
                error!("Failed to update core: {}", e);
                return Message::new_response (
                    Status::Error,
                    None,
                    400,
                    "Failed to update core"
                );
            }
        }
    }
}