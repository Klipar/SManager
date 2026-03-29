use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Run {
    pub id: i32,
    pub task_id: i32,
    pub core_id: i32,

    pub script: String,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,

    pub return_code: Option<i32>,
    pub output: Option<String>,
}