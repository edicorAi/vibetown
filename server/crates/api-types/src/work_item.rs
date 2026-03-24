use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Deserialize, TS)]
pub struct CreateWorkItemRequest {
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ListWorkItemsResponse {
    pub work_items: Vec<WorkItem>,
}
