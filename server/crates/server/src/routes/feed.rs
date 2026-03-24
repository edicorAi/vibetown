use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use serde::{Deserialize, Serialize};

use crate::DeploymentImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct FeedQuery {
    pub rig_id: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct FeedResponse {
    pub events: Vec<FeedEvent>,
    pub total: i32,
}

pub fn router(_deployment: &DeploymentImpl) -> Router<DeploymentImpl> {
    Router::new()
        .route("/feed/events", get(get_recent_events))
}

async fn get_recent_events(
    State(_deployment): State<DeploymentImpl>,
    Query(query): Query<FeedQuery>,
) -> Json<FeedResponse> {
    let _limit = query.limit.unwrap_or(50);
    // Stub: return empty feed. Will be replaced with DB query + gRPC bridge.
    Json(FeedResponse {
        events: vec![],
        total: 0,
    })
}
