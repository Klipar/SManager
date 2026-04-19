use async_trait::async_trait;
use log::{error, info};
use serde_json::{json, Value};
use crate::{handler::handler_trait::HandlerTrait, server::connection_context::ConnectionContext};
use shared::server::message::{Message, Status};
use shared::db::models::Agent;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub struct GetAllAgentsHandler {
    pub pool: Arc<PgPool>,
}

impl GetAllAgentsHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for GetAllAgentsHandler {
    async fn handle(&self, _data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Received request for extracting all agents");

        let agents = sqlx::query_as::<_, Agent>(
            "SELECT * FROM agents ORDER BY id ASC"
        )
        .fetch_all(&*self.pool)
        .await;

        match agents {
            Ok(agents) => Message::new_response(
                Status::Ok,
                Some(json!({ "agents": agents })),
                200,
                "Successfully extracted agents",
            ),
            Err(e) => {
                error!("Failed to extract agents: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to extract agents")
            }
        }
    }
}