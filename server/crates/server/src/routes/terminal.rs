use axum::{
    Router,
    extract::{Query, State},
    extract::ws::WebSocketUpgrade,
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    DeploymentImpl,
    error::ApiError,
};

#[derive(Debug, Deserialize)]
pub struct TerminalQuery {
    pub workspace_id: Uuid,
    #[serde(default = "default_cols")]
    pub cols: u16,
    #[serde(default = "default_rows")]
    pub rows: u16,
}

fn default_cols() -> u16 {
    80
}

fn default_rows() -> u16 {
    24
}

pub async fn terminal_ws(
    _ws: WebSocketUpgrade,
    State(_deployment): State<DeploymentImpl>,
    Query(_query): Query<TerminalQuery>,
) -> Result<impl IntoResponse, ApiError> {
    Err::<axum::response::Response, _>(ApiError::BadRequest(
        "Terminal functionality is not available".to_string(),
    ))
}

pub fn router() -> Router<DeploymentImpl> {
    Router::new().route("/terminal/ws", get(terminal_ws))
}
