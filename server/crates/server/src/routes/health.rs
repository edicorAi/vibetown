use axum::response::Json;
use serde::Serialize;
use utils::response::ApiResponse;

pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("OK".to_string()))
}

#[derive(Serialize)]
pub struct ReadyResponse {
    pub status: String,
    pub db: String,
    pub engine: String,
}

pub async fn ready_check() -> Json<ReadyResponse> {
    // TODO: Actually check DB connectivity and engine gRPC health
    Json(ReadyResponse {
        status: "ok".to_string(),
        db: "ok".to_string(),
        engine: "unknown".to_string(),
    })
}
