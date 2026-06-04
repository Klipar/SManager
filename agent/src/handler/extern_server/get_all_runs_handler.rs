use async_trait::async_trait;
use serde_json::Value;
use serde_json::json;
use shared::{
    db::models::Run,
    server::{
        connection_context::ConnectionContext,
        handler_trait::HandlerTrait,
        message::{Message, Status},
    },
};
use sqlx::SqlitePool;
use std::sync::Arc;

use log::{error, info};

pub struct GetAllRunsHandler {
    pub pool: Arc<SqlitePool>,
}

impl GetAllRunsHandler {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for GetAllRunsHandler {
    async fn handle(&self, _data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Extracting all runs");

        let runs = sqlx::query_as::<_, Run>("SELECT * FROM runs")
            .fetch_all(&*self.pool)
            .await;

        match runs {
            Ok(runs) => {
                return Message::new_response(
                    Status::Ok,
                    Some(json!({"runs" : runs})),
                    200,
                    "Successfully extracted all runs.",
                );
            }
            Err(e) => {
                error!("Failed to extract runs: {}", e);
                return Message::new_response(Status::Error, None, 400, "Failed to extract runs");
            }
        }
    }
}
