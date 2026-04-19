use async_trait::async_trait;
use serde_json::Value;

use shared::server::message::Message;
use crate::server::connection_context::ConnectionContext;

#[async_trait]
pub trait HandlerTrait: Send + Sync {
    async fn handle(&self, data: Option<Value>, ctx: &mut ConnectionContext) -> Message;
}