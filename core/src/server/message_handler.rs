use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use log::error;
use serde_json::Value;
use shared::server::message::{Message, Status};
use crate::{
    handler::handler_trait::HandlerTrait,
    server::connection_context::ConnectionContext,
    server::auth::{extract_token, verify_token, is_action_allowed},
};

pub async fn process_message(
    message: Message,
    handlers: &Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    ctx: &mut ConnectionContext,
    addr: SocketAddr,
) -> Message {
    match message {
        Message::Request { id, action, data } => {
            let response = match action.as_str() {
                "authenticate" => handle_authenticate_request(id, &data, ctx).await,
                "login" => handle_login_request(id, action, data, handlers, ctx).await,
                _ => handle_regular_request(id, action, data, handlers, ctx).await,
            };
            response
        }
        Message::Response { .. } => {
            error!("Received response message from {}", addr);
            Message::new_response(
                Status::Error,
                None,
                400,
                "Response messages are not accepted on this socket",
            )
        }
    }
}

async fn handle_authenticate_request(
    id: u64,
    data: &Option<Value>,
    ctx: &mut ConnectionContext,
) -> Message {
    let token = match extract_token(data) {
        Some(token) => token,
        None => {
            let mut response = Message::new_response(
                Status::Error,
                None,
                401,
                "Missing token",
            );
            response.set_id(id);
            return response;
        }
    };

    match verify_token(&token) {
        Ok(claims) => {
            ctx.user_id = Some(claims.sub);
            ctx.is_admin = claims.is_admin;
            ctx.id = Some(claims.sub);

            log::info!("User authenticated: id={}, is_admin={}", claims.sub, claims.is_admin);

            let mut response = Message::new_response(
                Status::Ok,
                Some(serde_json::json!({
                    "user_id": claims.sub,
                    "is_admin": claims.is_admin
                })),
                200,
                "Authenticated",
            );
            response.set_id(id);
            response
        }
        Err(e) => {
            error!("Token verification failed: {}", e);
            let mut response = Message::new_response(
                Status::Error,
                None,
                401,
                "Invalid token",
            );
            response.set_id(id);
            response
        }
    }
}

async fn handle_login_request(
    id: u64,
    action: String,
    data: Option<Value>,
    handlers: &Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    ctx: &mut ConnectionContext,
) -> Message {
    let Some(handler) = handlers.get(&action) else {
        error!("Unknown action: {}", action);
        let mut response = Message::new_response(
            Status::Error,
            None,
            404,
            format!("Unknown action: {}", action),
        );
        response.set_id(id);
        return response;
    };

    let mut response = handler.handle(data, ctx).await;
    response.set_id(id);
    response
}

async fn handle_regular_request(
    id: u64,
    action: String,
    data: Option<Value>,
    handlers: &Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    ctx: &mut ConnectionContext,
) -> Message {
    if ctx.user_id.is_none() {
        let mut response = Message::new_response(
            Status::Error,
            None,
            401,
            "Not authenticated. Send authenticate first",
        );
        response.set_id(id);
        return response;
    }

    if !is_action_allowed(&action, &data, ctx) {
        let mut response = Message::new_response(
            Status::Error,
            None,
            403,
            "Forbidden",
        );
        response.set_id(id);
        return response;
    }

    let Some(handler) = handlers.get(&action) else {
        error!("Unknown action: {}", action);
        let mut response = Message::new_response(
            Status::Error,
            None,
            404,
            format!("Unknown action: {}", action),
        );
        response.set_id(id);
        return response;
    };

    let mut response = handler.handle(data, ctx).await;
    response.set_id(id);
    response
}