use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::db::models::AgentStatus;

#[derive(Deserialize, Serialize)]
pub struct AgentResponseDto {
    pub id: i32,
    pub ip: String,
    pub port: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: AgentStatus,
    pub last_connection: Option<NaiveDateTime>,
    pub last_message: Option<NaiveDateTime>,
}