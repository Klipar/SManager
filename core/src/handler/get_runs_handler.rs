use async_trait::async_trait;
use log::error;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::{orchestrator::AgentOrchestrator, requests::run::get_runs},
};
use shared::server::message::{Message, Status};

pub struct GetRunsHandler {
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl GetRunsHandler {
    pub fn new(orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl HandlerTrait for GetRunsHandler {
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

        match get_runs(&manager).await {
            Ok(runs) => Message::new_response(Status::Ok, Some(json!({ "runs": runs })), 200, "Successfully retrieved runs"),
            Err(e) => {
                error!("Failed to get runs: {}", e);
                Message::new_response(Status::Error, None, 500, "Failed to get runs")
            }
        }
    }
}