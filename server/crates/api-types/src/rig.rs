use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct Rig {
    pub id: String,
    pub town_id: Option<String>,
    pub name: String,
    pub repo_url: Option<String>,
    pub beads_prefix: String,
    pub config_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, TS)]
pub struct CreateRigRequest {
    pub id: String,
    pub town_id: Option<String>,
    pub name: String,
    pub repo_url: Option<String>,
    pub beads_prefix: String,
    pub config_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ListRigsResponse {
    pub rigs: Vec<Rig>,
}
