use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "restart_policy", rename_all = "lowercase")]
pub enum RestartPolicy {
    No,
    Always,
    OnFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Ok,
    Starting,
    Failed,
    Stopped,
    Executed,
}
