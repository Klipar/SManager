use agent_lib::{extern_server::{connection_registry::ConnectionRegistry, server::Server}, handler::{extern_server::{get_all_cores_handler::GetAllCoresHandler, get_all_tasks_handler::GetAllTasksHandler, new_core_handler::NewCoreHandler, new_task_handler::NewTaskHandler, ping_handler::PingHandler, remove_core_handler::RemoveCoreHandler, remove_task_handler::RemoveTaskHandler, run_task_handler::RunTaskHandler, start_stream_handler::StartStreamHandler, stop_task_handler::StopTaskHandler, update_core_handler::UpdateCoreHandler, update_task_handler::UpdateTaskHandler}, intern_server::authenticate_handler::AuthenticateHandler}, managers::task_manager::TaskManager};
use shared::server::endpoint::Endpoint;
use sqlx::postgres::PgPool;

use log::error;
use dotenvy::dotenv;
use std::sync::Arc;
use agent_lib::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let cfg = Config::from_env();

    let shared_pool = Arc::new(PgPool::connect(&cfg.database_url).await?);

    let intern_endpoint = Arc::new(Endpoint::new("127.0.0.1", cfg.intern_port));
    let extern_endpoint = Arc::new(Endpoint::new(cfg.extern_ip, cfg.extern_port));
    let connection_registry = ConnectionRegistry::default();

    let task_manager = Arc::new(TaskManager::new(shared_pool.clone(), intern_endpoint.clone(), connection_registry.clone()));

    let mut intern_server = agent_lib::intern_server::server::Server::new(intern_endpoint.clone());

    intern_server.add_handler("authenticate", Arc::new(AuthenticateHandler::new(shared_pool.clone(), task_manager.clone())));

    let intern_handle = tokio::spawn(async move {
        if let Err(e) = intern_server.start_server().await {
            error!("[INTERN SERVER ERROR] {}", e);
        }
    });

    let mut extern_server = Server::new(extern_endpoint.clone(), shared_pool.clone(), connection_registry.clone());

    // CRUD for Cores
    extern_server.add_handler("new-core", Arc::new(NewCoreHandler::new(shared_pool.clone())));
    extern_server.add_handler("get-all-cores", Arc::new(GetAllCoresHandler::new(shared_pool.clone())));
    extern_server.add_handler("update-core", Arc::new(UpdateCoreHandler::new(shared_pool.clone())));
    extern_server.add_handler("remove-core", Arc::new(RemoveCoreHandler::new(shared_pool.clone())));

    // CRUD for Tasks
    extern_server.add_handler("new-task", Arc::new(NewTaskHandler::new(shared_pool.clone())));
    extern_server.add_handler("get-all-tasks", Arc::new(GetAllTasksHandler::new(shared_pool.clone())));
    extern_server.add_handler("update-task", Arc::new(UpdateTaskHandler::new(shared_pool.clone())));
    extern_server.add_handler("remove-task", Arc::new(RemoveTaskHandler::new(shared_pool.clone())));

    // Task Operation
    extern_server.add_handler("run-task", Arc::new(RunTaskHandler::new(task_manager.clone())));
    extern_server.add_handler("stop-task", Arc::new(StopTaskHandler::new(task_manager.clone())));

    // Ping - Pong
    extern_server.add_handler("ping", Arc::new(PingHandler::new()));

    // register connection
    extern_server.add_handler("start-stream", Arc::new(StartStreamHandler::new(connection_registry.clone())));

    let extern_handle = tokio::spawn(async move {
        if let Err(e) = extern_server.start_server().await {
            error!("[EXTERN SERVER ERROR] {}", e);
        }
    });

    tokio::try_join!(intern_handle, extern_handle)?;

    Ok(())
}