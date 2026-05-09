use async_trait::async_trait;
use serde_json::Value;
use crate::{handler::handler_trait::HandlerTrait, server::connection_context::ConnectionContext};
use shared::server::{message::{Message, Status}, get_hash::get_hash};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use log::{error, info};

pub struct RemoveUserHandler {
	pub pool: Arc<PgPool>,
}

impl RemoveUserHandler {
	pub fn new(pool: Arc<PgPool>) -> Self {
		Self { pool }
	}
}

#[async_trait]
impl HandlerTrait for RemoveUserHandler {
	async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
		info!("Received request removing user");

		let data = match data {
			Some(v) => v,
			None => {
				return Message::new_response(
					Status::Error,
					None,
					400,
					"Missing data",
				);
			}
		};

		let id = match data.get("id").and_then(|v| v.as_i64()) {
			Some(id) => id as i32,
			None => {
				return Message::new_response(
					Status::Error,
					None,
					400,
					"Invalid remove-user request: missing id",
				);
			}
		};

		let is_self_deletion = _ctx.user_id == Some(id);
		let is_admin_deleting_other = _ctx.is_admin && !is_self_deletion;

		if !is_admin_deleting_other {
			let password = match data.get("password").and_then(|v| v.as_str()) {
				Some(p) => p.to_string(),
				None => {
					return Message::new_response(
						Status::Error,
						None,
						400,
						"Password is required",
					);
				}
			};

			let stored = sqlx::query!(
				r#"
				SELECT password
				FROM users
				WHERE id = $1
				"#,
				id
			)
			.fetch_optional(&*self.pool)
			.await;

			match stored {
				Ok(Some(row)) => {
					let stored_hash = row.password;
					let supplied_hash = get_hash(&password);
					if supplied_hash != stored_hash {
						return Message::new_response(
							Status::Error,
							None,
							401,
							"Invalid password",
						)
					}
				}
				Ok(None) => {
					return Message::new_response(
						Status::Error,
						None,
						404,
						"User not found",
					)
				}
				Err(e) => {
					error!("Failed to fetch user for deletion: {}", e);
					return Message::new_response(
						Status::Error,
						None,
						500,
						"Failed to verify password",
					)
				}
			}
		}

		let result = sqlx::query!(
			r#"
			DELETE FROM users
			WHERE id = $1
			"#,
			id
		)
		.execute(&*self.pool)
		.await;

		match result {
			Ok(result) => {
				if result.rows_affected() == 1 {
					Message::new_response(
						Status::Ok,
						None,
						200,
						format!("Successfully deleted user {}", id),
					)
				} else {
					Message::new_response(
						Status::Error,
						None,
						404,
						"User not found",
					)
				}
			}
			Err(e) => {
				error!("Failed to delete user: {}", e);
				Message::new_response(
					Status::Error,
					None,
					500,
					"Failed to delete user",
				)
			}
		}
	}
}
