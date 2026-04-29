use crate::{enums::{task_errors::TaskError}, managers::{managed_task::ManagedTask, token_manager::{TokenManager}}, repository::task_repository::TaskRepository};
use shared::{db::models::{Task, TaskStatus}, enums::script_types::ScriptType, server::endpoint::Endpoint};
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::fs;
use std::path::PathBuf;
use tokio::sync::Mutex;



use dashmap::DashMap;

use log::{error};

pub struct TaskManager {
    pool: Arc<PgPool>,
    tasks: Arc<DashMap<i64, ManagedTask>>, // i64 -> run id
    token_manager: Arc<Mutex<TokenManager>>,
    endpoint: Arc<Endpoint>
}

impl TaskManager {
    pub fn new(pool: Arc<PgPool>, endpoint: Arc<Endpoint>) -> Self {
        Self {
            pool: pool,
            tasks: Arc::new(DashMap::new()),
            token_manager: Arc::new(Mutex::new(TokenManager::new())),
            endpoint
        }
    }

    pub async fn run_task(self: Arc<Self>, task_id: i64, scrypt_type: ScriptType) -> Result<(), TaskError> {
        let rask_repository = TaskRepository::new(self.pool.clone());

        let mut task = rask_repository.get_by_id(task_id).await?;

        if matches!(task.status, TaskStatus::Ok | TaskStatus::Starting) {
            return Err(TaskError::TaskAlreadyRunning);
        }

        task.status = TaskStatus::Starting;

        task = rask_repository.update_task(task).await?;

        let run_id = TaskManager::create_run_record(self.pool.clone(), &task, scrypt_type).await
            .map_err(|_| TaskError::DatabaseError)?;

        let script_path = self.prepare_dir(&task, scrypt_type)
            .await
            .map_err(|e| TaskError::FailedToPrepareEnvironment(e.to_string()))?;

        self.tasks.insert(
            run_id,
            ManagedTask::new(
                script_path,
                self.clone(),
                run_id,
                {
                    let mut tm = self.token_manager.lock().await;
                    tm.gen_token(task_id)
                },
                self.endpoint.clone()
            ).await
            .map_err(|_e| TaskError::FailedToRunTask)?
        );

        return Ok(());
    }

    pub async fn handle_stdout(&self, run_id: i64, line: &str) {
        // TODO: notice if need that task is finished
        TaskManager::write_std_to_db(&self, run_id, line, "STDOUT").await;
    }

    pub async fn handle_stderr(&self, run_id: i64, line: &str) {
        // TODO: notice if need that task is finished
        TaskManager::write_std_to_db(&self, run_id, line, "STDERR").await;
    }

    async fn write_std_to_db(&self, run_id: i64, line: &str, test_type: &str) {
        let res = sqlx::query!(
            r#"
            UPDATE runs
            SET output = COALESCE(output, '') || $1 || E'\n'
            WHERE id = $2
            "#,
            format!("[{}] {}", test_type, line),
            run_id
        )
        .execute(&*self.pool)
        .await;

        if let Err(e) = res { error!("[MANAGER STDOUT DB ERROR] {}", e) }
    }

    pub async fn stop_task(self: Arc<Self>, task_id: i64) -> Result<(), TaskError> {
        let task_repository = TaskRepository::new(self.pool.clone());

        let mut task = task_repository.get_by_id(task_id).await?;
        if matches!(task.status, TaskStatus::Stopped | TaskStatus::Executed | TaskStatus::Failed) {
            return Err(TaskError::TaskAlreadyStopped);
        }

        let run_id = sqlx::query_scalar!(
            r#"
            SELECT id FROM runs
            WHERE task_id = $1
            ORDER BY id DESC
            LIMIT 1
            "#,
            task_id as i32
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|_| TaskError::FailedToManageRun)?
        .ok_or(TaskError::FailedToManageRun)?;

        let managed_task_pid = self.tasks.remove(&run_id)
            .ok_or(TaskError::TaskAlreadyStopped)?.1.pid;

        nix::sys::signal::kill(
            nix::unistd::Pid::from_raw(managed_task_pid as i32),
            nix::sys::signal::Signal::SIGTERM
        ).ok();

        task.status = TaskStatus::Stopped;
        task_repository.update_task(task).await?;

        Ok(())
    }

    pub async fn handle_exit(&self, run_id: i64, code: i32) {
        // TODO: notice if need that task is finished
        self.tasks.remove(&run_id);

        let res = sqlx::query!(
            r#"
            UPDATE runs
            SET end_time = NOW(),
                return_code = $1
            WHERE id = $2
            RETURNING task_id
            "#,
            code,
            run_id
        )
        .fetch_one(&*self.pool)
        .await;

        match res { // updating task status after finishing
            Ok(row) => {
                let task_repository = TaskRepository::new(self.pool.clone());
                let task = task_repository.get_by_id(row.task_id as i64).await;
                match task {
                    Ok(mut task) => {
                        if matches!(task.status, TaskStatus::Stopped) {
                            return;
                        }

                        task.status = match code {
                            0 => TaskStatus::Executed,
                            _ => TaskStatus::Failed,
                        };
                        let task = task_repository.update_task(task).await;
                        match task {
                            Ok(..) => {},
                            Err(e) => { error!("[MANAGER EXIT DB ERROR] {}", e) }
                        }
                    },
                    Err(e) => { error!("[MANAGER EXIT DB ERROR] {}", e) }
                }
            },
            Err(e) => { error!("[MANAGER EXIT DB ERROR] {}", e) }
        }
    }

    async fn create_run_record(
        pool: Arc<PgPool>,
        task: &Task,
        script_type: ScriptType,
    ) -> Result<i64, sqlx::Error> {

        let rec = sqlx::query!(
            r#"
            INSERT INTO runs (task_id, core_id, script)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            task.id,
            task.core_id,
            script_type as ScriptType
        )
        .fetch_one(&*pool)
        .await?;

        Ok(rec.id)
    }

    async fn prepare_dir(&self, task: &Task, scrypt_type: ScriptType) -> std::io::Result<PathBuf>{
        let mut path = PathBuf::from("tasks_storage/data");
        path.push(task.id.to_string());

        fs::create_dir_all(path.clone()).await?;

        path.push(scrypt_type.file_name());

        match scrypt_type.get_script(&task) {
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

    pub async fn token_to_task_id(self: Arc<Self>, token: &String) -> Option<i64>{
        let mut tm = self.token_manager.lock().await;
        tm.use_token(token)
    }
}