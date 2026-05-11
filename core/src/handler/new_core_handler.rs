use async_trait::async_trait;
use log::error;
use serde_json::Value;
use std::sync::Arc;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::{orchestrator::AgentOrchestrator, requests::core::new_core},
};
use shared::server::{dto::create_core_dto::CreateCoreDto, message::{Message, Status}};

pub struct NewCoreHandler {
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl NewCoreHandler {
    pub fn new(orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl HandlerTrait for NewCoreHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let agent_id = match data.get("agent_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing agent_id"),
        };

        let dto: CreateCoreDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse new-core request: {}", e);
                return Message::new_response(Status::Error, None, 400, "Invalid new-core request");
            }
        };

        let manager = match self.orchestrator.get(agent_id).await {
            Ok(m) => m,
            Err(_) => return Message::new_response(Status::Error, None, 503, format!("Agent {} not connected", agent_id)),
        };

        match new_core(&manager, dto).await {
            Ok(_) => Message::new_response(Status::Ok, None, 200, "Core created"),
            Err(e) => {
                error!("Failed to create core: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to create core")
            }
        }
    }
}