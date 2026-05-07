use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Deserialize, Serialize, FromRow)]
pub struct UsersDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
}