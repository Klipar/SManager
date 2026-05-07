use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use serde_json::Value;
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct UserResponseDto {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
    pub gui_settings: Option<Value>,
}