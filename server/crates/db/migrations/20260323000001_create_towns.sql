CREATE TABLE IF NOT EXISTS towns (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    owner TEXT,
    config_json TEXT NOT NULL DEFAULT '{}',
    settings_json TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
