/// Account routes — legacy stubs kept for compatibility.
/// Most routes are now handled by user.rs (Better-Reload port).
/// Only routes NOT present in user.rs are kept here.
use axum::{
    Router,
    routing::get,
    response::IntoResponse,
};
use serde_json::json;

pub fn router() -> Router {
    Router::new()
        // These are not in user.rs
        .route("/presence/api/v1/_/:account_id/settings/subscriptions", get(presence_subscriptions))
        .route("/fortnite/api/game/v2/privacy/account/:account_id", get(privacy_account))
}

async fn presence_subscriptions() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn privacy_account() -> impl IntoResponse {
    axum::Json(json!([]))
}
