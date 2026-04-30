use async_trait::async_trait;
use serde_json::Value;
use shared::server::{connection_context::ConnectionContext,
                    handler_trait::HandlerTrait, message::{Message, Status}};
use log::info;

pub struct PingHandler {}

impl PingHandler {
    pub fn new() -> Self {
        PingHandler {}
    }
}

#[async_trait]
impl HandlerTrait for PingHandler {
    async fn handle(&self, _data: Option<Value>, _ctx: &mut ConnectionContext)-> Message {
        info!("Received ping request");

        return Message::new_response (
            Status::Ok,
            None,
            200,
            "Pong!"
        );
    }
}