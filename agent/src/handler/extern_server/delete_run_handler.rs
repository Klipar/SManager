use async_trait::async_trait;
use serde_json::Value;
use shared::server::{
    connection_context::ConnectionContext,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};
use sqlx::SqlitePool;
use std::sync::Arc;

use log::{error, info, warn};

pub struct DeleteRunByIdHandler {
    pub pool: Arc<SqlitePool>,
}

impl DeleteRunByIdHandler {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HandlerTrait for DeleteRunByIdHandler {
    async fn handle(&self, data: Option<Value>, _ctx: &mut ConnectionContext) -> Message {
        let run_id = match data {
            Some(Value::Object(obj)) => match obj.get("id") {
                Some(Value::Number(num)) => num.as_i64().ok_or_else(|| {
                    error!("Invalid ID format: not an integer");
                    "Invalid ID format"
                }),
                Some(Value::String(s)) => s.parse::<i64>().map_err(|_| {
                    error!("Invalid ID format: cannot parse string to integer");
                    "Invalid ID format"
                }),
                _ => {
                    error!("Missing or invalid 'id' field");
                    return Message::new_response(
                        Status::Error,
                        None,
                        400,
                        "Missing or invalid 'id' field. Please provide a valid run ID.",
                    );
                }
            },
            _ => {
                error!("Invalid request data format");
                return Message::new_response(
                    Status::Error,
                    None,
                    400,
                    "Invalid request data. Expected JSON object with 'id' field.",
                );
            }
        };

        let run_id = match run_id {
            Ok(id) => id,
            Err(err_msg) => {
                return Message::new_response(Status::Error, None, 400, err_msg);
            }
        };

        info!("Attempting to delete completed run with ID: {}", run_id);

        let check_result = sqlx::query!(
            r#"
            SELECT return_code, end_time
            FROM runs
            WHERE id = ?
            "#,
            run_id
        )
        .fetch_optional(&*self.pool)
        .await;

        match check_result {
            Ok(Some(run)) => {
                if run.return_code.is_some() && run.end_time.is_some() {
                    info!("Run {} is completed, proceeding with deletion", run_id);

                    let delete_result = sqlx::query!(
                        "DELETE FROM runs WHERE id = $1 AND return_code IS NOT NULL AND end_time IS NOT NULL",
                        run_id
                    )
                    .execute(&*self.pool)
                    .await;

                    match delete_result {
                        Ok(result) => {
                            let rows_affected = result.rows_affected();
                            if rows_affected > 0 {
                                info!("Successfully deleted completed run with ID: {}", run_id);
                                Message::new_response(
                                    Status::Ok,
                                    None,
                                    200,
                                    &format!(
                                        "Successfully deleted completed run with ID: {}",
                                        run_id
                                    ),
                                )
                            } else {
                                warn!("Run {} was not deleted (possibly changed status)", run_id);
                                Message::new_response(
                                    Status::Error,
                                    None,
                                    400,
                                    "Failed to delete run. The run may have been modified.",
                                )
                            }
                        }
                        Err(e) => {
                            error!("Failed to delete run {}: {}", run_id, e);
                            Message::new_response(
                                Status::Error,
                                None,
                                500,
                                &format!("Database error: {}", e),
                            )
                        }
                    }
                } else {
                    warn!(
                        "Run {} is not completed (return_code: {:?}, end_time: {:?}). Cannot delete.",
                        run_id, run.return_code, run.end_time
                    );
                    Message::new_response(
                        Status::Error,
                        None,
                        400,
                        &format!(
                            "Cannot delete run with ID: {}. Run is not completed yet.",
                            run_id
                        ),
                    )
                }
            }
            Ok(None) => {
                error!("Run with ID {} not found", run_id);
                Message::new_response(
                    Status::Error,
                    None,
                    404,
                    &format!("Run with ID: {} not found", run_id),
                )
            }
            Err(e) => {
                error!("Database error while checking run {}: {}", run_id, e);
                Message::new_response(Status::Error, None, 500, &format!("Database error: {}", e))
            }
        }
    }
}
