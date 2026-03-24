use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
pub struct Formula {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub definition: String,
    pub rig_id: Option<String>,
    pub metadata_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFormula {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub definition: String,
    pub rig_id: Option<String>,
    pub metadata_json: Option<String>,
}

impl Formula {
    pub async fn create(pool: &SqlitePool, data: &CreateFormula) -> Result<Self, sqlx::Error> {
        let metadata_json = data.metadata_json.as_deref().unwrap_or("{}");

        sqlx::query_as::<_, Formula>(
            r#"INSERT INTO formulas (id, name, description, definition, rig_id, metadata_json)
               VALUES (?, ?, ?, ?, ?, ?)
               RETURNING id, name, description, definition, rig_id, metadata_json,
                         created_at, updated_at"#,
        )
        .bind(&data.id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(&data.definition)
        .bind(&data.rig_id)
        .bind(metadata_json)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Formula>(
            r#"SELECT id, name, description, definition, rig_id, metadata_json,
                      created_at, updated_at
               FROM formulas
               WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Formula>(
            r#"SELECT id, name, description, definition, rig_id, metadata_json,
                      created_at, updated_at
               FROM formulas
               ORDER BY created_at DESC"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_rig(pool: &SqlitePool, rig_id: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Formula>(
            r#"SELECT id, name, description, definition, rig_id, metadata_json,
                      created_at, updated_at
               FROM formulas
               WHERE rig_id = ?
               ORDER BY created_at DESC"#,
        )
        .bind(rig_id)
        .fetch_all(pool)
        .await
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM formulas WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
