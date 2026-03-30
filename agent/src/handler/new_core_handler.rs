use async_trait::async_trait;
use serde_json::{Value, json};
use shared::server::{connection_context::ConnectionContext, handler_trait::HandlerTrait, message::{Message, Status}};
use sqlx::postgres::PgPool;
use std::sync::Arc;

use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};
use sha2::{Sha256, Digest};

pub struct NewCoreHandler {
    pub pool: Arc<PgPool>,
}

impl NewCoreHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for NewCoreHandler {
    async fn handle(&self, data: Value, ctx: &mut ConnectionContext) -> Message {
        println!("Creating new core using data: {}", data);

        let mut bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);

        // encode in Base64
        let token = general_purpose::STANDARD.encode(&bytes);

        println!("{}", token);


        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let result = hasher.finalize();

        // encode hash in Base64
        let hash_base64 = general_purpose::STANDARD.encode(result);

        println!("{}", hash_base64);
        return Message::new_response (
            Status::Error,
            json!({ "message": "Not implemented" }),
            9999,
        );
    }
}