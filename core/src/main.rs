    use std::sync::Arc;
    use sqlx::postgres::PgPool;
    use dotenvy::dotenv;
    use core_lib::tls_client::connection_manager::ConnectionManager;
    use core_lib::tls_client::requests::task::{start_stream, run_task, stop_task, stop_stream,get_all_tasks,update_task,remove_task};
    use core_lib::tls_client::requests::core::{new_core,get_all_cores,update_core, remove_core};
    use shared::server::dto::create_core_dto::CreateCoreDto;
    use shared::db::models::task::Task;
    use shared::enums::script_types::ScriptType;
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
    use shared::db::models::{RestartPolicy, TaskStatus};
    use shared::server::dto::get_cores_dto::CoresDTO;

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
            let manager = ConnectionManager::connect(&agent_ip, agent_port, &agent_cn).await?;
            // let mut stream = start_stream(&manager).await?;
    //         run_task(&manager, RunTaskDTO {
    //             task_id: 2,
    //             script_type: ScriptType::Run,
    //         }).await?;
    //
    //         let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(30));
    //         tokio::pin!(timeout);
    //
    //         loop {
    //             tokio::select! {
    //                 Some(run) = stream.recv() => {
    //                     if run.end_time.is_some() {
    //                         println!("[Run {}] task={} DONE return_code={:?}", run.id, run.task_id, run.return_code);
    //                         break;
    //                     } else {
    //                         if let Some(output) = &run.output {
    //                             if let Some(last_line) = output.trim_end_matches('\n').lines().last() {
    //                                 println!("[Run {}] >> {}", run.id, last_line);
    //                             }
    //                         }
    //                     }
    //     }
    //     _  = &mut timeout => {
    //         println!("Timeout — stopping task...");
    //         stop_task(&manager, 2).await?;
    //         stop_stream(&manager).await?;
    //         break;
    //     }
    // }
    //         }
    //         new_core(&manager, CreateCoreDto {
    //             ip: "192.168.1.10".to_string(),
    //             name: "my-core".to_string(),
    //             client_cn: "my-core-cn".to_string(),
    //         }).await?;
    //         let cores = get_all_cores(&manager).await?;
    //         for core in cores {
    //             println!("Core {}: {} ({})", core.id, core.name, core.ip);
    //         }
            // let updated = update_core(&manager, CoresDTO {
            //     id: 1,
            //     ip: "192.168.1.10".to_string(),
            //     name: "Core-1".to_string(),
            // }).await?;
            // remove_core(&manager, 3).await?;
            // let cores = get_all_cores(&manager).await?;
            // for core in cores {
            //     println!("Core {}: {} ({})", core.id, core.name, core.ip);
            // }
            let tasks = get_all_tasks(&manager).await?;
            for task in tasks {
                println!("Task {}: {} ({:?})", task.id, task.name, task.status);
            }
            remove_task(&manager, 1).await?;
            // update_task(&manager, Task {
            //     id: 1,
            //     core_id:Some(1),
            //     name: "Task Name".to_string(),
            //     description: Some("Task Description".to_string()),
            //     install_script: Some("install.sh".to_string()),
            //     run_script: Some("run.sh".to_string()),
            //     delete_script: Some("delete.sh".to_string()),
            //     restart_policy: RestartPolicy::Always,
            //     status: TaskStatus::Stopped,
            // }).await?;
            let tasks = get_all_tasks(&manager).await?;
            for task in tasks {
                println!("Task {}: {} ({:?})", task.id, task.name, task.status);
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