use chrono::{DateTime, Duration, Utc};
use rand::RngCore;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use crate::errors::AuthError;
use crate::models::{UserSession, UserSessionRow};

/// Generate a cryptographically random 256-bit session token.
pub fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// SHA-256 hash of a session token for storage.
pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
            use std::fmt::Write;
            write!(s, "{b:02x}").unwrap();
            s
        })
}

mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        super::hex_encode(bytes.as_ref())
    }
}

/// Create a new session in the database and return the raw token.
pub async fn create_session(
    pool: &SqlitePool,
    user_id: &str,
    ttl_seconds: i64,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
) -> Result<(String, UserSession), AuthError> {
    let token = generate_token();
    let token_hash = hash_token(&token);
    let session_id = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::seconds(ttl_seconds);
    let expires_at_str = expires_at.to_rfc3339();

    sqlx::query(
        r#"INSERT INTO user_sessions (id, user_id, token_hash, expires_at, ip_address, user_agent)
           VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&session_id)
    .bind(user_id)
    .bind(&token_hash)
    .bind(&expires_at_str)
    .bind(ip_address)
    .bind(user_agent)
    .execute(pool)
    .await?;

    let session = UserSession {
        id: session_id,
        user_id: user_id.to_string(),
        expires_at,
        revoked: false,
    };

    Ok((token, session))
}

/// Validate a session token. Returns the session if valid.
pub async fn validate_session(
    pool: &SqlitePool,
    token: &str,
) -> Result<UserSession, AuthError> {
    let token_hash = hash_token(token);

    let row = sqlx::query_as::<_, UserSessionRow>(
        r#"SELECT id, user_id, token_hash, expires_at, last_used_at, revoked
           FROM user_sessions
           WHERE token_hash = ?"#,
    )
    .bind(&token_hash)
    .fetch_optional(pool)
    .await?
    .ok_or(AuthError::Unauthorized)?;

    if row.revoked != 0 {
        return Err(AuthError::SessionRevoked);
    }

    let expires_at = DateTime::parse_from_rfc3339(&row.expires_at)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| AuthError::Internal(format!("Invalid expiry date: {e}")))?;

    if Utc::now() > expires_at {
        return Err(AuthError::SessionExpired);
    }

    // Update last_used_at (rolling session)
    let now = Utc::now().to_rfc3339();
    sqlx::query("UPDATE user_sessions SET last_used_at = ? WHERE id = ?")
        .bind(&now)
        .bind(&row.id)
        .execute(pool)
        .await?;

    Ok(UserSession {
        id: row.id,
        user_id: row.user_id,
        expires_at,
        revoked: false,
    })
}

/// Revoke a session by ID.
pub async fn revoke_session(pool: &SqlitePool, session_id: &str) -> Result<(), AuthError> {
    sqlx::query("UPDATE user_sessions SET revoked = 1 WHERE id = ?")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Revoke a session by token hash.
pub async fn revoke_session_by_token(pool: &SqlitePool, token: &str) -> Result<(), AuthError> {
    let token_hash = hash_token(token);
    sqlx::query("UPDATE user_sessions SET revoked = 1 WHERE token_hash = ?")
        .bind(&token_hash)
        .execute(pool)
        .await?;
    Ok(())
}

/// Revoke all sessions for a user.
pub async fn revoke_all_user_sessions(pool: &SqlitePool, user_id: &str) -> Result<(), AuthError> {
    sqlx::query("UPDATE user_sessions SET revoked = 1 WHERE user_id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Clean up expired sessions.
pub async fn cleanup_expired_sessions(pool: &SqlitePool) -> Result<u64, AuthError> {
    let now = Utc::now().to_rfc3339();
    let result = sqlx::query("DELETE FROM user_sessions WHERE expires_at < ? OR revoked = 1")
        .bind(&now)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
