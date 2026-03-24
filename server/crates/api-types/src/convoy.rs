use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct Convoy {
    pub id: String,
    pub name: String,
    pub status: String,
    pub formula: Option<String>,
    pub config_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, TS)]
pub struct CreateConvoyRequest {
    pub id: String,
    pub name: String,
    pub status: Option<String>,
    pub formula: Option<String>,
    pub config_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ListConvoysResponse {
    pub convoys: Vec<Convoy>,
}
