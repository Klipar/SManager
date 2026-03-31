use async_trait::async_trait;
use serde_json::Value;
use shared::server::{connection_context::ConnectionContext,
                    dto::get_cores_dto::CoresDTO,
                    handler_trait::HandlerTrait, message::{Message, Status}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use serde_json::json;
use log::{info, error};

pub struct GetAllCoresHandler {
    pub pool: Arc<PgPool>,
}

impl GetAllCoresHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for GetAllCoresHandler {
    async fn handle(&self, _data: Value, _ctx: &mut ConnectionContext)-> Message {
        info!("Received request for extracting all cores");
        let cores = sqlx::query_as!(
            CoresDTO,
            r#"
            SELECT id, ip, name
            FROM cores
            "#
        )
        .fetch_all(&*self.pool)
        .await;

        match cores{
            Ok(cores) => {
                let cores = json!(cores);
                let response = json!({"message" : "Successfully extracted cores on this agent.", "cores" : cores});
                return Message::new_response (
                    Status::Ok,
                    response,
                    200,
                );
            }
            Err(e) => {
                error!("Failed to extract cores: {}", e);
                return Message::new_response (
                    Status::Error,
                    json!({ "message": "No cores found" }),
                    404,
                );
            }
        }
    }
}