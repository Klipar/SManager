use async_trait::async_trait;
use serde_json::Value;
use shared::{server::{connection_context::ConnectionContext, dto::run_task_dto::RunTaskDTO, handler_trait::HandlerTrait, message::{Message, Status}}};
use std::sync::Arc;
use serde_json::json;

use log::{error};

use crate::managers::task_manager::TaskManager;

pub struct RunTaskHandler {
    task_manager: Arc<TaskManager>,
}

impl RunTaskHandler {
    pub fn new(task_manager: Arc<TaskManager>) -> Self {
        Self {task_manager }
    }
}

#[async_trait]
impl HandlerTrait for RunTaskHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext)-> Message {
        let data = match data {
            Some(v) => v,
            None => {
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Missing data"
                );
            }
        };

        let dto: RunTaskDTO = match serde_json::from_value(data) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to parse run task request: {}", e);
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid run-task request"
                );
            }
        };

        let result = TaskManager::run_task(self.task_manager.clone(), dto.task_id, dto.script_type).await;

        match result{
            Ok(..) =>{
                return Message::new_response (
                    Status::Ok,
                    None,
                    200,
                    "Successfully started task."
                );
            },
            Err(error) =>{
                return Message::new_response (
                    Status::Error,
                    Some(json!({"error" : error})),
                    400,
                    "Failed to start tasks"
                );
            }
        }
    }
}