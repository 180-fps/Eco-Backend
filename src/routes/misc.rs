/// Miscellaneous routes from Better-Reload's main.js
use axum::{
    Router,
    routing::{get, post},
    response::IntoResponse,
    extract::Path,
    http::{StatusCode, header},
};
use serde_json::json;

static DISCOVERY_API_ASSETS_JSON: &str = include_str!("../responses/Discovery/discovery_api_assets.json");
static CLOUDDIR_MANIFEST: &[u8] = include_bytes!("../responses/CloudDir/EcoBackend.manifest");
static CLOUDDIR_CHUNK: &[u8] = include_bytes!("../responses/CloudDir/EcoBackend.chunk");
static CLOUDDIR_INI: &[u8] = include_bytes!("../responses/CloudDir/Full.ini");

pub fn router() -> Router {
    Router::new()
        // Launcher / CloudDir
        .route("/launcher/api/public/assets/*path", get(launcher_assets))
        .route("/Builds/Fortnite/Content/CloudDir/:file", get(clouddir_file))
        // Arena events
        .route("/api/v1/events/Fortnite/download/:account_id", get(arena_events))
        .route("/api/v1/events/Fortnite/:event_id/history/:account_id", get(event_history))
        // Creative / discovery assets
        .route("/api/v1/assets/Fortnite/:a/:b", post(discovery_assets))
        // Game misc
        .route("/fortnite/api/game/v2/grant_access/:account_id", post(grant_access))
        .route("/fortnite/api/game/v2/events/tournamentandhistory/*path", get(tournament_history))
        .route("/waitingroom/api/waitingroom", get(waitingroom))
        .route("/fortnite/api/game/v2/chat/*path", post(chat_rooms))
        .route("/fortnite/api/feedback/*path", post(feedback))
}

async fn launcher_assets() -> impl IntoResponse {
    axum::Json(json!({
        "appName": "FortniteContentBuilds",
        "labelName": "EcoBackend",
        "buildVersion": "++Fortnite+Release-12.41-CL-11883027-Windows",
        "catalogItemId": "5cb97847cee34581afdbc445400e2f77",
        "expires": "9999-12-31T23:59:59.999Z",
        "items": {
            "MANIFEST": {
                "signature": "EcoBackend",
                "distribution": "https://ecobackend.ol.epicgames.com/",
                "path": "Builds/Fortnite/Content/CloudDir/EcoBackend.manifest",
                "hash": "55bb954f5596cadbe03693e1c06ca73368d427f3",
                "additionalDistributions": []
            },
            "CHUNKS": {
                "signature": "EcoBackend",
                "distribution": "https://ecobackend.ol.epicgames.com/",
                "path": "Builds/Fortnite/Content/CloudDir/EcoBackend.chunk",
                "additionalDistributions": []
            }
        },
        "assetId": "FortniteContentBuilds"
    }))
}

async fn clouddir_file(Path(file): Path<String>) -> impl IntoResponse {
    let octet = [(header::CONTENT_TYPE, "application/octet-stream")];
    if file.to_lowercase().ends_with(".manifest") {
        return (octet, CLOUDDIR_MANIFEST).into_response();
    }
    if file.to_lowercase().ends_with(".chunk") {
        return (octet, CLOUDDIR_CHUNK).into_response();
    }
    if file.to_lowercase().ends_with(".ini") {
        return (octet, CLOUDDIR_INI).into_response();
    }
    StatusCode::NOT_FOUND.into_response()
}

async fn arena_events(Path(account_id): Path<String>) -> impl IntoResponse {
    let mut events: serde_json::Value = serde_json::from_str(
        include_str!("../responses/eventlistactive.json")
    ).unwrap_or(json!({}));

    events["player"] = json!({
        "accountId": account_id,
        "gameId": "Fortnite",
        "persistentScores": { "Hype": 0 },
        "tokens": ["ARENA_S24_Division1"]
    });

    axum::Json(events)
}

async fn event_history() -> impl IntoResponse {
    axum::Json(json!({
        "events": [],
        "paging": { "count": 0, "total": 0 }
    }))
}

async fn discovery_assets(
    axum::extract::Json(body): axum::extract::Json<serde_json::Value>,
) -> impl IntoResponse {
    if body.get("FortCreativeDiscoverySurface").and_then(|v| v.as_i64()) == Some(0) {
        let assets: serde_json::Value = serde_json::from_str(DISCOVERY_API_ASSETS_JSON)
            .unwrap_or(json!({}));
        return axum::Json(assets).into_response();
    }
    axum::Json(json!({
        "FortCreativeDiscoverySurface": { "meta": { "promotion": 0 }, "assets": {} }
    })).into_response()
}

async fn grant_access() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn tournament_history() -> impl IntoResponse {
    axum::Json(json!({}))
}

async fn waitingroom() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn chat_rooms() -> impl IntoResponse {
    axum::Json(json!({}))
}

async fn feedback() -> StatusCode {
    StatusCode::OK
}
