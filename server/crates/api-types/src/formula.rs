use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct Formula {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub definition: String,
    pub rig_id: Option<String>,
    pub metadata_json: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, TS)]
pub struct CreateFormulaRequest {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub definition: String,
    pub rig_id: Option<String>,
    pub metadata_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ListFormulasResponse {
    pub formulas: Vec<Formula>,
}
