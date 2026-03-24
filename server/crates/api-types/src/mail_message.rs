use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Deserialize, TS)]
pub struct CreateMailMessageRequest {
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ListMailMessagesResponse {
    pub messages: Vec<MailMessage>,
}
