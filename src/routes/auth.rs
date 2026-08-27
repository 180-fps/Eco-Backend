use axum::{
    Router,
    routing::{post, delete},
    response::IntoResponse,
};
use serde_json::json;

pub fn router() -> Router {
    Router::new()
        .route("/account/api/oauth/token", post(oauth_token))
        .route("/account/api/oauth/verify", post(oauth_verify))
        .route("/account/api/oauth/sessions/kill", delete(sessions_kill))
        .route("/account/api/oauth/sessions/kill/:token", delete(sessions_kill_token))
}

fn token_response() -> serde_json::Value {
    json!({
        "access_token": "eg1~fortnite",
        "expires_in": 28800,
        "expires_at": "9999-12-02T01:12:01.100Z",
        "token_type": "bearer",
        "refresh_token": "eg1~fortnite",
        "refresh_expires": 86400,
        "refresh_expires_at": "9999-12-02T01:12:01.100Z",
        "account_id": "fortnite",
        "client_id": "fortnite",
        "internal_client": true,
        "client_service": "fortnite",
        "displayName": "fortnite",
        "app": "fortnite",
        "in_app_id": "fortnite",
        "device_id": "fortnite"
    })
}

// POST /account/api/oauth/token
async fn oauth_token() -> impl IntoResponse {
    axum::Json(token_response())
}

// POST /account/api/oauth/verify
async fn oauth_verify() -> impl IntoResponse {
    axum::Json(token_response())
}

// DELETE /account/api/oauth/sessions/kill
async fn sessions_kill() -> impl IntoResponse {
    axum::Json(json!({
        "status": "OK",
        "code": 200
    }))
}

// DELETE /account/api/oauth/sessions/kill/:token
async fn sessions_kill_token() -> impl IntoResponse {
    axum::Json(json!({
        "status": "OK",
        "code": 200
    }))
}
