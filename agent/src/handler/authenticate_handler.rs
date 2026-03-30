use async_trait::async_trait;
use serde_json::Value;
use shared::{db::models::Core, server::{connection_context::ConnectionContext, handler_trait::HandlerTrait}};
use sqlx::postgres::PgPool;
use std::sync::Arc;

use base64::{engine::general_purpose, Engine as _};
use sha2::{Sha256, Digest};

use log::{info, warn, error, debug};

pub struct AuthenticateHandler {
    pub pool: Arc<PgPool>,
}

impl AuthenticateHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for AuthenticateHandler {
    async fn handle(&self, data: Value, ctx: &mut ConnectionContext) {
        if ctx.authenticated{
            error!("Received authenticate request for already authenticated socket...");
            let _ = ctx.send_response("Fails, second auth").await;  //TODO: normal response
            return;
        }
        info!("Received authenticate request");

        if let Some(token) = data.get("token").and_then(|v| v.as_str()) {
            let mut hasher = Sha256::new();
            hasher.update(token.as_bytes());
            let result = hasher.finalize();

            let token_hash = general_purpose::STANDARD.encode(result);

            let core = sqlx::query_as::<_, Core>(
                "SELECT * FROM cores WHERE token_hash = $1"
            )
            .bind(token_hash)
            .fetch_one(&*self.pool)
            .await;

            match core {
                Ok(core) => {
                    info!("Successful authentication for: `{}`", core.name);
                    ctx.authenticated = true;
                    let _ = ctx.send_response("OK").await;
                }
                Err(e) => {
                    let addr = ctx.framed.get_ref().peer_addr().unwrap();
                    warn!("Failed to authentication client [{}:{}] using token: {}", addr.ip(), addr.port(), token);
                    debug!("{}", e);

                    let _ = ctx.send_response("Failed to fetch core").await; //TODO: normal response
                }
            }
        } else {
            let addr = ctx.framed.get_ref().peer_addr().unwrap();
            error!("Received invalid `authenticate` from [{}:{}], request: {}", addr.ip(), addr.port(), data);
            let _ = ctx.send_response("Fails").await;  //TODO: normal response
        }
    }
}