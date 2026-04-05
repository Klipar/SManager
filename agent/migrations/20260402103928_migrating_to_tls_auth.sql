-- Add migration script here

ALTER TABLE CORES
RENAME COLUMN token_hash TO client_hash;

DROP INDEX IF EXISTS idx_cores_token_hash;

CREATE UNIQUE INDEX idx_cores_client_hash ON CORES(client_hash);

-- add test core
INSERT INTO cores (ip, name, client_hash)
    VALUES ('127.0.0.1', 'testing core', 'lI/mA/YdwDa1xZbcCf484/PTDckPAkyF88gtssyrZ50=');