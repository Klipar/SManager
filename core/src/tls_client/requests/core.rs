use anyhow::{bail, Result};
use serde::Serialize;
use crate::tls_client::connection_manager::ConnectionManager;
use shared::server::message::{Message, Status};
use shared::server::dto::create_core_dto::CreateCoreDto;
use shared::server::dto::get_cores_dto::CoresDTO;



pub async fn new_core(
    manager: &ConnectionManager,
    req: CreateCoreDto,
) -> Result<()> {
    let data = serde_json::to_value(&req)?;
    let response = manager.request("new-core", Some(data)).await?;

    match response {
        Message::Response { status: Status::Ok, .. } => Ok(()),
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("new-core failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}
pub async fn get_all_cores(manager: &ConnectionManager) -> Result<Vec<CoresDTO>> {
    let response = manager.request("get-all-cores", None).await?;

    match response {
        Message::Response { status: Status::Ok, data: Some(data), .. } => {
            let cores = data["cores"].clone();
            Ok(serde_json::from_value(cores)?)
        }
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("get-all-cores failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}
pub async fn update_core(
    manager: &ConnectionManager,
    req: CoresDTO,
) -> Result<CoresDTO> {
    let data = serde_json::to_value(&req)?;
    let response = manager.request("update-core", Some(data)).await?;

    match response {
        Message::Response { status: Status::Ok, data: Some(data), .. } => {
            let core = data["core"].clone();
            Ok(serde_json::from_value(core)?)
        }
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("update-core failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}
pub async fn remove_core(
    manager: &ConnectionManager,
    id: i64,
) -> Result<()> {
    let data = serde_json::json!({ "id": id });
    let response = manager.request("remove-core", Some(data)).await?;

    match response {
        Message::Response { status: Status::Ok, .. } => Ok(()),
        Message::Response { status: Status::Error, message, code, .. } => {
            bail!("remove-core failed [{code}]: {message}")
        }
        _ => bail!("Unexpected response format"),
    }
}