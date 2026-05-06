use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use crate::tls_client::connection_manager::ConnectionManager;
use shared::server::message::{Message, Status};
#[derive(Serialize)]
pub struct NewTaskRequest<'a> {
    pub name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    pub install_script: Option<&'a str>,
    pub run_script: Option<&'a str>,
    pub delete_script: Option<&'a str>,
    pub restart_policy: RestartPolicy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum RestartPolicy {
    No,
    Always,
    OnFailure,
}

// ---- Response data ----

#[derive(Deserialize, Debug)]
pub struct AgentTask {
    pub id: u64,
    pub core_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub install_script: Option<String>,
    pub run_script: Option<String>,
    pub delete_script: Option<String>,
    pub restart_policy: RestartPolicy,
    pub status: String,
}

// ---- Actions ----

pub async fn new_task(
    manager: &ConnectionManager,
    req: NewTaskRequest<'_>,
) -> Result<AgentTask> {
    let data = serde_json::to_value(&req)?;
    let response = manager.request("new-task", Some(data)).await?;

    match response {
        Message::Response { status: Status::Ok, data: Some(data), .. } => {
            let task = data["task"].clone();
            Ok(serde_json::from_value(task)?)
        }
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("new-task failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}