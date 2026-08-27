use axum::{Router, routing::post, response::IntoResponse};
use serde_json::json;

pub fn router() -> Router {
    Router::new()
        .route("/datarouter/api/v1/public/data", post(data))
}

// POST /datarouter/api/v1/public/data
async fn data() -> impl IntoResponse {
    axum::Json(json!({
        "status": "OK",
        "code": 200
    }))
}
