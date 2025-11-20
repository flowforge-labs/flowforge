use axum::{Json};
use serde_json::json;

pub async fn reload_logs_handler() -> Json<serde_json::Value> {
    // TODO: impl real log handling logic later

    Json(json!({
        "status": "ok",
        "message": "Reload endpoint reched"
    }))
}