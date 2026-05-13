# Team
`Bohdan Karpenko`, `Arseniy Malyuk`, `Horshchar Yevhen`

# Introduction to idea
## Idea
Create multi server management system, dashboard and monitor simultaneously. Program would be separated on 2 physical parts and 4 logical.
### Physical separation:
- **Agent** - program that would be installed on monitored server. According to cores control it can run tasks and provide API allowing them send data to Web GUI.
- **Core** - program that would be installed on servers from which monitoring would be executed. It connect to agents and run tasks on them, providing Web GUI interface.
### Logical separations:
- **Agent** - Execute and managing tasks according to cores commands.
- **Core** - Centralizing unit for agents, that manage them and collect logs.
- **Web GUI** - GUI that propose core to user. Through it user can connect/edit/delete cores and users, run/stop/edit/delete/export/interact with tasks.
- **Task** - Unit of execution on server. It consist of 3 bash scripts:
	+ **Install script** - install necessary programs, dirs, config files. Run while creation Task.
	+ **Run script** - connect to agent via socket and execute payload.
	+ **Delete script** - clear cash files, configs, working dirs. Run at deleting Task.
  According to possibility tasks connect to agent via socket it can not only send its status, logs, and other statistical data to agent, cores and GUI specifically but also receive custom commands from GUI and act according to them.
## Requirements
# Agent
+ Installed on monitored servers; connects to Core via mutual TLS.
+ Keep heartbeat and forwards task runs/stdout to Core.
+ Runs, stops, deletes tasks as commanded.
# Core
+ Central server with Web GUI, database, and API.
+ Manages agent registry, task definitions, task instances.
+ Sends install/run/delete scripts to agents.
+ Supports multiple users with roles (Admin, User).
# Web GUI
+ Dashboard: list agents, list tasks, task status.
+ Agent management: add/edit/remove agents.
+ Task management: create task (upload 3 bash scripts), run on selected agents, monitor.
+ User management (Admin only).
+ Core configuration.
# Task
+ Composed of three bash scripts: install, run, delete.
+ Logs are captured by agent and streamed to Core(stdout, stderr and return code captured to Run and delivered to Web GUI).
# Security & Non-functional
+ TLS 1.2+ for all communications (agent<->core, GUI<->core).
+ Passwords hashed with salt.
## Design diagram
![diagram](assets\Design-diagram.png)
# Design choices
We structured the project into several crates: agent for task management, core for the central server/orchestrator, shared for common DTOs, enums, messages and models, frontend for the user interface, and tasks for background task execution. This modular approach keeps concerns separated, allows both core and agent to reuse shared types, and makes the architecture easier to test and understand. A single monolithic crate would have created tighter coupling and made schema sharing more difficult.

For asynchronous execution we chose the Tokio runtime. It handles networking, TLS, database access, and task management. Tokio works well with WebSocket/TLS streaming and ping/reconnect logic, has a mature ecosystem, and supports libraries like tokio-tungstenite, tokio-rustls, and sqlx. The main alternative would be async-std or smol, but those have less ecosystem support for our stack.

Agent connections use TLS over WebSocket with a custom protocol. Messages are generic request/response structures plus push notifications. The AgentClient and ConnectionManager handle sending, receiving, and health checking. This ensures encrypted and authenticated communication, provides real-time streaming, and remains flexible for ping, execution streams, and other actions. Alternatives like plain TCP, HTTP/REST, or gRPC would be simpler in some ways but less suitable for persistent bidirectional streaming with authentication.

All shared definitions (messages, DTOs, enums, models) live in the shared crate. Both core and agent compile against the same definitions, which avoids duplicate schemas and serialization mismatches. Copy-pasting types across crates would risk errors and inconsistencies.

For robust connection management we implemented reconnect and health-check orchestration. The AgentOrchestrator holds ConnectionManager instances. Each manager uses ping and disconnect notifications to detect broken peers, and the orchestrator removes stale connections and retries. This handles unexpected drops, ensures recovery without manual restart, and keeps the logic centralised. Letting the transport reconnect silently or requiring manual reconnection would be less reliable for long-running tasks.

For database access we use sqlx. It provides asynchronous PostgreSQL queries that are non-blocking, fits the async runtime well, offers compile-time checking via query macros, and works with our existing schema and migrations. Alternatives like Diesel or blocking drivers would be heavier or require thread pools.
# Dependencies
## Agent Crate
+ anyhow: Unified error handling
+ async-trait: Macro for async trait methods
+ base64: Base64 encoding/decoding
+ chrono: Date and time manipulation
+ dashmap: Concurrent HashMap for multithreaded access
+ dotenvy: Loading environment variables from .env files
+ env_logger: Logging with environment configuration
+ futures: Asynchronous utilities and futures
+ jsonwebtoken: JWT tokens for authentication
+ log: Logging facade
+ nix: Unix system calls
+ rand: Generating random numbers
+ rustls: TLS implementation
+ rustls-pemfile: PEM files for rustls
+ serde: Data serialization/deserialization
+ serde_json: JSON serialization
+ sha2: SHA256 hashing
+ shared: Shared code between crates
+ sqlx: Asynchronous database access (PostgreSQL)
+ tokio: Asynchronous runtime
+ tokio-rustls: Tokio integration with rustls
+ tokio-util: Additional tokio utilities
+ x509-parser: Parsing X509 certificates

## Core Crate
+ anyhow: Error handling
+ async-trait: Async traits
+ base64: Base64 encoding
+ chrono: Date/time
+ dotenvy: Environment variables
+ env_logger: Logging
+ futures: Async utilities
+ futures-util: Futures utilities
+ jsonwebtoken: JWT handling
+ log: Logging
+ rand: Random numbers
+ rustls-pemfile: PEM files
+ serde: Serialization
+ serde_json: JSON
+ sha2: Hashing
+ shared: Shared code
+ sqlx: Database
+ tokio: Async runtime
+ tokio-rustls: TLS
+ tokio-tungstenite: WebSocket client/server
+ tokio-util: Tokio utilities
# Evaluation
This was a very interesting but quite challenging assignment. We spent a lot of time implementing the agent, handling script execution within tasks, writing outputs to runs, and setting up communication between the agent and the task. The hardest part for us was establishing communication between the Web GUI, Core, and Agent, as it required designing a communication protocol and setting up proper handlers for incoming messages. We also had to manage real-time communication and collaboration on the project effectively. Designing the modern GUI came quite easily to us because we already had plenty of experience with that. Our project still has many potential directions for further development, and my colleagues and I plan to continue working on it in our free time.
