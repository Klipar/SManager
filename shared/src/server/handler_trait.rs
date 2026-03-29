use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait HandlerTrait: Send + Sync {
    async fn handle(&self, data: Value);
}