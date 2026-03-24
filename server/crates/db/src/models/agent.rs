use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub role: String,
    pub rig_id: Option<String>,
    pub status: String,
    pub runtime: Option<String>,
    pub config_json: String,
    pub last_activity_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAgent {
    pub id: String,
    pub name: String,
    pub role: String,
    pub rig_id: Option<String>,
    pub status: Option<String>,
    pub runtime: Option<String>,
    pub config_json: Option<String>,
}

impl Agent {
    pub async fn create(pool: &SqlitePool, data: &CreateAgent) -> Result<Self, sqlx::Error> {
        let status = data.status.as_deref().unwrap_or("idle");
        let config_json = data.config_json.as_deref().unwrap_or("{}");

        sqlx::query_as::<_, Agent>(
            r#"INSERT INTO agents (id, name, role, rig_id, status, runtime, config_json)
               VALUES (?, ?, ?, ?, ?, ?, ?)
               RETURNING id, name, role, rig_id, status, runtime, config_json,
                         last_activity_at, created_at, updated_at"#,
        )
        .bind(&data.id)
        .bind(&data.name)
        .bind(&data.role)
        .bind(&data.rig_id)
        .bind(status)
        .bind(&data.runtime)
        .bind(config_json)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Agent>(
            r#"SELECT id, name, role, rig_id, status, runtime, config_json,
                      last_activity_at, created_at, updated_at
               FROM agents
               WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Agent>(
            r#"SELECT id, name, role, rig_id, status, runtime, config_json,
                      last_activity_at, created_at, updated_at
               FROM agents
               ORDER BY created_at DESC"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_rig(pool: &SqlitePool, rig_id: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Agent>(
            r#"SELECT id, name, role, rig_id, status, runtime, config_json,
                      last_activity_at, created_at, updated_at
               FROM agents
               WHERE rig_id = ?
               ORDER BY created_at DESC"#,
        )
        .bind(rig_id)
        .fetch_all(pool)
        .await
    }

    pub async fn update_status(
        pool: &SqlitePool,
        id: &str,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE agents SET status = ?, last_activity_at = datetime('now'),
               updated_at = datetime('now') WHERE id = ?"#,
        )
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM agents WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
