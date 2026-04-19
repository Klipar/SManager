CREATE TYPE SCRIPT_TYPE AS ENUM (
    'install',
    'run',
    'delete'
);

UPDATE runs SET script = 'run';

ALTER TABLE runs
ALTER COLUMN script TYPE script_type
USING script::script_type;