use async_trait::async_trait;
use serde_json::{Value, json};
use shared::{db::models::Core, server::{connection_context::ConnectionContext,
                                        dto::create_core_dto::CreateCoreDto,
                                        generate_token::generate_token,
                                        handler_trait::HandlerTrait,
                                        message::{Message, Status}}};
use sqlx::postgres::PgPool;
use std::sync::Arc;

use log::{info, error};

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
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Creating new core");

        let data = match data {
            Some(v) => v,
            None => {
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Missing data"
                );
            }
        };

        let dto: CreateCoreDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse create new core request: {}", e);
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid new-core request"
                );
            }
        };

        let (token, hash) = generate_token();

        let inserted = sqlx::query_as!(
            Core,
            r#"
            INSERT INTO cores (ip, name, token_hash)
            VALUES ($1, $2, $3)
            RETURNING id, ip, name, token_hash
            "#,
            dto.ip,
            dto.name,
            hash
        )
        .fetch_one(&*self.pool)
        .await;

        match inserted {
            Ok(core) => {
                info!("Successful created new core: `{}`", core.name);

                return Message::new_response (
                    Status::Ok,
                    Some(json!({"token" : token})),
                    200,
                    "Created successfully!"
                );
            }
             Err(e) => {
                if let sqlx::Error::Database(db_err) = &e { // non uniq ip-port
                    if let Some(constraint) = db_err.constraint() {
                        if constraint == "unique_ip_port" {
                            return Message::new_response(
                                Status::Error,
                                None,
                                409,
                                "Core with this IP and port already exists."
                            );
                        }
                    }
                }

                error!("Failed to create core: {}", e);
                return Message::new_response(
                    Status::Error,
                    None,
                    500,
                    "Failed to create new core."
                );
            }
        }
    }
}