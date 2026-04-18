use async_trait::async_trait;
use serde_json::{json, Value};
use shared::server::{
    connection_context::ConnectionContext,
    dto::{create_user_dto::CreateUserDto, user_response_dto::UserResponseDto},
    get_hash::get_hash,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use log::{info, error};

pub struct NewUserHandler {
    pub pool: Arc<PgPool>,
}

impl NewUserHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for NewUserHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Creating new user");

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

        let dto: CreateUserDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse create new user request: {}", e);
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid new-user request"
                );
            }
        };

        let inserted = sqlx::query_as!(
            UserResponseDto,
            r#"
            INSERT INTO users (name, email, password, is_admin)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id,
                name,
                email,
                COALESCE(is_admin, FALSE) AS "is_admin!",
                last_login AS "last_login?: chrono::NaiveDateTime",
                last_update AS "last_update?: chrono::NaiveDateTime",
                gui_settings
            "#,
            dto.name,
            dto.email,
            get_hash(&dto.password),
            dto.is_admin
        )
        .fetch_one(&*self.pool)
        .await;

        match inserted {
            Ok(user) => {
                info!("Successful created new user: `{}`", user.name);

                return Message::new_response (
                    Status::Ok,
                    Some(json!({"user": user})),
                    200,
                    "Created successfully!"
                );
            }
             Err(e) => {
                if let sqlx::Error::Database(db_err) = &e { // non unique email
                    if db_err.code().as_deref() == Some("23505") {
                        return Message::new_response(
                            Status::Error,
                            None,
                            409,
                            "User with this email already exists."
                        );
                    }
                }

                error!("Failed to create user: {}", e);
                return Message::new_response(
                    Status::Error,
                    None,
                    500,
                    "Failed to create new user."
                );
            }
        }
    }
}