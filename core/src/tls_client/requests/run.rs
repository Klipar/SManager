use anyhow::{bail, Result};
use crate::tls_client::connection_manager::ConnectionManager;
use shared::server::message::{Message, Status};
use shared::db::models::run::Run;
pub async fn get_runs(manager: &ConnectionManager) -> Result<Vec<Run>> {
    let response = manager.request("get-runs", None).await?;

    match response {
        Message::Response {
            status: Status::Ok,
            data: Some(data),
            ..
        } => {
            let runs = data["runs"].clone();
            Ok(serde_json::from_value(runs)?)
        }

        Message::Response {
            status: Status::Error,
            message,
            code,
            ..
        } => {
            bail!("get-runs failed [{code}]: {message}")
        }

        _ => bail!("Unexpected response format"),
    }
}
pub async fn delete_run(
    manager: &ConnectionManager,
    id: i64,
) -> Result<()> {
    let data = serde_json::json!({ "id": id });
    let response = manager.request("delete-run", Some(data)).await?;

    match response {
        Message::Response { status: Status::Ok, .. } => Ok(()),
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("delete-run failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}
pub async fn delete_inactive_runs(manager: &ConnectionManager, task_id: i64) -> Result<i64> {
    let data = serde_json::json!({ "id": task_id });
    let response = manager.request("delete-inactive-runs", Some(data)).await?;

    match response {
        Message::Response {
            status: Status::Ok,
            data: Some(data),
            ..
        } => {
            let deleted_count = data["deleted_count"]
                .as_i64()
                .ok_or_else(|| anyhow::anyhow!("Missing deleted_count field"))?;

            Ok(deleted_count)
        }

        Message::Response {
            status: Status::Error,
            message,
            code,
            ..
        } => {
            bail!("delete-inactive-runs failed [{code}]: {message}")
        }

        _ => bail!("Unexpected response format"),
    }
}