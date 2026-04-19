CREATE TABLE USERS (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	email VARCHAR UNIQUE NOT NULL,
	password VARCHAR NOT NULL,
	is_admin BOOLEAN DEFAULT FALSE,
	last_login TIMESTAMP,
	last_update TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	gui_settings JSONB
);

CREATE TYPE AGENT_STATUS AS ENUM (
    'online',
	'offline',
	'error'
);

CREATE TABLE AGENTS (
    id SERIAL PRIMARY KEY,
    ip VARCHAR NOT NULL,
    port INT NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR DEFAULT NULL,
    last_connection TIMESTAMP,
    last_message TIMESTAMP,
    status AGENT_STATUS DEFAULT 'offline'
);

CREATE TABLE TASKS (
    id INT PRIMARY KEY,
    agent_id INT NOT NULL,

    FOREIGN KEY (agent_id)
	    REFERENCES agents(id)
	    ON DELETE CASCADE
);

CREATE TABLE LOGS (
    id INT PRIMARY KEY,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    message TEXT NOT NULL,
    agent_id INT DEFAULT NULL,
    task_id INT DEFAULT NULL,
    user_id INT DEFAULT NULL,

    FOREIGN KEY (agent_id)
	    REFERENCES agents(id)
		ON DELETE SET NULL,
    FOREIGN KEY (task_id)
	    REFERENCES tasks(id)
	    ON DELETE SET NULL,
    FOREIGN KEY (user_id)
	    REFERENCES users(id)
		ON DELETE SET NULL
);