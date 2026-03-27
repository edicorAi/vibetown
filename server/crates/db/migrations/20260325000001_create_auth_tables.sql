-- Users table: stores all user accounts regardless of auth provider
CREATE TABLE IF NOT EXISTS users (
    id          TEXT PRIMARY KEY NOT NULL,
    email       TEXT NOT NULL UNIQUE,
    username    TEXT UNIQUE,
    display_name TEXT,
    password_hash TEXT,
    auth_provider TEXT NOT NULL DEFAULT 'local',
    external_id TEXT,
    avatar_url  TEXT,
    is_active   INTEGER NOT NULL DEFAULT 1,
    is_admin    INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now')),
    last_login_at TEXT
);

-- User sessions for cookie-based auth
CREATE TABLE IF NOT EXISTS user_sessions (
    id            TEXT PRIMARY KEY NOT NULL,
    user_id       TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash    TEXT NOT NULL UNIQUE,
    expires_at    TEXT NOT NULL,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    last_used_at  TEXT,
    ip_address    TEXT,
    user_agent    TEXT,
    revoked       INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_user_sessions_user ON user_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_user_sessions_token ON user_sessions(token_hash);
CREATE INDEX IF NOT EXISTS idx_user_sessions_expires ON user_sessions(expires_at);

-- Roles
CREATE TABLE IF NOT EXISTS roles (
    id   TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    permissions_json TEXT NOT NULL DEFAULT '[]'
);

INSERT OR IGNORE INTO roles (id, name, permissions_json) VALUES
    ('role-admin',  'admin',  '["*"]'),
    ('role-member', 'member', '["read","write","execute"]'),
    ('role-viewer', 'viewer', '["read"]');

-- Teams
CREATE TABLE IF NOT EXISTS teams (
    id          TEXT PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL,
    description TEXT,
    created_by  TEXT REFERENCES users(id),
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Team members
CREATE TABLE IF NOT EXISTS team_members (
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id TEXT NOT NULL REFERENCES roles(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (team_id, user_id)
);

-- Workspace individual sharing
CREATE TABLE IF NOT EXISTS workspace_members (
    workspace_id BLOB NOT NULL,
    user_id      TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id      TEXT NOT NULL REFERENCES roles(id),
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (workspace_id, user_id)
);

-- Workspace team sharing
CREATE TABLE IF NOT EXISTS workspace_team_access (
    workspace_id BLOB NOT NULL,
    team_id      TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    role_id      TEXT NOT NULL REFERENCES roles(id),
    created_at   TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (workspace_id, team_id)
);

-- Town individual sharing
CREATE TABLE IF NOT EXISTS town_members (
    town_id TEXT NOT NULL REFERENCES towns(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id TEXT NOT NULL REFERENCES roles(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (town_id, user_id)
);

-- Town team sharing
CREATE TABLE IF NOT EXISTS town_team_access (
    town_id TEXT NOT NULL REFERENCES towns(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    role_id TEXT NOT NULL REFERENCES roles(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (town_id, team_id)
);

-- Auth provider configurations
CREATE TABLE IF NOT EXISTS auth_providers (
    id            TEXT PRIMARY KEY NOT NULL,
    provider_type TEXT NOT NULL,
    name          TEXT NOT NULL,
    config_json   TEXT NOT NULL DEFAULT '{}',
    is_enabled    INTEGER NOT NULL DEFAULT 1,
    priority      INTEGER NOT NULL DEFAULT 0,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Add owner to workspaces
ALTER TABLE workspaces ADD COLUMN owner_user_id TEXT REFERENCES users(id);
