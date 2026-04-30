ALTER TABLE logs
    DROP CONSTRAINT IF EXISTS logs_task_id_fkey;

ALTER TABLE tasks
    DROP CONSTRAINT IF EXISTS tasks_pkey;

ALTER TABLE tasks
    ADD CONSTRAINT tasks_id_key UNIQUE (id);

ALTER TABLE tasks
    ADD CONSTRAINT tasks_pkey PRIMARY KEY (id, agent_id);

ALTER TABLE logs
    ADD CONSTRAINT logs_task_id_fkey
    FOREIGN KEY (task_id)
    REFERENCES tasks(id)
    ON DELETE SET NULL;