use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;

use super::enums::AgentStatus;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Agent {
    pub id: i32,
    pub ip: String,
    pub port: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: AgentStatus,
    pub last_connection: Option<NaiveDateTime>,
    pub last_message: Option<NaiveDateTime>,
}
