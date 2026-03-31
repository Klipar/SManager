-- Add migration script here
-- Drop unique constraint first (it depends on port)
ALTER TABLE CORES
DROP CONSTRAINT IF EXISTS unique_ip_port;

-- Drop the column
ALTER TABLE CORES
DROP COLUMN IF EXISTS port;