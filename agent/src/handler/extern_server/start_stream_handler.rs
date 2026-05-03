use std::sync::Arc;
use sqlx::postgres::PgPool;
use async_trait::async_trait;
use serde_json::{Value, json};
use shared::{db::models::Run, server::{
    connection_context::ConnectionContext,
    handler_trait::HandlerTrait,
    message::{Message, Status},
}};
use log::{info, error};

use crate::extern_server::connection_registry::ConnectionRegistry;


pub struct StartStreamHandler {
    registry: ConnectionRegistry,
    pool: Arc<PgPool>,

}

impl StartStreamHandler {
    pub fn new(registry: ConnectionRegistry, pool: Arc<PgPool>) -> Self {
        Self { registry, pool }
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

        match self.registry.join_group(core_id, "execution_stream").await {
            Ok(..) => {
                let runs = match sqlx::query_as::<_, Run>("SELECT * FROM runs")
                    .fetch_all(&*self.pool)
                    .await
                {
                    Ok(runs) => runs,
                    Err(e) => {
                        error!("{}",e);
                        vec![]
                    },
                };

                Message::new_response(
                    Status::Ok,
                    Some(json!({"runs" : runs})),
                    200,
                    "Joined execution_stream",
                )
            },
            Err(..) => {
                Message::new_response(
                    Status::Error,
                    None,
                    208,
                    "Already joined execution_stream",
                )
            }
        }
    }
}