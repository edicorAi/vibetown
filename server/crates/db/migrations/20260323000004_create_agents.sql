CREATE TABLE IF NOT EXISTS agents (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    role TEXT NOT NULL,
    rig_id TEXT REFERENCES rigs(id),
    status TEXT NOT NULL DEFAULT 'idle',
    runtime TEXT,
    config_json TEXT NOT NULL DEFAULT '{}',
    last_activity_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
