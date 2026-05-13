use async_trait::async_trait;
use log::error;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::{orchestrator::AgentOrchestrator, requests::core::get_all_cores},
};
use shared::server::message::{Message, Status};

pub struct GetAllCoresHandler {
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl GetAllCoresHandler {
    pub fn new(orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl HandlerTrait for GetAllCoresHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let agent_id = match data.get("agent_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing agent_id"),
        };

        let manager = match self.orchestrator.get(agent_id).await {
            Ok(m) => m,
            Err(_) => return Message::new_response(Status::Error, None, 503, format!("Agent {} not connected", agent_id)),
        };

        match get_all_cores(&manager).await {
            Ok(cores) => Message::new_response(Status::Ok, Some(json!({ "cores": cores })), 200, "Successfully retrieved cores"),
            Err(e) => {
                error!("Failed to get cores: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to get cores")
            }
        }
    }
}