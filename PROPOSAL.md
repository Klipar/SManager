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

## What problems does it solve:
+ Dashboard and remote control of list of servers.
+ Center of statistic data collection, both for standard server loading and any custom data
+ Simultaneously centralized dashboard with multi user access and distributed control system according to possibility one core connect to various agent and one agent support many connected cores, which protect from situation when control of server cluster can by lost according to shutting down control server.
+ System of automatic creation and monitoring user specific tasks, thankfully flexibly script oriented architecture and GUI build in GUI interaction.
## What we expecting to learn:
From this project we expect to learn following:
- Mastering Ownership and Borrowing rules to manage system resources and sockets.
- Implementing flexible interfaces for diverse Task types and system metrics using Traits and Generic programming.
- Developing a resilient error-handling architecture using Result and Option types to ensure background agent stability.
- Leveraging the Tokio Async/Await runtime to monitor hundreds of processes and handle network requests concurrently.
- Utilizing asynchronous channels for thread-safe message passing between Bash scripts and WebSocket streams.
- Designing custom TCP socket protocols.
- Implementing secure node authentication and traffic encryption.
- Interfacing with the operating system for real-time metric collection and Bash script execution.
- Building a type-safe REST API to ensure data integrity at compile time.
- Managing shared application state and asynchronous SQL interactions within a multi-threaded PostgreSQL environment.
- Exploring the Rust binary life cycle, including static linking and deployment across various remote server configurations.
# Requirements
+ [ ] Possibility to connect core and agents.
+ [ ] Possibility to run tasks on agents.
+ [ ] Possibility to monitor tasks state via sockets.
+ [ ] Possibility to connect to agents via web GUI.
+ [ ] Possibility to support many users per core.
+ [ ] Possibility to create new tasks from web GUI and run them.
+ [ ] Possibility to separate users by rights.
+ [ ] Possibility to communicate with task from GUI.
+ [ ] Possibility to centralize logs at core from agents and save them.
+ [ ] Secure data transferring.
+ [ ] health checks for tasks.
## Libraries
1. tokio
2. axum
3. serde/serde_json
4. sqlx
5. dotenvy
6. sysinfo
7. chrono
8. argon2
9. jsonwebtoken
10. rustls
11. axum-server
12. validator
13. tower-http
14. tokio-util
15. tempfile

### Web Requirements
**React** - this JS framework will be useful for building reliable web frontend. We need to have real time update of our application with effective resource expenses. Also usage of components what is key approach in React is very handy tool for reusage of code and standardizing GUI. Why web application? That's because we don't need to have accesses to disk and operating system on machine where our dashboard will be open. Furthermore, web applications are more indifferent and easier to optimize for different operating systems and web browsers.
