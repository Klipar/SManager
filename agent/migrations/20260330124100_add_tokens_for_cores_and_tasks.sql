-- Add migration script here
-- Add token_hash column to TASKS
ALTER TABLE TASKS
ADD COLUMN token_hash TEXT NOT NULL;

-- Add token_hash column to CORES
ALTER TABLE CORES
ADD COLUMN token_hash TEXT NOT NULL;

CREATE UNIQUE INDEX idx_tasks_token_hash ON TASKS(token_hash);
CREATE UNIQUE INDEX idx_cores_token_hash ON CORES(token_hash);
