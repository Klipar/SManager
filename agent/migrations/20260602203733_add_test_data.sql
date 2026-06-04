------------------------------------------------------------
-- Creating prime core for tests
------------------------------------------------------------

INSERT INTO cores (
    id,
    spiffe_id,
    create_by_core_id,
    create_at,
    update_by_core_id,
    update_at
)
VALUES (
    1,
    'spiffe://dev/client1',
    1,
    datetime('now'),
    1,
    datetime('now')
);

------------------------------------------------------------
-- Add second core
------------------------------------------------------------

INSERT INTO cores (
    spiffe_id,
    create_by_core_id,
    create_at,
    update_by_core_id,
    update_at
)
VALUES (
    'spiffe://dev/client2',
    1,
    datetime('now'),
    1,
    datetime('now')
);

------------------------------------------------------------
-- Add tasks
------------------------------------------------------------

INSERT INTO tasks (
    id, name, description,
    restart_policy,
    restart_sec,
    start_limit_burst,
    start_limit_interval_sec,
    active_state,
    substatus,

    create_by_core_id,
    create_at,
    update_by_core_id,
    update_at
)
VALUES
(
    1,
    'resource monitor',
    'test resource monitor with mock data',
    'on-failure',
    2,
    5,
    10,
    'inactive',
    'dead',

    1,
    datetime('now'),
    1,
    datetime('now')
),
(
    2,
    'Hello',
    'background worker',
    'always',
    1,
    3,
    10,
    'inactive',
    'dead',

    1,
    datetime('now'),
    1,
    datetime('now')
);

------------------------------------------------------------
-- Add scripts
------------------------------------------------------------

INSERT INTO scripts (id, task_id, name, code)
VALUES
    (1, 1, 'first script', 'echo first script'),
    (2, 1, 'second script', 'echo second script'),
    (3, 2, 'test script', 'echo test script');
