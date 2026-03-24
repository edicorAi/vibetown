use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Deserialize, TS)]
pub struct CreateMergeRequestRequest {
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ListMergeRequestsResponse {
    pub merge_requests: Vec<MergeRequest>,
}
