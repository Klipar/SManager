use core_lib::state::AppState;
use sqlx::postgres::PgPool;
use dotenvy::dotenv;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let state = AppState::new(PgPool::connect(&std::env::var("DATABASE_URL_CORE")?).await?);

    let mut server = server::server::Server::new("127.0.0.1".to_string(), 6767);

    server.start_server().await;

    Ok(())
}
