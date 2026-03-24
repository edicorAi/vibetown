use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub role: String,
    pub rig_id: Option<String>,
    pub status: String,
    pub runtime: Option<String>,
    pub config_json: String,
    pub last_activity_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, TS)]
pub struct CreateAgentRequest {
    pub id: String,
    pub name: String,
    pub role: String,
    pub rig_id: Option<String>,
    pub status: Option<String>,
    pub runtime: Option<String>,
    pub config_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ListAgentsResponse {
    pub agents: Vec<Agent>,
}
