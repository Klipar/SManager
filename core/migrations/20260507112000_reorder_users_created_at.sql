BEGIN;

ALTER TABLE logs DROP CONSTRAINT IF EXISTS logs_user_id_fkey;

ALTER TABLE users RENAME TO users_old;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    password VARCHAR NOT NULL,
    is_admin BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP,
    gui_settings JSONB
);

INSERT INTO users (
    id,
    name,
    email,
    password,
    is_admin,
    created_at,
    updated_at,
    last_login,
    gui_settings
)
SELECT
    id,
    name,
    email,
    password,
    is_admin,
    created_at,
    last_update,
    last_login,
    gui_settings
FROM users_old;

DO $$
DECLARE
    sequence_name text;
    max_id bigint;
BEGIN
    sequence_name := pg_get_serial_sequence('users', 'id');
    SELECT COALESCE(MAX(id), 0) INTO max_id FROM users;

    IF max_id > 0 THEN
        PERFORM setval(sequence_name, max_id, true);
    ELSE
        PERFORM setval(sequence_name, 1, false);
    END IF;
END $$;

DROP TABLE users_old;

ALTER TABLE logs
ADD CONSTRAINT logs_user_id_fkey
FOREIGN KEY (user_id)
REFERENCES users(id)
ON DELETE SET NULL;

COMMIT;
