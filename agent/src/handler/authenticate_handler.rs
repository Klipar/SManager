use async_trait::async_trait;
use serde_json::Value;
use shared::{db::models::Core, server::{connection_context::ConnectionContext, handler_trait::HandlerTrait, message::{Message, Status}}};
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
    async fn handle(&self, data: Option<Value>, ctx: &mut ConnectionContext) -> Message {
        if ctx.authenticated{
            error!("Received authenticate request for already authenticated socket...");
            return Message::new_response (
                Status::Error,
                None,
                401,
                "Double authorization",
            );
        }

        if ctx.ip == "0.0.0.0"{
            error!("Failed to check ip, unable to get it...");
            return Message::new_response (
                Status::Error,
                None,
                500,
                "Socket error, unable to check ip",
            );
        }

        info!("Received authenticate request");

        match data {
            Some(data) => {
                if let Some(token) = data.get("token").and_then(|v| v.as_str()) {
                    let mut hasher = Sha256::new();
                    hasher.update(token.as_bytes());
                    let result = hasher.finalize();

                    let token_hash = general_purpose::STANDARD.encode(result);

                    let core = sqlx::query_as::<_, Core>(
                        "SELECT * FROM cores WHERE token_hash = $1 AND ip = $2"
                    )
                    .bind(token_hash)
                    .bind(&ctx.ip)
                    .fetch_one(&*self.pool)
                    .await;

                    match core {
                        Ok(core) => {
                            info!("Successful authentication for: `{}`", core.name);
                            ctx.authenticated = true;
                            ctx.id = Some(core.id);
                            return Message::new_response (
                                Status::Ok,
                                None,
                                200,
                                "Authorized successfully!"
                            );
                        }
                        Err(e) => {
                            warn!("Failed to authentication client using token: {}", token);
                            debug!("{}", e);

                            return Message::new_response (
                                Status::Error,
                                None,
                                401,
                                "Failed to authenticate. Your token invalid ur assigned to different ip address. If you using proxy you should change assigned ip."
                            );
                        }
                    }
                } else {
                    return Message::new_response (
                        Status::Error,
                        None,
                        400,
                        "Invalid auth request, field 'data.token' is not exist or not string.");
                }
            },
            None => {
                return Message::new_response (
                    Status::Error,
                    None,
                    400,
                    "Invalid auth request, field 'data' is not exist."
                );
            },
        }
    }
}