use std::sync::Arc;
use sqlx::postgres::PgPool;
use dotenvy::dotenv;
use core_lib::tls_client::orchestrator::AgentOrchestrator;
use core_lib::{
    handler::{
        get_all_agents_handler::GetAllAgentsHandler,
        get_all_users_handler::GetAllUsersHandler,
        get_all_tasks_handler::GetAllTasksHandler,
        get_logs_handler::GetLogsHandler,
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
        connect_agent_handler::ConnectAgentHandler,
        run_task_handler::RunTaskHandler,
        stop_task_handler::StopTaskHandler,
    },
    state::AppState,
    server::server::Server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let pg_pool = PgPool::connect(&std::env::var("CORE_DATABASE_URL")?).await?;
    let state = AppState::new(pg_pool);
    let orchestrator = Arc::new(AgentOrchestrator::new());

    // Pripoj všetkých agentov pri štarte
    let errors = orchestrator.connect_all(&state.pool).await;
    for (agent_id, e) in &errors {
        eprintln!("Could not connect to agent {}: {}", agent_id, e);
    }

    let ip = std::env::var("CORE_SERVER_IP").unwrap_or("127.0.0.1".to_string());
    let port: u16 = std::env::var("CORE_SERVER_PORT")
        .unwrap_or("6767".to_string())
        .parse()
        .unwrap_or(6767);

    let mut server = Server::new(ip.to_string(), port);

    // Users
    server.add_handler("login", Arc::new(LoginUserHandler::new(state.pool.clone())));
    server.add_handler("new-user", Arc::new(NewUserHandler::new(state.pool.clone())));
    server.add_handler("get-all-users", Arc::new(GetAllUsersHandler::new(state.pool.clone())));
    server.add_handler("update-user", Arc::new(UpdateUserHandler::new(state.pool.clone())));
    server.add_handler("remove-user", Arc::new(RemoveUserHandler::new(state.pool.clone())));

    // Agents
    server.add_handler("new-agent", Arc::new(NewAgentHandler::new(state.pool.clone())));
    server.add_handler("get-all-agents", Arc::new(GetAllAgentsHandler::new(state.pool.clone())));
    server.add_handler("update-agent", Arc::new(UpdateAgentHandler::new(state.pool.clone())));
    server.add_handler("remove-agent", Arc::new(RemoveAgentHandler::new(state.pool.clone())));
    server.add_handler("connect-agent", Arc::new(ConnectAgentHandler::new(
        state.pool.clone(),
        Arc::clone(&orchestrator),
    )));

    // Tasks
    server.add_handler("new-task", Arc::new(NewTaskHandler::new(
        state.pool.clone(),
        Arc::clone(&orchestrator),
    )));
    server.add_handler("get-all-tasks", Arc::new(GetAllTasksHandler::new(state.pool.clone())));
    server.add_handler("update-task", Arc::new(UpdateTaskHandler::new(state.pool.clone())));
    server.add_handler("remove-task", Arc::new(RemoveTaskHandler::new(state.pool.clone())));
    server.add_handler("run-task", Arc::new(RunTaskHandler::new(Arc::clone(&orchestrator))));
    server.add_handler("stop-task", Arc::new(StopTaskHandler::new(Arc::clone(&orchestrator))));

    // Logs
    server.add_handler("get-logs", Arc::new(GetLogsHandler::new(state.pool.clone())));

    server.start_server().await;

    Ok(())
}