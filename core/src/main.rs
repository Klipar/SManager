use std::sync::Arc;

use core_lib::{handler::new_user_handler::NewUserHandler, state::AppState};
use sqlx::postgres::PgPool;
use dotenvy::dotenv;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let pg_pool = PgPool::connect(&std::env::var("DATABASE_URL_CORE")?).await?;
    let state = AppState::new(pg_pool);

    let mut server = server::server::Server::new("127.0.0.1".to_string(), 6767);

    // CRUD for Users
    server.add_handler("new-user", Arc::new(NewUserHandler::new(state.pool.clone())));

    server.start_server().await;

    Ok(())
}
