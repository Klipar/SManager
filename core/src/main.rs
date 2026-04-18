use std::sync::Arc;

use core_lib::{
    handler::{
        get_all_agents_handler::GetAllAgentsHandler,
        get_all_users_handler::GetAllUsersHandler,
        get_all_tasks_handler::GetAllTasksHandler,
        login_user_handler::LoginUserHandler,
        new_agent_handler::NewAgentHandler,
        new_user_handler::NewUserHandler,
        new_task_handler::NewTaskHandler,
        remove_agent_handler::RemoveAgentHandler,
        remove_user_handler::RemoveUserHandler,
        remove_task_handler::RemoveTaskHandler,
        update_agent_handler::UpdateAgentHandler,
        update_user_handler::UpdateUserHandler,
        update_task_handler::UpdateTaskHandler,
    },
    state::AppState,
};
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
    server.add_handler("login", Arc::new(LoginUserHandler::new(state.pool.clone())));
    server.add_handler("new-user", Arc::new(NewUserHandler::new(state.pool.clone())));
    server.add_handler("get-all-users", Arc::new(GetAllUsersHandler::new(state.pool.clone())));
    server.add_handler("update-user", Arc::new(UpdateUserHandler::new(state.pool.clone())));
    server.add_handler("remove-user", Arc::new(RemoveUserHandler::new(state.pool.clone())));

    // CRUD for Agents
    server.add_handler("new-agent", Arc::new(NewAgentHandler::new(state.pool.clone())));
    server.add_handler("get-all-agents", Arc::new(GetAllAgentsHandler::new(state.pool.clone())));
    server.add_handler("update-agent", Arc::new(UpdateAgentHandler::new(state.pool.clone())));
    server.add_handler("remove-agent", Arc::new(RemoveAgentHandler::new(state.pool.clone())));

    // CRUD for Tasks
    server.add_handler("new-task", Arc::new(NewTaskHandler::new(state.pool.clone())));
    server.add_handler("get-all-tasks", Arc::new(GetAllTasksHandler::new(state.pool.clone())));
    server.add_handler("update-task", Arc::new(UpdateTaskHandler::new(state.pool.clone())));
    server.add_handler("remove-task", Arc::new(RemoveTaskHandler::new(state.pool.clone())));

    server.start_server().await;

    Ok(())
}