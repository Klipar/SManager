use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    tls_client::orchestrator::AgentOrchestrator,
};
use shared::server::message::{Message, Status};

pub struct DisconnectAgentHandler {
    pub orchestrator: Arc<AgentOrchestrator>,
}

impl DisconnectAgentHandler {
    pub fn new(orchestrator: Arc<AgentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

#[async_trait]
impl HandlerTrait for DisconnectAgentHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let agent_id = match data.get("agent_id").and_then(|v| v.as_i64()) {
            Some(id) => id,
            None => return Message::new_response(Status::Error, None, 400, "Missing agent_id"),
        };

        self.orchestrator.disconnect(agent_id).await;
        Message::new_response(Status::Ok, None, 200, "Disconnected from agent")
    }
}