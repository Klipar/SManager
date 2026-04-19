use async_trait::async_trait;
use serde_json::{json, Value};
use shared::server::{
    dto::{update_user_dto::UpdateUserDto, user_response_dto::UserResponseDto},
    get_hash::get_hash,
    message::{Message, Status},
};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use log::{info, error};

use crate::{handler::handler_trait::HandlerTrait, server::connection_context::ConnectionContext};

pub struct UpdateUserHandler {
    pub pool: Arc<PgPool>,
}

impl UpdateUserHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for UpdateUserHandler {
    async fn handle(&self, data: Option<Value>, ctx: &mut ConnectionContext)-> Message {
        info!("Received request for updating user");

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

        let dto: UpdateUserDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse update-user request: {}", e);
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid update-user request"
                );
            }
        };

        if !ctx.is_admin && dto.is_admin.is_some() {
            return Message::new_response(
                Status::Error,
                None,
                403,
                "Forbidden"
            );
        }

        if dto.name.is_none()
            && dto.email.is_none()
            && dto.password.is_none()
            && dto.is_admin.is_none()
            && dto.gui_settings.is_none()
        {
            return Message::new_response(
                Status::Error,
                None,
                400,
                "No fields to update"
            );
        }

        let password_hash = dto.password.as_ref().map(|pwd| get_hash(pwd));

        let updated_user = sqlx::query_as!(
            UserResponseDto,
            r#"
            UPDATE users
            SET
                name = COALESCE($1, name),
                email = COALESCE($2, email),
                password = COALESCE($3, password),
                is_admin = COALESCE($4, is_admin),
                gui_settings = COALESCE($5::jsonb, gui_settings),
                last_update = NOW()
            WHERE id = $6
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
            password_hash,
            dto.is_admin,
            dto.gui_settings,
            dto.id
        )
        .fetch_one(&*self.pool)
        .await;

        match updated_user{
            Ok(user) => {
                return Message::new_response (
                    Status::Ok,
                    Some(json!({"user" : user})),
                    200,
                    "Successfully updated user."
                );
            }
            Err(e) => {
                if let sqlx::Error::RowNotFound = e {
                    return Message::new_response(
                        Status::Error,
                        None,
                        404,
                        "User not found"
                    );
                }

                if let sqlx::Error::Database(db_err) = &e {
                    if db_err.code().as_deref() == Some("23505") {
                        return Message::new_response(
                            Status::Error,
                            None,
                            409,
                            "User with this email already exists."
                        );
                    }
                }

                error!("Failed to update user: {}", e);
                return Message::new_response (
                    Status::Error,
                    None,
                    500,
                    "Failed to update user"
                );
            }
        }
    }
}