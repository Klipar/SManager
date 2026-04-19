use serde::{Serialize, Deserialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "restart_policy", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum RestartPolicy {
    No,
    Always,

    #[serde(rename = "on-failure")]
    #[sqlx(rename = "on-failure")]
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

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "agent_status", rename_all = "lowercase")]
pub enum AgentStatus {
    Online,
    Offline,
    Error,
}
