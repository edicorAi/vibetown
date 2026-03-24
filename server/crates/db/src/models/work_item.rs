use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
pub struct WorkItem {
    pub id: String,
    pub project_id: Option<String>,
    pub rig_id: Option<String>,
    pub convoy_id: Option<String>,
    pub prefix: Option<String>,
    pub item_type: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: Option<i32>,
    pub assignee: Option<String>,
    pub labels_json: String,
    pub metadata_json: String,
    pub parent_id: Option<String>,
    pub thread_id: Option<String>,
    pub sort_order: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkItem {
    pub id: String,
    pub project_id: Option<String>,
    pub rig_id: Option<String>,
    pub convoy_id: Option<String>,
    pub prefix: Option<String>,
    pub item_type: String,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub assignee: Option<String>,
    pub labels_json: Option<String>,
    pub metadata_json: Option<String>,
    pub parent_id: Option<String>,
    pub thread_id: Option<String>,
    pub sort_order: Option<f64>,
}

impl WorkItem {
    pub async fn create(pool: &SqlitePool, data: &CreateWorkItem) -> Result<Self, sqlx::Error> {
        let status = data.status.as_deref().unwrap_or("open");
        let labels_json = data.labels_json.as_deref().unwrap_or("[]");
        let metadata_json = data.metadata_json.as_deref().unwrap_or("{}");
        let priority = data.priority.unwrap_or(0);
        let sort_order = data.sort_order.unwrap_or(0.0);

        sqlx::query_as::<_, WorkItem>(
            r#"INSERT INTO work_items (id, project_id, rig_id, convoy_id, prefix, item_type,
                   title, description, status, priority, assignee, labels_json, metadata_json,
                   parent_id, thread_id, sort_order)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id, project_id, rig_id, convoy_id, prefix, item_type,
                         title, description, status, priority, assignee, labels_json,
                         metadata_json, parent_id, thread_id, sort_order, created_at, updated_at"#,
        )
        .bind(&data.id)
        .bind(&data.project_id)
        .bind(&data.rig_id)
        .bind(&data.convoy_id)
        .bind(&data.prefix)
        .bind(&data.item_type)
        .bind(&data.title)
        .bind(&data.description)
        .bind(status)
        .bind(priority)
        .bind(&data.assignee)
        .bind(labels_json)
        .bind(metadata_json)
        .bind(&data.parent_id)
        .bind(&data.thread_id)
        .bind(sort_order)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, WorkItem>(
            r#"SELECT id, project_id, rig_id, convoy_id, prefix, item_type,
                      title, description, status, priority, assignee, labels_json,
                      metadata_json, parent_id, thread_id, sort_order, created_at, updated_at
               FROM work_items
               WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, WorkItem>(
            r#"SELECT id, project_id, rig_id, convoy_id, prefix, item_type,
                      title, description, status, priority, assignee, labels_json,
                      metadata_json, parent_id, thread_id, sort_order, created_at, updated_at
               FROM work_items
               ORDER BY sort_order ASC, created_at DESC"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_rig(pool: &SqlitePool, rig_id: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, WorkItem>(
            r#"SELECT id, project_id, rig_id, convoy_id, prefix, item_type,
                      title, description, status, priority, assignee, labels_json,
                      metadata_json, parent_id, thread_id, sort_order, created_at, updated_at
               FROM work_items
               WHERE rig_id = ?
               ORDER BY sort_order ASC, created_at DESC"#,
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
            r#"UPDATE work_items SET status = ?, updated_at = datetime('now') WHERE id = ?"#,
        )
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM work_items WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
