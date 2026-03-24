use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DeploymentImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailMessage {
    pub id: String,
    pub from_addr: String,
    pub to_addr: String,
    pub subject: String,
    pub body: String,
    pub priority: String,
    pub message_type: String,
    pub thread_id: Option<String>,
    pub read: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct InboxQuery {
    pub to_addr: Option<String>,
    pub unread_only: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct InboxResponse {
    pub messages: Vec<MailMessage>,
    pub total: i32,
}

#[derive(Debug, Deserialize)]
pub struct SendMailRequest {
    pub from_addr: String,
    pub to_addr: String,
    pub subject: String,
    pub body: String,
    pub priority: Option<String>,
    pub message_type: Option<String>,
    pub thread_id: Option<String>,
}

pub fn router(_deployment: &DeploymentImpl) -> Router<DeploymentImpl> {
    Router::new()
        .route("/mail/inbox", get(get_inbox))
        .route("/mail/send", post(send_mail))
        .route("/mail/{id}/read", post(mark_read))
}

async fn get_inbox(
    State(_deployment): State<DeploymentImpl>,
    Query(_query): Query<InboxQuery>,
) -> Json<InboxResponse> {
    Json(InboxResponse {
        messages: vec![],
        total: 0,
    })
}

async fn send_mail(
    State(_deployment): State<DeploymentImpl>,
    Json(req): Json<SendMailRequest>,
) -> Json<MailMessage> {
    Json(MailMessage {
        id: Uuid::new_v4().to_string(),
        from_addr: req.from_addr,
        to_addr: req.to_addr,
        subject: req.subject,
        body: req.body,
        priority: req.priority.unwrap_or_else(|| "normal".to_string()),
        message_type: req.message_type.unwrap_or_else(|| "task".to_string()),
        thread_id: req.thread_id,
        read: false,
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

async fn mark_read(
    State(_deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({"id": id, "read": true}))
}
