use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Deserialize, TS)]
pub struct CreateFeedEventRequest {
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ListFeedEventsResponse {
    pub events: Vec<FeedEvent>,
}
