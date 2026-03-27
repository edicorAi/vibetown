use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::errors::AuthError;
use crate::password;

// ── User ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub auth_provider: String,
    pub external_id: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: String,
    pub last_login_at: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub password_hash: Option<String>,
    pub auth_provider: String,
    pub external_id: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: i32,
    pub is_admin: i32,
    pub created_at: String,
    pub updated_at: String,
    pub last_login_at: Option<String>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            email: row.email,
            username: row.username,
            display_name: row.display_name,
            auth_provider: row.auth_provider,
            external_id: row.external_id,
            avatar_url: row.avatar_url,
            is_active: row.is_active != 0,
            is_admin: row.is_admin != 0,
            created_at: row.created_at,
            updated_at: row.updated_at,
            last_login_at: row.last_login_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub auth_provider: String,
    pub external_id: Option<String>,
    pub avatar_url: Option<String>,
    pub is_admin: bool,
}

pub async fn create_user(pool: &SqlitePool, data: &CreateUser) -> Result<User, AuthError> {
    let id = uuid::Uuid::new_v4().to_string();
    let password_hash = match &data.password {
        Some(pw) => Some(password::hash_password(pw)?),
        None => None,
    };

    sqlx::query(
        r#"INSERT INTO users (id, email, username, display_name, password_hash, auth_provider, external_id, avatar_url, is_admin)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(&data.email)
    .bind(&data.username)
    .bind(&data.display_name)
    .bind(&password_hash)
    .bind(&data.auth_provider)
    .bind(&data.external_id)
    .bind(&data.avatar_url)
    .bind(data.is_admin as i32)
    .execute(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.message().contains("UNIQUE") => {
            AuthError::UserAlreadyExists
        }
        _ => AuthError::Database(e),
    })?;

    get_user_by_id(pool, &id)
        .await?
        .ok_or(AuthError::Internal("User created but not found".into()))
}

pub async fn get_user_by_id(pool: &SqlitePool, id: &str) -> Result<Option<User>, AuthError> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"SELECT id, email, username, display_name, password_hash, auth_provider,
                  external_id, avatar_url, is_active, is_admin, created_at, updated_at, last_login_at
           FROM users WHERE id = ?"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(User::from))
}

