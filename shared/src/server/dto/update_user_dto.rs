use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct UpdateUserDto {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_admin: Option<bool>,
    pub gui_settings: Option<Value>,
}