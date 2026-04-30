use async_trait::async_trait;
use serde_json::Value;
use shared::server::{
    connection_context::ConnectionContext,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use log::info;

use crate::extern_server::connection_registry::ConnectionRegistry;


pub struct StartStreamHandler {
    registry: ConnectionRegistry,
}

impl StartStreamHandler {
    pub fn new(registry: ConnectionRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl HandlerTrait for StartStreamHandler {
    async fn handle(
        &self,
        _data: Option<Value>,
        ctx: &mut ConnectionContext,
    ) -> Message {
        info!("Client joined execution_stream");

        let core_id =
        match ctx.id {
            Some(id) => id,
            None => {
                return Message::new_response(
                    Status::Error,
                    None,
                    401,
                    "Not authenticated!...",
                );
            }
        };

        self.registry
            .join_group(core_id, "execution_stream")
            .await;

        Message::new_response(
            Status::Ok,
            None,
            200,
            "Joined execution_stream",
        )
    }
}