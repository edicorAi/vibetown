CREATE TABLE IF NOT EXISTS merge_requests (
    id TEXT PRIMARY KEY NOT NULL,
    work_item_id TEXT REFERENCES work_items(id),
    rig_id TEXT REFERENCES rigs(id),
    branch TEXT NOT NULL,
    target_branch TEXT NOT NULL DEFAULT 'main',
    status TEXT NOT NULL DEFAULT 'pending',
    agent_id TEXT REFERENCES agents(id),
    pr_url TEXT,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    queued_at TEXT NOT NULL DEFAULT (datetime('now')),
    merged_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
