use async_trait::async_trait;
use log::error;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::{orchestrator::AgentOrchestrator, requests::core::update_core},
};
use shared::server::{dto::get_cores_dto::CoresDTO, message::{Message, Status}};

pub struct UpdateCoreHandler {
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl UpdateCoreHandler {
    pub fn new(orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl HandlerTrait for UpdateCoreHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let agent_id = match data.get("agent_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing agent_id"),
        };

        let dto: CoresDTO = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse update-core request: {}", e);
                return Message::new_response(Status::Error, None, 400, "Invalid update-core request");
            }
        };

        let manager = match self.orchestrator.get(agent_id).await {
            Ok(m) => m,
            Err(_) => return Message::new_response(Status::Error, None, 503, format!("Agent {} not connected", agent_id)),
        };

        match update_core(&manager, dto).await {
            Ok(core) => Message::new_response(Status::Ok, Some(json!({ "core": core })), 200, "Core updated"),
            Err(e) => {
                error!("Failed to update core: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to update core")
            }
        }
    }
}