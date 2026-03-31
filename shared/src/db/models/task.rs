use serde::{Serialize, Deserialize};
use sqlx::FromRow;

use super::enums::{RestartPolicy, TaskStatus};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub core_id: Option<i32>,

    pub name: String,
    pub description: Option<String>,

    pub install_script: Option<String>,
    pub run_script: Option<String>,
    pub delete_script: Option<String>,

    pub restart_policy: RestartPolicy,
    pub status: TaskStatus,

    pub token_hash: String,
}