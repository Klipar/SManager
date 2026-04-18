use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Log {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub message: String,
    pub agent_id: Option<i32>,
    pub task_id: Option<i32>,
    pub user_id: Option<i32>,
}