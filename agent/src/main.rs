use agent_lib::handler::{authenticate_handler::AuthenticateHandler, get_all_cores_handler::GetAllCoresHandler, get_all_task_handler::GetAllTaskHandler, new_core_handler::NewCoreHandler, new_task_handler::NewTaskHandler, remove_core_handler::RemoveCoreHandler, update_cure::UpdateCoreHandler, update_task_handler::UpdateTaskHandler};

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
        PgPool::connect(&std::env::var("DATABASE_URL")?).await?
    );

    let mut server = Server::new("127.0.0.1".to_string(), 6969);

    // Authenticate
    server.add_handler("authenticate", Arc::new(AuthenticateHandler::new(shared_pool.clone())));

    // CRUD for Cores
    server.add_handler("new-core", Arc::new(NewCoreHandler::new(shared_pool.clone())));
    server.add_handler("get-all-cores", Arc::new(GetAllCoresHandler::new(shared_pool.clone())));
    server.add_handler("update-core", Arc::new(UpdateCoreHandler::new(shared_pool.clone())));
    server.add_handler("remove-core", Arc::new(RemoveCoreHandler::new(shared_pool.clone())));

    // CRUD for Tasks
    server.add_handler("new-task", Arc::new(NewTaskHandler::new(shared_pool.clone())));
    server.add_handler("get-all-tasks", Arc::new(GetAllTaskHandler::new(shared_pool.clone())));
    server.add_handler("update-task", Arc::new(UpdateTaskHandler::new(shared_pool.clone())));
    server.start_server().await?;

    Ok(())
}