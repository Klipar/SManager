use anyhow::{bail, Result};
use log::error;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use crate::tls_client::connection_manager::ConnectionManager;
use shared::server::message::{Message, Status};
use shared::server::dto::new_task_request_dto::NewTaskRequestDTO;
use shared::server::dto::run_task_dto::RunTaskDTO;
use shared::db::models::enums::RestartPolicy;

use shared::db::models::run::Run;

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
    req: NewTaskRequestDTO,
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
pub async fn run_task(
    manager: &ConnectionManager,
    req: RunTaskDTO,
) -> Result<()> {
    let data = serde_json::to_value(&req)?;
    let response = manager.request("run-task", Some(data)).await?;

    match response {
        Message::Response { status: Status::Ok, .. } => Ok(()),
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("run-task failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}
pub async fn start_stream(manager: &ConnectionManager) -> Result<mpsc::Receiver<Run>> {
    let mut raw_rx = manager.subscribe_push().await;
    let response = manager.request("start-stream", None).await?;
    match response {
        Message::Response { status: Status::Ok, .. } => {}
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("start-stream failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }

    let (tx, rx) = mpsc::channel::<Run>(64);

    tokio::spawn(async move {
        while let Some(msg) = raw_rx.recv().await {
            if let Message::Request { data: Some(data), .. } = msg {
                
                match serde_json::from_value::<Vec<Run>>(data) {
                    Ok(runs) => {
                        for run in runs {
                            if tx.send(run).await.is_err() {
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        error!("[stream] Failed to parse execution_stream: {}", e);
                    }
                }
            }
        }
    });

    Ok(rx)
}