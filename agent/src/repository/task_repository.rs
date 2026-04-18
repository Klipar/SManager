use shared::db::models::Task;

use crate::enums::task_errors::TaskError;
use sqlx::postgres::PgPool;
use std::sync::Arc;


pub struct TaskRepository {
    pool: Arc<PgPool>,
}

impl TaskRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Task, TaskError> {
        let task = sqlx::query_as::<_, Task>(
                "SELECT * FROM tasks WHERE id=$1"
            )
            .bind(id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|_| TaskError::DatabaseError)?;

        task.ok_or(TaskError::TaskNotFound)
    }

    pub async fn update_task(&self, task: Task) -> Result<Task, TaskError> {
        let updated = sqlx::query_as::<_, Task>(
            r#"
            UPDATE tasks
            SET
                name = $1,
                description = $2,
                install_script = $3,
                run_script = $4,
                delete_script = $5,
                restart_policy = $6
            WHERE id = $7
            RETURNING
                id, core_id, name, description,
                install_script, run_script, delete_script,
                restart_policy, status
            "#
        )
        .bind(&task.name)
        .bind(&task.description)
        .bind(&task.install_script)
        .bind(&task.run_script)
        .bind(&task.delete_script)
        .bind(task.restart_policy)
        .bind(task.id)
        .fetch_optional(&*self.pool)
        .await;

        match updated {
            Ok(Some(task)) => Ok(task),
            Ok(None) => Err(TaskError::TaskNotFound),
            Err(_) => Err(TaskError::DatabaseError),
        }
    }
}