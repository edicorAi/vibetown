use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use ts_rs::TS;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
pub struct MailMessage {
    pub id: String,
    pub from_addr: String,
    pub to_addr: Option<String>,
    pub subject: String,
    pub body: Option<String>,
    pub priority: String,
    pub message_type: String,
    pub delivery: String,
    pub thread_id: Option<String>,
    pub reply_to: Option<String>,
    pub queue: Option<String>,
    pub channel: Option<String>,
    pub claimed_by: Option<String>,
    pub claimed_at: Option<String>,
    pub pinned: bool,
    pub wisp: bool,
    pub read: bool,
    pub cc_json: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMailMessage {
    pub id: String,
    pub from_addr: String,
    pub to_addr: Option<String>,
    pub subject: String,
    pub body: Option<String>,
    pub priority: Option<String>,
    pub message_type: Option<String>,
    pub delivery: Option<String>,
    pub thread_id: Option<String>,
    pub reply_to: Option<String>,
    pub queue: Option<String>,
    pub channel: Option<String>,
    pub cc_json: Option<String>,
}

impl MailMessage {
    pub async fn create(
        pool: &SqlitePool,
        data: &CreateMailMessage,
    ) -> Result<Self, sqlx::Error> {
        let priority = data.priority.as_deref().unwrap_or("normal");
        let message_type = data.message_type.as_deref().unwrap_or("task");
        let delivery = data.delivery.as_deref().unwrap_or("queue");
        let cc_json = data.cc_json.as_deref().unwrap_or("[]");

        sqlx::query_as::<_, MailMessage>(
            r#"INSERT INTO mail_messages (id, from_addr, to_addr, subject, body, priority,
                   message_type, delivery, thread_id, reply_to, queue, channel, cc_json)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id, from_addr, to_addr, subject, body, priority, message_type,
                         delivery, thread_id, reply_to, queue, channel, claimed_by, claimed_at,
                         pinned, wisp, read, cc_json, created_at"#,
        )
        .bind(&data.id)
        .bind(&data.from_addr)
        .bind(&data.to_addr)
        .bind(&data.subject)
        .bind(&data.body)
        .bind(priority)
        .bind(message_type)
        .bind(delivery)
        .bind(&data.thread_id)
        .bind(&data.reply_to)
        .bind(&data.queue)
        .bind(&data.channel)
        .bind(cc_json)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, MailMessage>(
            r#"SELECT id, from_addr, to_addr, subject, body, priority, message_type,
                      delivery, thread_id, reply_to, queue, channel, claimed_by, claimed_at,
                      pinned, wisp, read, cc_json, created_at
               FROM mail_messages
               WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, MailMessage>(
            r#"SELECT id, from_addr, to_addr, subject, body, priority, message_type,
                      delivery, thread_id, reply_to, queue, channel, claimed_by, claimed_at,
                      pinned, wisp, read, cc_json, created_at
               FROM mail_messages
               ORDER BY created_at DESC"#,
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_queue(
        pool: &SqlitePool,
        queue: &str,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, MailMessage>(
            r#"SELECT id, from_addr, to_addr, subject, body, priority, message_type,
                      delivery, thread_id, reply_to, queue, channel, claimed_by, claimed_at,
                      pinned, wisp, read, cc_json, created_at
               FROM mail_messages
               WHERE queue = ?
               ORDER BY created_at DESC"#,
        )
        .bind(queue)
        .fetch_all(pool)
        .await
    }

    pub async fn mark_read(pool: &SqlitePool, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query(r#"UPDATE mail_messages SET read = 1 WHERE id = ?"#)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn claim(
        pool: &SqlitePool,
        id: &str,
        claimed_by: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"UPDATE mail_messages SET claimed_by = ?, claimed_at = datetime('now') WHERE id = ?"#,
        )
        .bind(claimed_by)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM mail_messages WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
