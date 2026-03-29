CREATE TYPE RESTART_POLICY AS ENUM (
    'no',
    'always',
    'on-failure'
);

CREATE TYPE TASK_STATUS AS ENUM (
    'ok',
    'starting',
    'failed',
    'stopped',
    'executed'
);

CREATE TABLE CORES (
    id SERIAL PRIMARY KEY,
    ip VARCHAR NOT NULL,
    port INT NOT NULL,
    name VARCHAR
);

CREATE TABLE TASKS (
    id SERIAL PRIMARY KEY,
    core_id INT,
    name VARCHAR NOT NULL,
    description VARCHAR,

    install_script VARCHAR,
    run_script VARCHAR,
    delete_script VARCHAR,

    restart_policy RESTART_POLICY DEFAULT 'no',
    status TASK_STATUS DEFAULT 'stopped',

    FOREIGN KEY (core_id)
        REFERENCES cores(id)
        ON DELETE SET NULL
);

CREATE TABLE RUNS (
    id BIGSERIAL PRIMARY KEY,
    task_id INT NOT NULL,
    core_id INT NOT NULL,

    script VARCHAR NOT NULL,
    start_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    end_time TIMESTAMP,
    return_code INT,
    output TEXT,

    FOREIGN KEY (task_id)
        REFERENCES tasks(id)
        ON DELETE CASCADE,

    FOREIGN KEY (core_id)
        REFERENCES cores(id)
        ON DELETE CASCADE
);