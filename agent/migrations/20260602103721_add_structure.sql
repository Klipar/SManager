PRAGMA foreign_keys = ON;

CREATE TABLE actions (
    id INTEGER PRIMARY KEY,
    core_id INTEGER, -- Can be NULL if core was deleted.
    timestamp DATETIME NOT NULL,

    FOREIGN KEY (core_id)
        REFERENCES cores(id)
        ON DELETE SET NULL
        ON UPDATE CASCADE
);

CREATE TABLE cores (
    id INTEGER PRIMARY KEY,
    spiffe_id TEXT NOT NULL,
    create_action INTEGER NOT NULL,
    update_action INTEGER NOT NULL,

    UNIQUE(spiffe_id),

    FOREIGN KEY (create_action)
        REFERENCES actions(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE,
    FOREIGN KEY (update_action)
        REFERENCES actions(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE
);

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    restart_policy TEXT
        DEFAULT 'no'
        NOT NULL
        CHECK(restart_policy IN (
            'no','always','on-success','on-failure','on-abnormal', 'on-abort', 'on-watchdog'
        )),

    restart_sec INTEGER   -- delay before restart
        DEFAULT 0 NOT NULL
        CHECK(restart_sec >= 0),
    start_limit_burst INTEGER -- count of restart attempts before fail
        DEFAULT 5 NOT NULL
        CHECK(start_limit_burst > 0),
    start_limit_interval_sec INTEGER DEFAULT 10 NOT NULL  -- time window for restarting
        CHECK(start_limit_interval_sec > 0),

    active_state TEXT
        DEFAULT 'inactive' NOT NULL
        CHECK(active_state IN (
                'active',
                'inactive',
                'activating',
                'deactivating',
                'failed',
                'reloading'
            )),

    substatus TEXT
        DEFAULT 'dead'
        NOT NULL
        CHECK(substatus != ''),

    create_action INTEGER NOT NULL,
    update_action INTEGER NOT NULL,

    FOREIGN KEY (create_action)
        REFERENCES actions(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE,
    FOREIGN KEY (update_action)
        REFERENCES actions(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE
);

CREATE TABLE scripts (
    id INTEGER PRIMARY KEY,
    task_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    code TEXT NOT NULL DEFAULT 'echo Hello from template script!',
    FOREIGN KEY (task_id) REFERENCES tasks(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE runs (
    id INTEGER PRIMARY KEY,
    task_id INTEGER NOT NULL,
    script_id INTEGER NOT NULL,
    start_action INTEGER NOT NULL,
    stop_action INTEGER,
    return_code INTEGER
        DEFAULT NULL
        CHECK(return_code IS NULL OR return_code >= 0),

    output TEXT NOT NULL DEFAULT '',

    FOREIGN KEY (task_id) REFERENCES tasks(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    FOREIGN KEY (script_id) REFERENCES scripts(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE,

    FOREIGN KEY (start_action) REFERENCES actions(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE,

    FOREIGN KEY (stop_action) REFERENCES actions(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE
);

CREATE TABLE pending_messages (
    id INTEGER PRIMARY KEY,
    message TEXT NOT NULL,
    retry_budget INTEGER DEFAULT 5 CHECK(retry_budget > 0 OR retry_budget IS NULL),
    send_to_core_id INTEGER NOT NULL,
    FOREIGN KEY (send_to_core_id) REFERENCES cores(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
