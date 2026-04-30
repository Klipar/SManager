use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use serde_json::Value;
use crate::server::connection_context::ConnectionContext;

#[derive(Debug, Deserialize)]
pub struct AuthClaims {
    pub sub: i32,
    pub is_admin: bool,
}

pub fn extract_token(data: &Option<Value>) -> Option<String> {
    data.as_ref()?
        .get("token")?
        .as_str()
        .map(|v| v.to_string())
}

fn get_target_user_id(data: &Option<Value>) -> Option<i32> {
    data.as_ref()?
        .get("id")?
        .as_i64()
        .map(|id| id as i32)
}

pub fn is_action_allowed(action: &str, data: &Option<Value>, ctx: &ConnectionContext) -> bool {
    if ctx.is_admin {
        return true;
    }

    match action {
        "update-user" | "remove-user" => {
            match (ctx.user_id, get_target_user_id(data)) {
                (Some(current_user_id), Some(target_user_id)) => current_user_id == target_user_id,
                _ => false,
            }
        }
        _ => false,
    }
}

pub fn verify_token(token: &str) -> Result<AuthClaims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    let mut validation = Validation::default();
    validation.required_spec_claims.remove("exp");

    decode::<AuthClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
}