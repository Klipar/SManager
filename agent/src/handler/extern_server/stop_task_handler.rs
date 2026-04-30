use async_trait::async_trait;
use serde_json::Value;
use shared::{server::{connection_context::ConnectionContext, handler_trait::HandlerTrait, message::{Message, Status}}};
use std::sync::Arc;
use serde_json::json;

use log::{error};

use crate::managers::task_manager::TaskManager;

pub struct StopTaskHandler {
    task_manager: Arc<TaskManager>,
}

impl StopTaskHandler {
    pub fn new(task_manager: Arc<TaskManager>) -> Self {
        Self {task_manager }
    }
}

#[async_trait]
impl HandlerTrait for StopTaskHandler {
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

        match data.get("task_id").and_then(|v| v.as_i64()){
            Some(id) => {
                let result = TaskManager::stop_task(self.task_manager.clone(), id).await;
                match result{
                    Ok(..) =>{
                        return Message::new_response (
                            Status::Ok,
                            None,
                            200,
                            "Successfully stopped task."
                        );
                    },
                    Err(error) =>{
                        return Message::new_response (
                            Status::Error,
                            Some(json!({"error" : error})),
                            400,
                            "Failed to stop tasks"
                        );
                    }
                }
            },
            None => {
                error!("Failed to parse stop task request");
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid stop-task request"
                );
            }
        }
    }
}