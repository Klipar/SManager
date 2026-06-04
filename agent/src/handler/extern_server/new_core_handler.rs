use async_trait::async_trait;
use log::{error, info};
use serde_json::Value;
use shared::server::{
    connection_context::ConnectionContext,
    dto::create_core_dto::CreateCoreDto,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct NewCoreHandler {
    pub pool: Arc<SqlitePool>,
}

impl NewCoreHandler {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for NewCoreHandler {
    async fn handle(&self, data: Option<Value>, ctx: &mut ConnectionContext) -> Message {
        info!("Creating new core");

        let data = match data {
            Some(v) => v,
            None => return Message::new_response(Status::Error, None, 400, "Missing data"),
        };

        let dto: CreateCoreDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse create new core request: {}", e);
                return Message::new_response(Status::Error, None, 400, "Invalid new-core request");
            }
        };

        let inserted = sqlx::query_as!(
            Core,
            r#"
            INSERT INTO cores (spiffe_id, create_by_core_id, create_at, update_by_core_id, update_at)
            VALUES (?, ?, ?, ?, ?)
            RETURNING id
            "#,
            dto.ip,
            ctx.id,
        )
        .fetch_one(&self.pool)
        .await?;

        match inserted {
            Ok(core) => {
                info!("Successful created new core: `{}`", core.name);

                return Message::new_response(Status::Ok, None, 200, "Created successfully!");
            }
            Err(e) => {
                if let sqlx::Error::Database(db_err) = &e {
                    // non uniq ip-port
                    if let Some(constraint) = db_err.constraint() {
                        if constraint == "unique_ip_port" {
                            return Message::new_response(
                                Status::Error,
                                None,
                                409,
                                "Core with this IP and port already exists.",
                            );
                        }
                    }
                }

                error!("Failed to create core: {}", e);
                return Message::new_response(
                    Status::Error,
                    None,
                    500,
                    "Failed to create new core.",
                );
            }
        }
    }
}
