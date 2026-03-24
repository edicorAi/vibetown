CREATE TABLE IF NOT EXISTS rigs (
    id TEXT PRIMARY KEY NOT NULL,
    town_id TEXT REFERENCES towns(id),
    name TEXT NOT NULL UNIQUE,
    repo_url TEXT,
    beads_prefix TEXT NOT NULL,
    config_json TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
