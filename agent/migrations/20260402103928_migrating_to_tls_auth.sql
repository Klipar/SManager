-- Add migration script here

ALTER TABLE CORES
RENAME COLUMN token_hash TO client_hash;

DROP INDEX IF EXISTS idx_cores_token_hash;

CREATE UNIQUE INDEX idx_cores_client_hash ON CORES(client_hash);