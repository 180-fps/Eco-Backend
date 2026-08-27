use axum::{Router, routing::get, response::IntoResponse};
use serde_json::json;

pub fn router() -> Router {
    Router::new()
        .route("/lightswitch/api/service/Fortnite/status", get(fortnite_status))
        .route("/lightswitch/api/service/bulk/status", get(bulk_status))
}

// GET /lightswitch/api/service/Fortnite/status
async fn fortnite_status() -> impl IntoResponse {
    axum::Json(json!({
        "serviceInstanceId": "fortnite",
        "status": "UP",
        "message": "Fortnite is online",
        "maintenanceUri": null,
        "overrideCatalogIds": [
            "a7f138b2e51945ffbfdacc1af0541053"
        ],
        "allowedActions": [],
        "banned": false,
        "launcherInfoDTO": {
            "appName": "Fortnite",
            "catalogItemId": "4fe75bbc5a674f4f9b356b5c90567da5",
            "namespace": "fn"
        }
    }))
}

// GET /lightswitch/api/service/bulk/status
async fn bulk_status() -> impl IntoResponse {
    axum::Json(json!({
        "serviceInstanceId": "fortnite",
        "status": "UP",
        "message": "fortnite is up.",
        "maintenanceUri": null,
        "overrideCatalogIds": [
            "a7f138b2e51945ffbfdacc1af0541053"
        ],
        "allowedActions": [
            "PLAY",
            "DOWNLOAD"
        ],
        "banned": false,
        "launcherInfoDTO": {
            "appName": "Fortnite",
            "catalogItemId": "4fe75bbc5a674f4f9b356b5c90567da5",
            "namespace": "fn"
        }
    }))
}
