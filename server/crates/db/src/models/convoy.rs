use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
pub struct Convoy {
    pub id: String,
    pub name: String,
    pub status: String,
    pub formula: Option<String>,
    pub config_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateConvoy {
    pub id: String,
    pub name: String,
    pub status: Option<String>,
    pub formula: Option<String>,
    pub config_json: Option<String>,
}

impl Convoy {
    pub async fn create(pool: &SqlitePool, data: &CreateConvoy) -> Result<Self, sqlx::Error> {
        let status = data.status.as_deref().unwrap_or("active");
        let config_json = data.config_json.as_deref().unwrap_or("{}");

        sqlx::query_as::<_, Convoy>(
            r#"INSERT INTO convoys (id, name, status, formula, config_json)
               VALUES (?, ?, ?, ?, ?)
               RETURNING id, name, status, formula, config_json, created_at, updated_at"#,
        )
        .bind(&data.id)
        .bind(&data.name)
        .bind(status)
        .bind(&data.formula)
        .bind(config_json)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Convoy>(
            r#"SELECT id, name, status, formula, config_json, created_at, updated_at
               FROM convoys
               WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Convoy>(
            r#"SELECT id, name, status, formula, config_json, created_at, updated_at
               FROM convoys
               ORDER BY created_at DESC"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update_status(
        pool: &SqlitePool,
        id: &str,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE convoys SET status = ?, updated_at = datetime('now') WHERE id = ?"#,
        )
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM convoys WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
