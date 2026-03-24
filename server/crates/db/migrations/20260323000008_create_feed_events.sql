CREATE TABLE IF NOT EXISTS feed_events (
    id TEXT PRIMARY KEY NOT NULL,
    event_type TEXT NOT NULL,
    source TEXT NOT NULL,
    rig_id TEXT,
    agent_id TEXT,
    work_item_id TEXT,
    summary TEXT NOT NULL,
    details_json TEXT NOT NULL DEFAULT '{}',
    severity TEXT NOT NULL DEFAULT 'info',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS idx_feed_events_created_at ON feed_events(created_at);
CREATE INDEX IF NOT EXISTS idx_feed_events_rig_id ON feed_events(rig_id);
