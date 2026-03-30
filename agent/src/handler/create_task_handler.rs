use async_trait::async_trait;
use serde_json::Value;
use shared::server::{connection_context::ConnectionContext, handler_trait::HandlerTrait};
use sqlx::postgres::PgPool;
use std::sync::Arc;

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
    async fn handle(&self, data: Value, ctx: &mut ConnectionContext) {
        println!("Creating new task on data: {}", data);


    }
}