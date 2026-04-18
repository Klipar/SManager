use std::sync::Arc;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shared::server::{
    connection_context::ConnectionContext,
    dto::{
        login_response_dto::LoginResponseDto,
        login_user_dto::LoginUserDto,
        user_response_dto::UserResponseDto,
    },
    get_hash::get_hash,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use sqlx::postgres::PgPool;

pub struct LoginUserHandler {
    pub pool: Arc<PgPool>,
}

#[derive(Serialize, Deserialize)]
struct LoginClaims {
    sub: i32,
    email: String,
    is_admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    exp: Option<usize>,
}

impl LoginUserHandler {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for LoginUserHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        info!("Processing login request");

        let data = match data {
            Some(v) => v,
            None => {
                return Message::new_response(Status::Error, None, 400, "Missing data");
            }
        };

        let dto: LoginUserDto = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse login request: {}", e);
                return Message::new_response(Status::Error, None, 400, "Invalid login request");
            }
        };

        let user = sqlx::query_as!(
            UserResponseDto,
            r#"
            UPDATE users
            SET last_login = NOW()
            WHERE (email = $1 OR name = $1) AND password = $2
            RETURNING
                id,
                name,
                email,
                COALESCE(is_admin, FALSE) AS "is_admin!",
                last_login AS "last_login?: chrono::NaiveDateTime",
                last_update AS "last_update?: chrono::NaiveDateTime",
                gui_settings
            "#,
            dto.login,
            get_hash(&dto.password)
        )
        .fetch_optional(&*self.pool)
        .await;

        let user = match user {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Message::new_response(Status::Error, None, 401, "Invalid credentials");
            }
            Err(e) => {
                error!("Login query failed: {}", e);
                return Message::new_response(Status::Error, None, 500, "Failed to login");
            }
        };

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
        let jwt_expiration = std::env::var("JWT_EXPIRATION")
            .ok()
            .map(|value| value.replace('_', ""))
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(86_400);

        let exp = if jwt_expiration == 0 {
            None
        } else {
            Some((Utc::now() + Duration::seconds(jwt_expiration as i64)).timestamp() as usize)
        };

        let claims = LoginClaims {
            sub: user.id,
            email: user.email.clone(),
            is_admin: user.is_admin,
            exp,
        };

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        ) {
            Ok(token) => token,
            Err(e) => {
                error!("Failed to generate login token: {}", e);
                return Message::new_response(Status::Error, None, 500, "Failed to login");
            }
        };

        let response = LoginResponseDto { token, user };

        Message::new_response(
            Status::Ok,
            Some(json!({"auth": response})),
            200,
            "Login successful",
        )
    }
}