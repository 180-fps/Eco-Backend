use axum::{Router, routing::post, response::IntoResponse};
use serde_json::json;

pub fn router() -> Router {
    Router::new()
        .route("/fortnite/api/game/v2/profile/:account_id/client/:operation", post(mcp_operation))
}

// POST /fortnite/api/game/v2/profile/:accountId/client/:operation
async fn mcp_operation() -> impl IntoResponse {
    axum::Json(json!({
        "status": "OK",
        "code": 200
    }))
}
