use serde::{Deserialize, Serialize};

use crate::db::models::RestartPolicy;

#[derive(Deserialize, Serialize)]
pub struct UpdateTaskDTO {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub install_script: Option<String>,
    pub run_script: Option<String>,
    pub delete_script: Option<String>,
    pub restart_policy: RestartPolicy
}