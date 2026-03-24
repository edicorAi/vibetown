use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
pub struct Town {
    pub id: String,
    pub name: String,
    pub owner: Option<String>,
    pub config_json: String,
    pub settings_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTown {
    pub id: String,
    pub name: String,
    pub owner: Option<String>,
    pub config_json: Option<String>,
    pub settings_json: Option<String>,
}

impl Town {
    pub async fn create(pool: &SqlitePool, data: &CreateTown) -> Result<Self, sqlx::Error> {
        let config_json = data.config_json.as_deref().unwrap_or("{}");
        let settings_json = data.settings_json.as_deref().unwrap_or("{}");

        sqlx::query_as::<_, Town>(
            r#"INSERT INTO towns (id, name, owner, config_json, settings_json)
               VALUES (?, ?, ?, ?, ?)
               RETURNING id, name, owner, config_json, settings_json, created_at, updated_at"#,
        )
        .bind(&data.id)
        .bind(&data.name)
        .bind(&data.owner)
        .bind(config_json)
        .bind(settings_json)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Town>(
            r#"SELECT id, name, owner, config_json, settings_json, created_at, updated_at
               FROM towns
               WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Town>(
            r#"SELECT id, name, owner, config_json, settings_json, created_at, updated_at
               FROM towns
               ORDER BY created_at DESC"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update(
        pool: &SqlitePool,
        id: &str,
        name: Option<&str>,
        config_json: Option<&str>,
        settings_json: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE towns SET
                name = COALESCE(?, name),
                config_json = COALESCE(?, config_json),
                settings_json = COALESCE(?, settings_json),
                updated_at = datetime('now')
            WHERE id = ?"#,
        )
        .bind(name)
        .bind(config_json)
        .bind(settings_json)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM towns WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
