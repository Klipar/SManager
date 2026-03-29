use agent_lib::handler::test_handler::TestHandler;
use sqlx::postgres::PgPool;
use shared::server::server::Server;
use dotenvy::dotenv;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // connecting to db, and extracting shared_pool
    dotenv().ok();
    let shared_pool = Arc::new(
        PgPool::connect(&std::env::var("DATABASE_URL")?).await?
    );

    let mut server = Server::new("127.0.0.1".to_string(), 6969);

    server.add_handler("test", Arc::new(TestHandler::new(shared_pool.clone())));

    server.start_server().await?;

    Ok(())
}