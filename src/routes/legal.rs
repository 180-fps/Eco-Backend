use axum::{Router, routing::get, response::IntoResponse, http::StatusCode};

static EULA_JSON: &str = include_str!("../responses/eula/SharedAgreements.json");

pub fn router() -> Router {
    Router::new()
        .route("/eulatracking/api/shared/agreements/fn", get(eula_shared))
        .route("/eulatracking/api/public/agreements/fn/account/:account_id", get(eula_account))
}

async fn eula_shared() -> impl IntoResponse {
    let data: serde_json::Value = serde_json::from_str(EULA_JSON).unwrap_or(serde_json::json!({}));
    axum::Json(data)
}

async fn eula_account() -> StatusCode {
    StatusCode::NO_CONTENT
}
