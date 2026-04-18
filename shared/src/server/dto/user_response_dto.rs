use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct UserResponseDto {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub last_login: Option<String>,
    pub last_update: Option<String>,
    pub gui_settings: Option<Value>,
}