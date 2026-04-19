use async_trait::async_trait;
use serde_json::Value;
use shared::{server::message::{Message, Status}};
use sqlx::postgres::PgPool;
use std::sync::Arc;

use log::{info, warn, error};

use crate::{intern_server::{connection_context::ConnectionContext, handler_trait::HandlerTrait}, managers::task_manager::TaskManager};

pub struct AuthenticateHandler {
    pub pool: Arc<PgPool>,
    pub task_manager: Arc<TaskManager>,
}

impl AuthenticateHandler {
    pub fn new(pool: Arc<PgPool>, task_manager: Arc<TaskManager>) -> Self {
        Self { pool, task_manager }
    }
}

#[async_trait]
impl HandlerTrait for AuthenticateHandler {
    async fn handle(&self, data: Option<Value>, ctx: &mut ConnectionContext) -> Message {
        if ctx.authenticated{
            error!("Received authenticate request for already authenticated socket...");
            return Message::new_response (
                Status::Error,
                None,
                401,
                "Double authorization",
            );
        }

        info!("Received authenticate request");

        match data {
            Some(data) => {
                if let Some(token) = data.get("token").and_then(|v| v.as_str()) {
                    let task_id = TaskManager::token_to_task_id(self.task_manager.clone(), &token.to_string()).await;

                    match task_id {
                        Some(task_id) => {
                            info!("Successful authentication for task id: `{}`", task_id);
                            ctx.authenticated = true;
                            ctx.id = Some(task_id);
                            return Message::new_response (
                                Status::Ok,
                                None,
                                200,
                                "Authorized successfully!"
                            );
                        }
                        None => {
                            warn!("Failed to authentication task using token: {}", token);

                            return Message::new_response (
                                Status::Error,
                                None,
                                401,
                                "Failed to authenticate. Your token invalid."
                            );
                        }
                    }
                } else {
                    return Message::new_response (
                        Status::Error,
                        None,
                        400,
                        "Invalid auth request, field 'data.token' is not exist or not string.");
                }
            },
            None => {
                return Message::new_response (
                    Status::Error,
                    None,
                    400,
                    "Invalid auth request, field 'data' is not exist."
                );
            },
        }
    }
}