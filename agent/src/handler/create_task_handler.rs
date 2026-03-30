use async_trait::async_trait;
use serde_json::Value;
use shared::server::{connection_context::ConnectionContext, handler_trait::HandlerTrait, message::{Message, Status}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use serde_json::json;

pub struct CreateTaskHandler {
    pub pool: Arc<PgPool>,
}

impl CreateTaskHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for CreateTaskHandler {
    async fn handle(&self, data: Value, ctx: &mut ConnectionContext)-> Message {
        println!("Creating new task on data: {}", data);
        return Message::new_response (
            Status::Error,
            json!({ "message": "Not implemented" }),
            9999,
        );
    }
}