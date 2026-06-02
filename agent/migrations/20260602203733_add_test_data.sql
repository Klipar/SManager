------------------------------------------------------------
-- Creating prime core for tests.
------------------------------------------------------------

INSERT INTO actions (core_id, timestamp) -- empty core id because its first actions
VALUES
    (NULL, datetime('now')),
    (NULL, datetime('now'));

INSERT INTO cores (id, spiffe_id, create_action, update_action) -- insert prime core
VALUES
    (1, 'spiffe://dev/client1', 1, 2);

UPDATE actions  -- self referencing to ourself as a creator
  SET core_id = 1
  WHERE id = 1;

UPDATE actions
  SET core_id = 1
  WHERE id = 2;

------------------------------------------------------------
-- Add second core
------------------------------------------------------------

INSERT INTO actions (core_id, timestamp)
VALUES
    (1, datetime('now')),
    (1, datetime('now'));

INSERT INTO cores (spiffe_id, create_action, update_action) -- insert prime core
VALUES
    ('spiffe://dev/client2', 3, 4);

------------------------------------------------------------
-- Add tasks and scripts
------------------------------------------------------------

INSERT INTO actions (core_id, timestamp)
VALUES
    (1, datetime('now')),
    (1, datetime('now')),
    (1, datetime('now')),
    (1, datetime('now'));

INSERT INTO tasks (
    id, name, description,
    restart_policy,
    restart_sec,
    start_limit_burst,
    start_limit_interval_sec,
    active_state,
    substatus,
    create_action,
    update_action
)
VALUES
    (1, 'resource monitor', 'test resource monitor with moke data', 'on-failure', 2, 5, 10, 'inactive', 'dead', 5, 6),
    (2, 'Hello', 'background worker', 'always', 1, 3, 10, 'inactive', 'dead', 7, 8);

INSERT INTO actions (core_id, timestamp)
VALUES
    (1, datetime('now')),
    (1, datetime('now')),
    (1, datetime('now')),
    (1, datetime('now'));

INSERT INTO scripts (id, task_id, name, code)
VALUES
    (1, 1, 'first script', 'echo first script'),
    (2, 1, 'second script ', 'echo second script'),
    (3, 2, 'test scrypt', 'echo test scrypt');
