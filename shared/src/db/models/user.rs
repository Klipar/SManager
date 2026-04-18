use serde::{Serialize, Deserialize};
use serde_json::Value;
use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub last_login: Option<NaiveDateTime>,
    pub last_update: Option<NaiveDateTime>,
    pub gui_settings: Option<Value>,
}