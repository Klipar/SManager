use std::path::PathBuf;
use std::sync::Arc;
use shared::server::endpoint::Endpoint;
use tokio::process::{Command, Child};
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;
use tokio::sync::Mutex;
use crate::managers::task_manager::TaskManager;

pub struct ManagedTask {
    pub pid: u32,
}

impl ManagedTask {
    pub async fn new(
        script_path: PathBuf,
        manager: Arc<TaskManager>,
        run_id: i64,
        token: String,
        endpoint: Arc<Endpoint>
    ) -> anyhow::Result<Self> {
        let script_dir = script_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Cannot get parent directory of script"))?;
        let file_name = script_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid script path"))?;

        let mut child = Command::new("bash")
            .arg(&file_name)
            .current_dir(script_dir)
            .env("AGENT_IP", &endpoint.ip)
            .env("AGENT_PORT", endpoint.port.to_string())
            .env("TOKEN", token)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture stdout"))?;
        let stderr = child.stderr.take()
            .ok_or_else(|| anyhow::anyhow!("Failed to capture stderr"))?;

        let pid = child.id()
            .ok_or_else(|| anyhow::anyhow!("Process has no PID"))?;

        let child = Arc::new(Mutex::new(child));

        Self::listen_stdout(stdout, manager.clone(), run_id);
        Self::listen_stderr(stderr, manager.clone(), run_id);
        Self::listen_exit(child.clone(), manager.clone(), run_id);

        Ok(Self { pid })
    }

    fn listen_stdout(stdout: tokio::process::ChildStdout, manager: Arc<TaskManager>, run_id: i64) {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                manager.handle_stdout(run_id, &line).await;
            }
        });
    }

    fn listen_stderr(stderr: tokio::process::ChildStderr, manager: Arc<TaskManager>, run_id: i64) {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                manager.handle_stderr(run_id, &line).await;
            }
        });
    }

    fn listen_exit(child: Arc<Mutex<Child>>, manager: Arc<TaskManager>, run_id: i64) {
        tokio::spawn(async move {
            let status = child.lock().await.wait().await;
            match status {
                Ok(status) => {
                    let code = status.code().unwrap_or(-1);
                    manager.handle_exit(run_id, code).await;
                }
                Err(e) => {
                    eprintln!("Failed to wait process: {}", e);
                }
            }
        });
    }

    pub async fn kill(&self) {
    let _ = tokio::process::Command::new("kill")
            .arg(self.pid.to_string())
            .status()
            .await;
    }
}