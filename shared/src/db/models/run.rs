use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

use crate::enums::script_types::ScriptType;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Run {
    pub id: i64,
    pub task_id: i32,
    pub core_id: Option<i32>,
    pub script: ScriptType,

    #[sqlx(rename = "start_time")]
    pub start_time: NaiveDateTime,

    #[sqlx(rename = "end_time")]
    pub end_time: Option<NaiveDateTime>,

    #[sqlx(rename = "return_code")]
    pub return_code: Option<i32>,
    pub output: Option<String>,
}