pub async fn get_user_by_email(pool: &SqlitePool, email: &str) -> Result<Option<UserRow>, AuthError> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"SELECT id, email, username, display_name, password_hash, auth_provider,
                  external_id, avatar_url, is_active, is_admin, created_at, updated_at, last_login_at
           FROM users WHERE email = ?"#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn get_user_by_external_id(
    pool: &SqlitePool,
    provider: &str,
    external_id: &str,
) -> Result<Option<UserRow>, AuthError> {
    let row = sqlx::query_as::<_, UserRow>(
        r#"SELECT id, email, username, display_name, password_hash, auth_provider,
                  external_id, avatar_url, is_active, is_admin, created_at, updated_at, last_login_at
           FROM users WHERE auth_provider = ? AND external_id = ?"#,
    )
    .bind(provider)
    .bind(external_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, AuthError> {
    let rows = sqlx::query_as::<_, UserRow>(
        r#"SELECT id, email, username, display_name, password_hash, auth_provider,
                  external_id, avatar_url, is_active, is_admin, created_at, updated_at, last_login_at
           FROM users ORDER BY created_at ASC"#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(User::from).collect())
}

pub async fn update_user_active(
    pool: &SqlitePool,
    user_id: &str,
    is_active: bool,
) -> Result<(), AuthError> {
    sqlx::query("UPDATE users SET is_active = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(is_active as i32)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_user_admin(
    pool: &SqlitePool,
    user_id: &str,
    is_admin: bool,
) -> Result<(), AuthError> {
    sqlx::query("UPDATE users SET is_admin = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(is_admin as i32)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_user_password(
    pool: &SqlitePool,
    user_id: &str,
    new_password: &str,
) -> Result<(), AuthError> {
    let hash = password::hash_password(new_password)?;
    sqlx::query("UPDATE users SET password_hash = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(&hash)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_last_login(pool: &SqlitePool, user_id: &str) -> Result<(), AuthError> {
    sqlx::query("UPDATE users SET last_login_at = datetime('now') WHERE id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn user_count(pool: &SqlitePool) -> Result<i64, AuthError> {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

// ── Session ──

#[derive(Debug, Clone, Serialize)]
pub struct UserSession {
    pub id: String,
    pub user_id: String,
    pub expires_at: DateTime<Utc>,
    pub revoked: bool,
}

#[derive(Debug, FromRow)]
pub struct UserSessionRow {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub expires_at: String,
    pub last_used_at: Option<String>,
    pub revoked: i32,
}

// ── Team ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, FromRow)]
pub struct TeamRow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<TeamRow> for Team {
    fn from(row: TeamRow) -> Self {
        Team {
            id: row.id,
            name: row.name,
            description: row.description,
            created_by: row.created_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTeam {
    pub name: String,
    pub description: Option<String>,
}

pub async fn create_team(
    pool: &SqlitePool,
    data: &CreateTeam,
    created_by: &str,
) -> Result<Team, AuthError> {
    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        r#"INSERT INTO teams (id, name, description, created_by) VALUES (?, ?, ?, ?)"#,
    )
    .bind(&id)
    .bind(&data.name)
    .bind(&data.description)
    .bind(created_by)
    .execute(pool)
    .await?;

    // Add creator as admin member
    sqlx::query(
        r#"INSERT INTO team_members (team_id, user_id, role_id) VALUES (?, ?, 'role-admin')"#,
    )
    .bind(&id)
    .bind(created_by)
    .execute(pool)
    .await?;

    get_team_by_id(pool, &id)
        .await?
        .ok_or(AuthError::Internal("Team created but not found".into()))
}

pub async fn get_team_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Team>, AuthError> {
    let row = sqlx::query_as::<_, TeamRow>(
        "SELECT id, name, description, created_by, created_at, updated_at FROM teams WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(Team::from))
}

pub async fn list_user_teams(pool: &SqlitePool, user_id: &str) -> Result<Vec<Team>, AuthError> {
    let rows = sqlx::query_as::<_, TeamRow>(
        r#"SELECT t.id, t.name, t.description, t.created_by, t.created_at, t.updated_at
           FROM teams t
           JOIN team_members tm ON t.id = tm.team_id
           WHERE tm.user_id = ?
           ORDER BY t.name ASC"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Team::from).collect())
}

pub async fn delete_team(pool: &SqlitePool, id: &str) -> Result<(), AuthError> {
    sqlx::query("DELETE FROM teams WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// ── Team Members ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub user_id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub role: String,
}

#[derive(Debug, FromRow)]
pub struct TeamMemberRow {
    pub user_id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub role_name: String,
}

pub async fn add_team_member(
    pool: &SqlitePool,
    team_id: &str,
    user_id: &str,
    role_id: &str,
) -> Result<(), AuthError> {
    sqlx::query(
        r#"INSERT OR REPLACE INTO team_members (team_id, user_id, role_id) VALUES (?, ?, ?)"#,
    )
    .bind(team_id)
    .bind(user_id)
    .bind(role_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_team_member(
    pool: &SqlitePool,
    team_id: &str,
    user_id: &str,
) -> Result<(), AuthError> {
    sqlx::query("DELETE FROM team_members WHERE team_id = ? AND user_id = ?")
        .bind(team_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_team_members(
    pool: &SqlitePool,
    team_id: &str,
) -> Result<Vec<TeamMember>, AuthError> {
    let rows = sqlx::query_as::<_, TeamMemberRow>(
        r#"SELECT u.id as user_id, u.email, u.display_name, r.name as role_name
           FROM team_members tm
           JOIN users u ON tm.user_id = u.id
           JOIN roles r ON tm.role_id = r.id
           WHERE tm.team_id = ?
           ORDER BY u.email ASC"#,
    )
    .bind(team_id)
    .fetch_all(pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| TeamMember {
            user_id: r.user_id,
            email: r.email,
            display_name: r.display_name,
            role: r.role_name,
        })
        .collect())
}

// ── Workspace Members ──

#[derive(Debug, Deserialize)]
pub struct AddWorkspaceMember {
    pub user_id: String,
    pub role: String,
}

pub async fn add_workspace_member(
    pool: &SqlitePool,
    workspace_id: &[u8],
    user_id: &str,
    role_id: &str,
) -> Result<(), AuthError> {
    sqlx::query(
        r#"INSERT OR REPLACE INTO workspace_members (workspace_id, user_id, role_id) VALUES (?, ?, ?)"#,
    )
    .bind(workspace_id)
    .bind(user_id)
    .bind(role_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_workspace_member(
    pool: &SqlitePool,
    workspace_id: &[u8],
    user_id: &str,
) -> Result<(), AuthError> {
    sqlx::query("DELETE FROM workspace_members WHERE workspace_id = ? AND user_id = ?")
        .bind(workspace_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_workspace_team_access(
    pool: &SqlitePool,
    workspace_id: &[u8],
    team_id: &str,
    role_id: &str,
) -> Result<(), AuthError> {
    sqlx::query(
        r#"INSERT OR REPLACE INTO workspace_team_access (workspace_id, team_id, role_id) VALUES (?, ?, ?)"#,
    )
    .bind(workspace_id)
    .bind(team_id)
    .bind(role_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_workspace_team_access(
    pool: &SqlitePool,
    workspace_id: &[u8],
    team_id: &str,
) -> Result<(), AuthError> {
    sqlx::query("DELETE FROM workspace_team_access WHERE workspace_id = ? AND team_id = ?")
        .bind(workspace_id)
        .bind(team_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Check if a user has access to a workspace (owner, individual, or team).
pub async fn check_workspace_access(
    pool: &SqlitePool,
    workspace_id: &[u8],
    user_id: &str,
) -> Result<bool, AuthError> {
    // Check individual membership
    let individual: Option<(String,)> = sqlx::query_as(
        "SELECT role_id FROM workspace_members WHERE workspace_id = ? AND user_id = ?",
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    if individual.is_some() {
        return Ok(true);
    }

    // Check team membership
    let team: Option<(String,)> = sqlx::query_as(
        r#"SELECT wta.role_id
           FROM workspace_team_access wta
           JOIN team_members tm ON wta.team_id = tm.team_id
           WHERE wta.workspace_id = ? AND tm.user_id = ?"#,
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(team.is_some())
}
