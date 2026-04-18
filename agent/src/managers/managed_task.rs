use shared::db::models::Task;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::process::{Command, Child};
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;
use tokio::sync::Mutex;

use crate::managers::task_manager::TaskManager;

pub struct ManagedTask {
    child: Arc<Mutex<Child>>,
}

impl ManagedTask {
    pub async fn new(
        task: Task,
        script_path: PathBuf,
        manager: Arc<TaskManager>,
    ) -> anyhow::Result<Self> {

        let script_dir = script_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot get parent directory of script"))?;

        let file_name = script_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid script path"))?;

        let mut child = Command::new("bash")
            .arg(&file_name)
            .current_dir(script_dir) // TODO: /work with real data for scrypt
            .env("AGENT_IP", "127.0.0.1")
            .env("AGENT_PORT", "6969")
            .env("TOKEN", "NOO it is secret token....")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture stdout"))?;

        let stderr = child.stderr.take()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture stderr"))?;

        let task = Arc::new(task);
        let child = Arc::new(Mutex::new(child));

        let managed = Self { child: child.clone() };

        Self::listen_stdout(task.clone(), stdout, manager.clone());
        Self::listen_stderr(task.clone(), stderr, manager.clone());
        Self::listen_exit(task.clone(), child.clone(), manager.clone());

        Ok(managed)
    }

    fn listen_stdout(
        task: Arc<Task>,
        stdout: tokio::process::ChildStdout,
        manager: Arc<TaskManager>,
    ) {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();

            while let Ok(Some(line)) = reader.next_line().await {
                manager.handle_stdout(&task, &line).await;
            }
        });
    }

    fn listen_stderr(
        task: Arc<Task>,
        stderr: tokio::process::ChildStderr,
        manager: Arc<TaskManager>,
    ) {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();

            while let Ok(Some(line)) = reader.next_line().await {
                manager.handle_stderr(&task, &line).await;
            }
        });
    }

    fn listen_exit(
        task: Arc<Task>,
        child: Arc<Mutex<Child>>,
        manager: Arc<TaskManager>,
    ) {
        tokio::spawn(async move {
            let status = child.lock().await.wait().await;

            match status {
                Ok(status) => {
                    let code = status.code().unwrap_or(-1);
                    manager.handle_exit(&task, code).await;
                }
                Err(e) => {
                    eprintln!("Failed to wait process: {}", e);
                }
            }
        });
    }

    pub async fn kill(&self) {
        let mut child = self.child.lock().await;
        let _ = child.kill().await;
    }
}