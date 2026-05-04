use std::sync::Arc;
use sqlx::postgres::PgPool;
use dotenvy::dotenv;
use core_lib::tls_client::client::AgentClient;
use core_lib::tls_client::connection::connect;
use shared::server::message::Message;

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

    {
        let agent_ip = std::env::var("AGENT_SERVER_IP").unwrap_or("127.0.0.1".to_string());
        let agent_port: u16 = std::env::var("AGENT_SERVER_PORT")
            .unwrap_or("6969".to_string())
            .parse()?;
        let agent_cn = std::env::var("AGENT_SERVER_CN").unwrap_or("localhost".to_string());

        println!("Connecting to agent {}:{}", agent_ip, agent_port);

        match connect(&agent_ip, agent_port, &agent_cn).await {
            Ok(framed) => {
                println!("TLS connected!");
                let (client, mut inbound_rx) = AgentClient::new(framed);

                let msg = Message::Request {
                    id: 1,
                    action: "ping".to_string(),
                    data: Some(serde_json::json!({})),
                };

                client.send(msg).await?;
                println!("Ping sent!");

                match tokio::time::timeout(
                    std::time::Duration::from_secs(5),
                    inbound_rx.recv()
                ).await {
                    Ok(Some(response)) => println!("Response: {:?}", response),
                    Ok(None)           => println!("Channel closed"),
                    Err(_)             => println!("Timeout — no response"),
                }
            }
            Err(e) => eprintln!("Failed to connect to agent: {e}"),
        }
    }

    let ip = std::env::var("CORE_SERVER_IP")
        .unwrap_or_else(|e| {
            eprintln!("Env error: {}", e);
            "127.0.0.1".to_string()
        });

    let port: u16 = std::env::var("CORE_SERVER_PORT")
        .unwrap_or_else(|e| {
            eprintln!("Env error: {}", e);
            "6767".to_string()
        })
        .parse()
        .unwrap_or_else(|e| {
            eprintln!("Parse error: {}", e);
            6767
        });

    let mut server = Server::new(ip.to_string(), port);

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

    // Read Logs
    server.add_handler("get-logs", Arc::new(GetLogsHandler::new(state.pool.clone())));

    server.start_server().await;

    Ok(())
}