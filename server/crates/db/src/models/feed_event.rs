use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
pub struct FeedEvent {
    pub id: String,
    pub event_type: String,
    pub source: String,
    pub rig_id: Option<String>,
    pub agent_id: Option<String>,
    pub work_item_id: Option<String>,
    pub summary: String,
    pub details_json: String,
    pub severity: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFeedEvent {
    pub id: String,
    pub event_type: String,
    pub source: String,
    pub rig_id: Option<String>,
    pub agent_id: Option<String>,
    pub work_item_id: Option<String>,
    pub summary: String,
    pub details_json: Option<String>,
    pub severity: Option<String>,
}

impl FeedEvent {
    pub async fn create(pool: &SqlitePool, data: &CreateFeedEvent) -> Result<Self, sqlx::Error> {
        let details_json = data.details_json.as_deref().unwrap_or("{}");
        let severity = data.severity.as_deref().unwrap_or("info");

        sqlx::query_as::<_, FeedEvent>(
            r#"INSERT INTO feed_events (id, event_type, source, rig_id, agent_id,
                   work_item_id, summary, details_json, severity)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id, event_type, source, rig_id, agent_id, work_item_id,
                         summary, details_json, severity, created_at"#,
        )
        .bind(&data.id)
        .bind(&data.event_type)
        .bind(&data.source)
        .bind(&data.rig_id)
        .bind(&data.agent_id)
        .bind(&data.work_item_id)
        .bind(&data.summary)
        .bind(details_json)
        .bind(severity)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, FeedEvent>(
            r#"SELECT id, event_type, source, rig_id, agent_id, work_item_id,
                      summary, details_json, severity, created_at
               FROM feed_events
               WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn list(pool: &SqlitePool, limit: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, FeedEvent>(
            r#"SELECT id, event_type, source, rig_id, agent_id, work_item_id,
                      summary, details_json, severity, created_at
               FROM feed_events
               ORDER BY created_at DESC
               LIMIT ?"#,
        )
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_rig(
        pool: &SqlitePool,
        rig_id: &str,
        limit: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, FeedEvent>(
            r#"SELECT id, event_type, source, rig_id, agent_id, work_item_id,
                      summary, details_json, severity, created_at
               FROM feed_events
               WHERE rig_id = ?
               ORDER BY created_at DESC
               LIMIT ?"#,
        )
        .bind(rig_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM feed_events WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
