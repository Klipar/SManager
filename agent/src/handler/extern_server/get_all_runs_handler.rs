use async_trait::async_trait;
use serde_json::Value;
use shared::{db::models::Run, server::{connection_context::ConnectionContext, handler_trait::HandlerTrait, message::{Message, Status}}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use serde_json::json;

use log::{info, error};

pub struct GetAllRunsHandler {
    pub pool: Arc<PgPool>,
}

impl GetAllRunsHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for GetAllRunsHandler {
    async fn handle(&self, _data: Option<Value>, _ctx: &mut ConnectionContext)-> Message {
        info!("Extracting all runs");

        let runs = sqlx::query_as::<_, Run>(
            "SELECT * FROM runs"
        )
        .fetch_all(&*self.pool)
        .await;

        match runs {
            Ok(runs) => {
                return Message::new_response (
                    Status::Ok,
                    Some(json!({"runs" : runs})),
                    200,
                    "Successfully extracted all runs."
                );
            }
            Err(e) => {
                error!("Failed to extract runs: {}", e);
                return Message::new_response (
                    Status::Error,
                    None,
                    400,
                    "Failed to extract runs"
                );
            }
        }
    }
}