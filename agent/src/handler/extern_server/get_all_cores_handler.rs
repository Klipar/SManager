use async_trait::async_trait;
use log::{error, info};
use serde_json::Value;
use serde_json::json;
use shared::server::{
    connection_context::ConnectionContext,
    dto::get_cores_dto::CoresDTO,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct GetAllCoresHandler {
    pub pool: Arc<SqlitePool>,
}

impl GetAllCoresHandler {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for GetAllCoresHandler {
    async fn handle(&self, _data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
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

        match cores {
            Ok(cores) => {
                return Message::new_response(
                    Status::Ok,
                    Some(json!({"cores" : cores})),
                    200,
                    "Successfully extracted cores on this agent.",
                );
            }
            Err(e) => {
                error!("Failed to extract cores: {}", e);
                return Message::new_response(Status::Error, None, 404, "No cores found");
            }
        }
    }
}
