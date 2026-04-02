use agent_lib::handler::{get_all_cores_handler::GetAllCoresHandler, get_all_task_handler::GetAllTaskHandler, new_core_handler::NewCoreHandler, new_task_handler::NewTaskHandler, remove_core_handler::RemoveCoreHandler, remove_task_handler::RemoveTaskHandler, update_cure::UpdateCoreHandler, update_task_handler::UpdateTaskHandler};
use sqlx::postgres::PgPool;
use shared::server::server::Server;
use dotenvy::dotenv;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();
    // connecting to db, and extracting shared_pool
    let shared_pool = Arc::new(
        PgPool::connect(&std::env::var("DATABASE_URL_Agent")?).await?
    );

    let mut server = Server::new("127.0.0.1".to_string(), 6969, shared_pool.clone());

    // CRUD for Cores
    server.add_handler("new-core", Arc::new(NewCoreHandler::new(shared_pool.clone())));
    server.add_handler("get-all-cores", Arc::new(GetAllCoresHandler::new(shared_pool.clone())));
    server.add_handler("update-core", Arc::new(UpdateCoreHandler::new(shared_pool.clone())));
    server.add_handler("remove-core", Arc::new(RemoveCoreHandler::new(shared_pool.clone())));

    // CRUD for Tasks
    server.add_handler("new-task", Arc::new(NewTaskHandler::new(shared_pool.clone())));
    server.add_handler("get-all-tasks", Arc::new(GetAllTaskHandler::new(shared_pool.clone())));
    server.add_handler("update-task", Arc::new(UpdateTaskHandler::new(shared_pool.clone())));
    server.add_handler("remove-task", Arc::new(RemoveTaskHandler::new(shared_pool.clone())));
    server.start_server().await?;

    Ok(())
}