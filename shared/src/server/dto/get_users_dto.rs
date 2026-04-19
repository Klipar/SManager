use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize)]
pub struct UsersDTO {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
    pub last_login: Option<NaiveDateTime>,
    pub last_update: Option<NaiveDateTime>,
}