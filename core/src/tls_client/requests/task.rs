use anyhow::{bail, Result};
use log::error;
use serde::Deserialize;
use tokio::sync::mpsc;
use crate::tls_client::connection_manager::ConnectionManager;
use shared::server::message::{Message, Status};
use shared::server::dto::new_task_request_dto::NewTaskRequestDTO;
use shared::server::dto::run_task_dto::RunTaskDTO;
use shared::db::models::enums::RestartPolicy;
use shared::db::models::task::Task;
use shared::db::models::run::Run;


// ---- Actions ----

pub async fn new_task(
    manager: &ConnectionManager,
    req: NewTaskRequestDTO,
) -> Result<Task> {
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
pub async fn get_all_tasks(manager: &ConnectionManager) -> Result<Vec<Task>> {
    let response = manager.request("get-all-tasks", None).await?;

    match response {
        Message::Response { status: Status::Ok, data: Some(data), .. } => {
            let tasks = data["tasks"].clone();
            Ok(serde_json::from_value(tasks)?)
        }
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("get-all-tasks failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}
pub async fn update_task(
    manager: &ConnectionManager,
    req: Task,
) -> Result<Task> {
    let data = serde_json::to_value(&req)?;
    let response = manager.request("update-task", Some(data)).await?;

    match response {
        Message::Response { status: Status::Ok, data: Some(data), .. } => {
            let task = data["task"].clone();
            Ok(serde_json::from_value(task)?)
        }
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("update-task failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}
pub async fn remove_task(
    manager: &ConnectionManager,
    id: i64,
) -> Result<()> {
    let data = serde_json::json!({ "id": id });
    let response = manager.request("remove-task", Some(data)).await?;

    match response {
        Message::Response { status: Status::Ok, .. } => Ok(()),
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("remove-task failed [{code}]: {message}")
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
        Message::Response { status: Status::Error, code: 208, .. } => {
            // Already joined — ok, pokračujeme
        }
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
pub async fn stop_task(
    manager: &ConnectionManager,
    task_id: i64,
) -> Result<()> {
    let data = serde_json::json!({ "task_id": task_id });
    let response = manager.request("stop-task", Some(data)).await?;

    match response {
        Message::Response { status: Status::Ok, .. } => Ok(()),
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("stop-task failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}
pub async fn stop_stream(manager: &ConnectionManager) -> Result<()> {
    let response = manager.request("stop-stream", None).await?;

    match response {
        Message::Response { status: Status::Ok, .. } => Ok(()),
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("stop-stream failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}
