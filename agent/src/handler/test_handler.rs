use async_trait::async_trait;
use serde_json::Value;
use shared::server::handler_trait::HandlerTrait;
use sqlx::postgres::PgPool;
use std::sync::Arc;

pub struct TestHandler {
    pub pool: Arc<PgPool>,
}

impl TestHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for TestHandler {
    async fn handle(&self, data: Value) {
        println!("TestHandler received data: {}", data);

        if let Some(msg) = data.get("message").and_then(|v| v.as_str()) {
            println!("Message field: {}", msg);
        } else {
            println!("No 'message' field found");
        }
    }
}