use agent_lib::{enums::script_types::ScriptType, extern_server::server::Server, handler::{extern_server::{get_all_cores_handler::GetAllCoresHandler, get_all_tasks_handler::GetAllTasksHandler, new_core_handler::NewCoreHandler, new_task_handler::NewTaskHandler, remove_core_handler::RemoveCoreHandler, remove_task_handler::RemoveTaskHandler, update_core_handler::UpdateCoreHandler, update_task_handler::UpdateTaskHandler}, intern_server::authenticate_handler::AuthenticateHandler}, managers::task_manager::TaskManager};
use shared::server::endpoint::Endpoint;
use sqlx::postgres::PgPool;

use log::error;
use dotenvy::dotenv;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    // connecting to db, and extracting shared_pool
    let shared_pool = Arc::new(
        PgPool::connect(&std::env::var("AGENT_DATABASE_URL")?).await?
    );

    let intern_port: u16 = std::env::var("AGENT_INTERN_SERVER_PORT")
        .unwrap_or_else(|e| {
            eprintln!("Env error: {}", e);
            "6767".to_string()
        })
        .parse()
        .unwrap_or_else(|e| {
            eprintln!("Parse error: {}", e);
            6767
        });

    let extern_ip = std::env::var("AGENT_EXTERN_SERVER_IP")
        .unwrap_or_else(|e| {
            eprintln!("Env error: {}", e);
            "127.0.0.1".to_string()
        });

    let extern_port: u16 = std::env::var("AGENT_EXTERN_SERVER_PORT")
        .unwrap_or_else(|e| {
            eprintln!("Env error: {}", e);
            "6969".to_string()
        })
        .parse()
        .unwrap_or_else(|e| {
            eprintln!("Parse error: {}", e);
            6969
        });

    let intern_endpoint = Arc::new(Endpoint::new("127.0.0.1", intern_port));
    let extern_endpoint = Arc::new(Endpoint::new(extern_ip, extern_port));

    let task_manager = Arc::new(TaskManager::new(shared_pool.clone(), intern_endpoint.clone()));
    let _result = TaskManager::run_task(task_manager.clone(), 2, ScriptType::Install).await;


    let mut intern_server = agent_lib::intern_server::server::Server::new(intern_endpoint.clone());

    intern_server.add_handler("authenticate", Arc::new(AuthenticateHandler::new(shared_pool.clone(), task_manager.clone())));

    let intern_handle = tokio::spawn(async move {
        if let Err(e) = intern_server.start_server().await {
            error!("[INTERN SERVER ERROR] {}", e);
        }
    });

    let mut extern_server = Server::new(extern_endpoint.clone(), shared_pool.clone());

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

    let extern_handle = tokio::spawn(async move {
        if let Err(e) = extern_server.start_server().await {
            error!("[EXTERN SERVER ERROR] {}", e);
        }
    });

    tokio::try_join!(intern_handle, extern_handle)?;

    Ok(())
}