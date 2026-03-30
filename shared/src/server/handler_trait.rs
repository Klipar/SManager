use async_trait::async_trait;
use serde_json::Value;

use crate::server::{connection_context::ConnectionContext, message::Message};

#[async_trait]
pub trait HandlerTrait: Send + Sync {
    async fn handle(&self, data: Value, ctx: &mut ConnectionContext) -> Message;
}