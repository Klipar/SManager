use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize, Clone)]
#[serde(tag = "type")]
pub enum ExecutionStreamEvent {
    Stdout {
        run_id: i64,
        new_output: String,
    },
    Exit {
        run_id: i64,
        return_code: i32,
        end_time: DateTime<Utc>,
    },
}