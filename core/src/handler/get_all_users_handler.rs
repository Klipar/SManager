use async_trait::async_trait;
use serde_json::Value;
use shared::server::{connection_context::ConnectionContext,
                    dto::get_users_dto::UsersDTO,
                    handler_trait::HandlerTrait, message::{Message, Status}};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use serde_json::json;
use log::{info, error};

pub struct GetAllUsersHandler {
    pub pool: Arc<PgPool>,
}

impl GetAllUsersHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for GetAllUsersHandler {
    async fn handle(&self, _data: Option<Value>, _ctx: &mut ConnectionContext)-> Message {
        info!("Received request for extracting all users");
        let users = sqlx::query_as!(
            UsersDTO,
            r#"
            SELECT
                id,
                name,
                email,
                COALESCE(is_admin, FALSE) AS "is_admin!",
                last_login,
                last_update
            FROM users
            "#
        )
        .fetch_all(&*self.pool)
        .await;

        match users{
            Ok(users) => {
                return Message::new_response (
                    Status::Ok,
                    Some(json!({"users" : users})),
                    200,
                    "Successfully extracted users."
                );
            }
            Err(e) => {
                error!("Failed to extract users: {}", e);
                return Message::new_response (
                    Status::Error,
                    None,
                    404,
                    "No users found"
                );
            }
        }
    }
}