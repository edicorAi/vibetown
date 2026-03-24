use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
pub struct MergeRequest {
    pub id: String,
    pub work_item_id: Option<String>,
    pub rig_id: Option<String>,
    pub branch: String,
    pub target_branch: String,
    pub status: String,
    pub agent_id: Option<String>,
    pub pr_url: Option<String>,
    pub metadata_json: String,
    pub queued_at: String,
    pub merged_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMergeRequest {
    pub id: String,
    pub work_item_id: Option<String>,
    pub rig_id: Option<String>,
    pub branch: String,
    pub target_branch: Option<String>,
    pub status: Option<String>,
    pub agent_id: Option<String>,
    pub pr_url: Option<String>,
    pub metadata_json: Option<String>,
}

impl MergeRequest {
    pub async fn create(
        pool: &SqlitePool,
        data: &CreateMergeRequest,
    ) -> Result<Self, sqlx::Error> {
        let target_branch = data.target_branch.as_deref().unwrap_or("main");
        let status = data.status.as_deref().unwrap_or("pending");
        let metadata_json = data.metadata_json.as_deref().unwrap_or("{}");

        sqlx::query_as::<_, MergeRequest>(
            r#"INSERT INTO merge_requests (id, work_item_id, rig_id, branch, target_branch,
                   status, agent_id, pr_url, metadata_json)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id, work_item_id, rig_id, branch, target_branch, status, agent_id,
                         pr_url, metadata_json, queued_at, merged_at, created_at, updated_at"#,
        )
        .bind(&data.id)
        .bind(&data.work_item_id)
        .bind(&data.rig_id)
        .bind(&data.branch)
        .bind(target_branch)
        .bind(status)
        .bind(&data.agent_id)
        .bind(&data.pr_url)
        .bind(metadata_json)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, MergeRequest>(
            r#"SELECT id, work_item_id, rig_id, branch, target_branch, status, agent_id,
                      pr_url, metadata_json, queued_at, merged_at, created_at, updated_at
               FROM merge_requests
               WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, MergeRequest>(
            r#"SELECT id, work_item_id, rig_id, branch, target_branch, status, agent_id,
                      pr_url, metadata_json, queued_at, merged_at, created_at, updated_at
               FROM merge_requests
               ORDER BY created_at DESC"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_rig(pool: &SqlitePool, rig_id: &str) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, MergeRequest>(
            r#"SELECT id, work_item_id, rig_id, branch, target_branch, status, agent_id,
                      pr_url, metadata_json, queued_at, merged_at, created_at, updated_at
               FROM merge_requests
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
        let merged_at_clause = if status == "merged" {
            ", merged_at = datetime('now')"
        } else {
            ""
        };
        let query = format!(
            "UPDATE merge_requests SET status = ?{}, updated_at = datetime('now') WHERE id = ?",
            merged_at_clause
        );
        sqlx::query(&query)
            .bind(status)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM merge_requests WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
