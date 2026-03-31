use agent_lib::handler::{authenticate_handler::AuthenticateHandler, get_all_cores_handler::GetAllCoresHandler, new_core_handler::NewCoreHandler};
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

    server.add_handler("authenticate", Arc::new(AuthenticateHandler::new(shared_pool.clone())));
    server.add_handler("new-core", Arc::new(NewCoreHandler::new(shared_pool.clone())));
    server.add_handler("get-all-cores", Arc::new(GetAllCoresHandler::new(shared_pool.clone())));

    server.start_server().await?;

    Ok(())
}