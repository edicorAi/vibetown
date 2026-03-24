use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DeploymentImpl;

// ─── Types ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Town {
    pub id: String,
    pub name: String,
    pub owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rig {
    pub id: String,
    pub town_id: String,
    pub name: String,
    pub repo_url: String,
    pub beads_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub role: String,
    pub rig_id: String,
    pub status: String,
    pub runtime: String,
    pub last_activity_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Convoy {
    pub id: String,
    pub name: String,
    pub status: String,
    pub formula: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequestEntry {
    pub id: String,
    pub branch: String,
    pub target_branch: String,
    pub status: String,
    pub agent_id: String,
    pub pr_url: String,
    pub queued_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTownRequest {
    pub name: String,
    pub owner: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRigRequest {
    pub town_id: String,
    pub name: String,
    pub repo_url: String,
    pub beads_prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct SpawnAgentRequest {
    pub name: String,
    pub role: String,
    pub rig_id: String,
    pub runtime: String,
}

#[derive(Debug, Deserialize)]
pub struct QueueMergeRequest {
    pub rig_id: String,
    pub branch: String,
    pub target_branch: String,
    pub agent_id: String,
}

// ─── Routes ──────────────────────────────────────────────────────────────────

pub fn router(_deployment: &DeploymentImpl) -> Router<DeploymentImpl> {
    Router::new()
        // Town
        .route("/orchestration/town", get(get_town))
        .route("/orchestration/town", post(create_town))
        // Rigs
        .route("/orchestration/rigs", get(list_rigs))
        .route("/orchestration/rigs", post(create_rig))
        // Agents
        .route("/orchestration/agents", get(list_agents))
        .route("/orchestration/agents/spawn", post(spawn_agent))
        .route("/orchestration/agents/{id}/kill", post(kill_agent))
        .route("/orchestration/agents/{id}", get(get_agent))
        // Convoys
        .route("/orchestration/convoys", get(list_convoys))
        .route("/orchestration/convoys/{id}", get(get_convoy))
        // Merge Queue
        .route("/orchestration/merge-queue", get(get_merge_queue))
        .route("/orchestration/merge-queue", post(queue_merge))
}

// ─── Handlers (stub implementations returning mock data) ─────────────────────
// These will be replaced with gRPC calls to the engine in Phase 1 integration.

async fn get_town(State(_deployment): State<DeploymentImpl>) -> Json<Town> {
    Json(Town {
        id: Uuid::new_v4().to_string(),
        name: "Default Town".to_string(),
        owner: "admin".to_string(),
    })
}

async fn create_town(
    State(_deployment): State<DeploymentImpl>,
    Json(req): Json<CreateTownRequest>,
) -> Json<Town> {
    Json(Town {
        id: Uuid::new_v4().to_string(),
        name: req.name,
        owner: req.owner,
    })
}

async fn list_rigs(State(_deployment): State<DeploymentImpl>) -> Json<Vec<Rig>> {
    Json(vec![Rig {
        id: Uuid::new_v4().to_string(),
        town_id: "default".to_string(),
        name: "vibetown".to_string(),
        repo_url: "https://github.com/edicorai/vibetown".to_string(),
        beads_prefix: "vt-".to_string(),
    }])
}

async fn create_rig(
    State(_deployment): State<DeploymentImpl>,
    Json(req): Json<CreateRigRequest>,
) -> Json<Rig> {
    Json(Rig {
        id: Uuid::new_v4().to_string(),
        town_id: req.town_id,
        name: req.name,
        repo_url: req.repo_url,
        beads_prefix: req.beads_prefix,
    })
}

async fn list_agents(State(_deployment): State<DeploymentImpl>) -> Json<Vec<Agent>> {
    Json(vec![])
}

async fn spawn_agent(
    State(_deployment): State<DeploymentImpl>,
    Json(req): Json<SpawnAgentRequest>,
) -> Json<Agent> {
    Json(Agent {
        id: Uuid::new_v4().to_string(),
        name: req.name,
        role: req.role,
        rig_id: req.rig_id,
        status: "idle".to_string(),
        runtime: req.runtime,
        last_activity_at: None,
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

async fn kill_agent(
    State(_deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({"id": id, "status": "killed"}))
}

async fn get_agent(
    State(_deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Json<Agent> {
    Json(Agent {
        id,
        name: "agent-1".to_string(),
        role: "polecat".to_string(),
        rig_id: "default".to_string(),
        status: "idle".to_string(),
        runtime: "claude".to_string(),
        last_activity_at: None,
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

async fn list_convoys(State(_deployment): State<DeploymentImpl>) -> Json<Vec<Convoy>> {
    Json(vec![])
}

async fn get_convoy(
    State(_deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Json<Convoy> {
    Json(Convoy {
        id,
        name: "convoy-1".to_string(),
        status: "active".to_string(),
        formula: "".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    })
}

async fn get_merge_queue(
    State(_deployment): State<DeploymentImpl>,
) -> Json<Vec<MergeRequestEntry>> {
    Json(vec![])
}

async fn queue_merge(
    State(_deployment): State<DeploymentImpl>,
    Json(req): Json<QueueMergeRequest>,
) -> Json<MergeRequestEntry> {
    Json(MergeRequestEntry {
        id: Uuid::new_v4().to_string(),
        branch: req.branch,
        target_branch: req.target_branch,
        status: "pending".to_string(),
        agent_id: req.agent_id,
        pr_url: "".to_string(),
        queued_at: chrono::Utc::now().to_rfc3339(),
    })
}
