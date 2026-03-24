use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
pub struct Rig {
    pub id: String,
    pub town_id: Option<String>,
    pub name: String,
    pub repo_url: Option<String>,
    pub beads_prefix: String,
    pub config_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRig {
    pub id: String,
    pub town_id: Option<String>,
    pub name: String,
    pub repo_url: Option<String>,
    pub beads_prefix: String,
    pub config_json: Option<String>,
}

impl Rig {
    pub async fn create(pool: &SqlitePool, data: &CreateRig) -> Result<Self, sqlx::Error> {
        let config_json = data.config_json.as_deref().unwrap_or("{}");

        sqlx::query_as::<_, Rig>(
            r#"INSERT INTO rigs (id, town_id, name, repo_url, beads_prefix, config_json)
               VALUES (?, ?, ?, ?, ?, ?)
               RETURNING id, town_id, name, repo_url, beads_prefix, config_json, created_at, updated_at"#,
        )
        .bind(&data.id)
        .bind(&data.town_id)
        .bind(&data.name)
        .bind(&data.repo_url)
        .bind(&data.beads_prefix)
        .bind(config_json)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Rig>(
            r#"SELECT id, town_id, name, repo_url, beads_prefix, config_json, created_at, updated_at
               FROM rigs
               WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Rig>(
            r#"SELECT id, town_id, name, repo_url, beads_prefix, config_json, created_at, updated_at
               FROM rigs
               ORDER BY created_at DESC"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_town(pool: &SqlitePool, town_id: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Rig>(
            r#"SELECT id, town_id, name, repo_url, beads_prefix, config_json, created_at, updated_at
               FROM rigs
               WHERE town_id = ?
               ORDER BY created_at DESC"#,
        )
        .bind(town_id)
        .fetch_all(pool)
        .await
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM rigs WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
