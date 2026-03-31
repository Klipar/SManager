use serde::{Deserialize, Serialize};

use crate::db::models::RestartPolicy;

#[derive(Deserialize, Serialize)]
pub struct NewTaskRequestDTO {
    pub name: String,
    pub description: String,
    pub install_script: String,
    pub run_script: String,
    pub delete_script: String,
    pub restart_policy: RestartPolicy,
}