use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct Town {
    pub id: String,
    pub name: String,
    pub owner: Option<String>,
    pub config_json: String,
    pub settings_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, TS)]
pub struct CreateTownRequest {
    pub id: String,
    pub name: String,
    pub owner: Option<String>,
    pub config_json: Option<String>,
    pub settings_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ListTownsResponse {
    pub towns: Vec<Town>,
}
