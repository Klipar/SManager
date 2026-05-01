use async_trait::async_trait;
use serde_json::Value;
use shared::server::{
    connection_context::ConnectionContext,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use log::info;

use crate::extern_server::connection_registry::ConnectionRegistry;


pub struct StopStreamHandler {
    registry: ConnectionRegistry,
}

impl StopStreamHandler {
    pub fn new(registry: ConnectionRegistry) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl HandlerTrait for StopStreamHandler {
    async fn handle(
        &self,
        _data: Option<Value>,
        ctx: &mut ConnectionContext,
    ) -> Message {

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

        match self.registry.leave_group(core_id, "execution_stream").await{
            Ok(..) => {
                info!("Client left execution_stream");
                Message::new_response(
                    Status::Ok,
                    None,
                    200,
                    "Left execution_stream",
                )
            },
            Err(..) => {
                Message::new_response(
                    Status::Error,
                    None,
                    409,
                    "Client already left execution_stream",
                )
            }
        }
    }
}