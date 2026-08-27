use axum::{
    Router,
    routing::{get, post},
    response::IntoResponse,
    extract::{Path, Extension},
    http::StatusCode,
};
use serde_json::json;
use std::sync::Arc;
use crate::config::Config;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/fortnite/api/game/v2/matchmakingservice/ticket/player/:account_id", get(matchmaking_ticket))
        .route("/fortnite/api/game/v2/matchmaking/account/:account_id/session/:session_id", get(matchmaking_session_account))
        .route("/fortnite/api/matchmaking/session/:session_id", get(matchmaking_session))
        .route("/fortnite/api/matchmaking/session/:session_id/join", post(matchmaking_join))
        .route("/fortnite/api/matchmaking/session/matchMakingRequest", post(matchmaking_request))
}

// GET /fortnite/api/game/v2/matchmakingservice/ticket/player/:accountId
async fn matchmaking_ticket(
    Path(account_id): Path<String>,
    Extension(config): Extension<Arc<Config>>,
) -> impl IntoResponse {
    let matchmaker_ip = &config.matchmaker_ip;
    let ws_url = if matchmaker_ip.starts_with("ws") {
        matchmaker_ip.clone()
    } else {
        format!("ws://{}", matchmaker_ip)
    };

    axum::Json(json!({
        "serviceUrl": ws_url,
        "ticketType": "mms-player",
        "payload": account_id,
        "signature": "account"
    }))
}

// GET /fortnite/api/game/v2/matchmaking/account/:accountId/session/:sessionId
async fn matchmaking_session_account(
    Path((account_id, session_id)): Path<(String, String)>,
) -> impl IntoResponse {
    axum::Json(json!({
        "accountId": account_id,
        "sessionId": session_id,
        "key": "none"
    }))
}

// GET /fortnite/api/matchmaking/session/:sessionId
async fn matchmaking_session(
    Path(session_id): Path<String>,
    Extension(config): Extension<Arc<Config>>,
) -> impl IntoResponse {
    // Use first configured game server or fallback
    let (server_ip, server_port, playlist) = config
        .game_servers
        .first()
        .and_then(|s| {
            let parts: Vec<&str> = s.splitn(3, ':').collect();
            if parts.len() == 3 {
                Some((parts[0].to_string(), parts[1].to_string(), parts[2].to_string()))
            } else {
                None
            }
        })
        .unwrap_or_else(|| ("127.0.0.1".into(), "7777".into(), "playlist_defaultsolo".into()));

    let session_key = Uuid::new_v4().to_string().replace('-', "").to_uppercase();
    let owner_id = Uuid::new_v4().to_string().replace('-', "").to_uppercase();

    axum::Json(json!({
        "id": session_id,
        "ownerId": owner_id,
        "ownerName": "[DS]fortnite-liveeugcec1c2e30ubrcore0a-z8hj-1968",
        "serverName": "[DS]fortnite-liveeugcec1c2e30ubrcore0a-z8hj-1968",
        "serverAddress": server_ip,
        "serverPort": server_port.parse::<u16>().unwrap_or(7777),
        "maxPublicPlayers": 220,
        "openPublicPlayers": 175,
        "maxPrivatePlayers": 0,
        "openPrivatePlayers": 0,
        "attributes": {
            "REGION_s": "EU",
            "GAMEMODE_s": "FORTATHENA",
            "ALLOWBROADCASTING_b": true,
            "SUBREGION_s": "GB",
            "DCID_s": "FORTNITE-LIVEEUGCEC1C2E30UBRCORE0A-14840880",
            "tenant_s": "Fortnite",
            "MATCHMAKINGPOOL_s": "Any",
            "STORMSHIELDDEFENSETYPE_i": 0,
            "HOTFIXVERSION_i": 0,
            "PLAYLISTNAME_s": playlist,
            "SESSIONKEY_s": session_key,
            "TENANT_s": "Fortnite",
            "BEACONPORT_i": 15009
        },
        "publicPlayers": [],
        "privatePlayers": [],
        "totalPlayers": 45,
        "allowJoinInProgress": false,
        "shouldAdvertise": false,
        "isDedicated": false,
        "usesStats": false,
        "allowInvites": false,
        "usesPresence": false,
        "allowJoinViaPresence": true,
        "allowJoinViaPresenceFriendsOnly": false,
        "buildUniqueId": "0",
        "lastUpdated": chrono::Utc::now().to_rfc3339(),
        "started": false
    }))
}

// POST /fortnite/api/matchmaking/session/:sessionId/join
async fn matchmaking_join() -> StatusCode {
    StatusCode::NO_CONTENT
}

// POST /fortnite/api/matchmaking/session/matchMakingRequest
async fn matchmaking_request() -> impl IntoResponse {
    axum::Json(json!([]))
}
