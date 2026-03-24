use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DeploymentImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub priority: i32,
    pub assignee: Option<String>,
    pub labels: Vec<String>,
    pub parent_id: Option<String>,
    pub sort_order: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct WorkItemQuery {
    pub status: Option<String>,
    pub item_type: Option<String>,
    pub assignee: Option<String>,
    pub rig_id: Option<String>,
    pub project_id: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkItemRequest {
    pub project_id: Option<String>,
    pub rig_id: Option<String>,
    pub item_type: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub assignee: Option<String>,
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkItemRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub assignee: Option<String>,
    pub labels: Option<Vec<String>>,
    pub sort_order: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct WorkItemListResponse {
    pub items: Vec<WorkItem>,
    pub total: i32,
}

pub fn router(_deployment: &DeploymentImpl) -> Router<DeploymentImpl> {
    Router::new()
        .route("/work-items", get(list_work_items))
        .route("/work-items", post(create_work_item))
        .route("/work-items/{id}", get(get_work_item))
        .route("/work-items/{id}", patch(update_work_item))
        .route("/work-items/{id}", delete(delete_work_item))
}

async fn list_work_items(
    State(_deployment): State<DeploymentImpl>,
    Query(_query): Query<WorkItemQuery>,
) -> Json<WorkItemListResponse> {
    // Stub: will be replaced with DB query in Phase 4
    Json(WorkItemListResponse {
        items: vec![],
        total: 0,
    })
}

async fn create_work_item(
    State(_deployment): State<DeploymentImpl>,
    Json(req): Json<CreateWorkItemRequest>,
) -> Json<WorkItem> {
    Json(WorkItem {
        id: Uuid::new_v4().to_string(),
        project_id: req.project_id,
        rig_id: req.rig_id,
        convoy_id: None,
        prefix: None,
        item_type: req.item_type,
        title: req.title,
        description: req.description,
        status: "open".to_string(),
        priority: req.priority.unwrap_or(0),
        assignee: req.assignee,
        labels: req.labels.unwrap_or_default(),
        parent_id: None,
        sort_order: 0.0,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    })
}

async fn get_work_item(
    State(_deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Json<WorkItem> {
    Json(WorkItem {
        id,
        project_id: None,
        rig_id: None,
        convoy_id: None,
        prefix: None,
        item_type: "task".to_string(),
        title: "Sample work item".to_string(),
        description: None,
        status: "open".to_string(),
        priority: 0,
        assignee: None,
        labels: vec![],
        parent_id: None,
        sort_order: 0.0,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    })
}

async fn update_work_item(
    State(_deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
    Json(_req): Json<UpdateWorkItemRequest>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({"id": id, "updated": true}))
}

async fn delete_work_item(
    State(_deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({"id": id, "deleted": true}))
}
