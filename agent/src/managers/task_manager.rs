use crate::{enums::{script_types::ScriptTypes, task_errors::TaskError}, managers::{managed_task::ManagedTask}, repository::task_repository::TaskRepository};
use shared::db::models::{Task, TaskStatus};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::fs;
use std::path::PathBuf;

use dashmap::DashMap;

pub struct TaskManager {
    pub pool: Arc<PgPool>,
    pub tasks: Arc<DashMap<i32, ManagedTask>>,
}

impl TaskManager {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            pool: pool,
            tasks: Arc::new(DashMap::new())
        }
    }

    pub async fn run_task(self: Arc<Self>, task_id: i32, scrypt_type: ScriptTypes) -> Result<(), TaskError> {
        let rask_repository = TaskRepository::new(self.pool.clone());

        let mut task = rask_repository.get_by_id(task_id).await?;

        if matches!(task.status, TaskStatus::Ok | TaskStatus::Starting) {
            return Err(TaskError::TaskAlreadyRunning);
        }

        task.status = TaskStatus::Starting;

        task = rask_repository.update_task(task).await?;

        let script_path = self.prepare_dir(&task, scrypt_type)
            .await
            .map_err(|e| TaskError::FailedToPrepareEnvironment(e.to_string()))?;

        self.tasks.insert(
            task_id,
            ManagedTask::new(
                task,
                script_path,
                self.clone()
            ).await
            .map_err(|_e| TaskError::FailedToRunTask)?
        );

        return Ok(());
    }

    pub async fn handle_stdout(&self, task: &Task, line: &str) {
        println!("[MANAGER STDOUT] {}: {}", task.id, line);

        //TODO: implement this
    }

    pub async fn handle_stderr(&self, task: &Task, line: &str) {
        println!("[MANAGER STDERR] {}: {}", task.id, line);

        //TODO: implement this
    }

    pub async fn handle_exit(&self, task: &Task, code: i32) {
        println!("[MANAGER EXIT] {}: {}", task.id, code);
        self.tasks.remove(&task.id);
        //TODO: implement this
    }

    async fn prepare_dir(&self, task: &Task, scrypt_type: ScriptTypes) -> std::io::Result<PathBuf>{
        let mut path = PathBuf::from("tasks_storage/data");
        path.push(task.id.to_string());

        fs::create_dir_all(path.clone()).await?;

        path.push(scrypt_type.file_name());

        match scrypt_type.get_scrypt(&task) {
            Some(scrypt) => {
                fs::write(&path, scrypt).await?;
                return Ok(path);
            },
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "script not exist",
                ));
            }
        }
    }

    fn _sanitize_dir_name(self, name: &str) -> String {
        let forbidden = ['\\', '/', ':', '*', '?', '"', '<', '>', '|'];

        name.chars()
            .map(|c| if forbidden.contains(&c) || c.is_control() { '_' } else { c })
            .collect()
    }
}