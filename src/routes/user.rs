/// User / Account routes — port of Better-Reload's user.js
use axum::{
    Router,
    routing::{get, post},
    response::IntoResponse,
    extract::Path,
    http::{StatusCode, header},
};
use serde_json::json;

static SDK_JSON: &str = include_str!("../responses/sdkv1.json");
static EPIC_SETTINGS_JSON: &str = include_str!("../responses/epic-settings.json");

pub fn router() -> Router {
    Router::new()
        // Account lookup
        .route("/account/api/public/account", get(public_accounts))
        .route("/account/api/public/account/displayName/:display_name", get(account_by_display_name))
        .route("/account/api/public/account/:account_id", get(account_by_id))
        .route("/account/api/public/account/:account_id/externalAuths", get(external_auths))
        .route("/persona/api/public/account/lookup", get(persona_lookup))
        .route("/api/v1/search/:account_id", get(search))
        .route("/epic/id/v2/sdk/accounts", get(sdk_accounts))
        // SDK / settings
        .route("/sdk/v1/*path", get(sdk_v1))
        .route("/v1/epic-settings/public/users/:account_id/values", get(epic_settings))
        .route("/v1/epic-settings/public/users/:account_id/values", post(epic_settings))
        // SSO
        .route("/account/api/epicdomains/ssodomains", get(sso_domains))
        // Platform
        .route("/fortnite/api/game/v2/tryPlayOnPlatform/account/:account_id", post(try_play_on_platform))
        .route("/fortnite/api/game/v2/profileToken/verify/:account_id", post(profile_token_verify))
        // Game info
        .route("/fortnite/api/game/v2/enabled_features", get(enabled_features))
        .route("/fortnite/api/game/v2/br-inventory/account/:account_id", get(br_inventory))
        .route("/fortnite/api/game/v2/world/info", get(world_info))
        .route("/fortnite/api/game/v2/privacy/account/:account_id", get(privacy))
        .route("/content-controls/:account_id", get(content_controls))
        // Stats
        .route("/fortnite/api/statsv2/account/:account_id", get(stats_account))
        .route("/statsproxy/api/statsv2/account/:account_id", get(stats_account))
        .route("/fortnite/api/statsv2/query", post(stats_query))
        .route("/statsproxy/api/statsv2/query", post(stats_query))
        .route("/fortnite/api/stats/accountId/:account_id/bulk/window/alltime", get(stats_bulk))
        // Social
        .route("/socialban/api/public/v1/:account_id", get(socialban))
        .route("/presence/api/v1/_/:account_id/settings/subscriptions", get(presence_subscriptions))
        .route("/presence/api/v1/_/:account_id/last-online", get(last_online))
        // Misc
        .route("/fortnite/api/receipts/v1/account/:account_id/receipts", get(receipts))
        .route("/fortnite/api/game/v2/leaderboards/cohort/:account_id", get(leaderboards_cohort))
        .route("/fortnite/api/game/v2/twitch/:path", get(twitch))
        .route("/fortnite/api/matchmaking/session/findPlayer/:path", get(find_player))
        .route("/fortnite/api/game/v2/chat/:a/:b/:c/pc", post(chat_rooms))
        .route("/fortnite/api/game/v2/chat/:path/recommendGeneralChatRooms/pc", post(recommend_chat))
        .route("/fortnite/api/game/v2/events/v2/setSubgroup/:path", post(set_subgroup))
        .route("/api/v1/user/setting", post(user_setting))
        .route("/region", get(region))
        .route("/launcher/api/public/distributionpoints/", get(distribution_points))
}

async fn public_accounts() -> impl IntoResponse {
    axum::Json(json!([{
        "id": "eco",
        "displayName": "Eco",
        "externalAuths": {}
    }]))
}

async fn account_by_display_name(Path(display_name): Path<String>) -> impl IntoResponse {
    axum::Json(json!({
        "id": "eco",
        "displayName": display_name,
        "externalAuths": {}
    }))
}

async fn account_by_id(Path(account_id): Path<String>) -> impl IntoResponse {
    axum::Json(json!({
        "id": account_id,
        "displayName": "Eco",
        "name": "Eco",
        "email": "[hidden]@eco.dev",
        "failedLoginAttempts": 0,
        "lastLogin": chrono::Utc::now().to_rfc3339(),
        "numberOfDisplayNameChanges": 0,
        "ageGroup": "UNKNOWN",
        "headless": false,
        "country": "US",
        "lastName": "Server",
        "preferredLanguage": "en",
        "canUpdateDisplayName": false,
        "tfaEnabled": false,
        "emailVerified": true,
        "minorVerified": false,
        "minorExpected": false,
        "minorStatus": "NOT_MINOR",
        "cabinedMode": false,
        "hasHashedEmail": false
    }))
}

async fn external_auths() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn persona_lookup() -> impl IntoResponse {
    axum::Json(json!({ "id": "eco", "displayName": "Eco", "externalAuths": {} }))
}

async fn search() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn sdk_accounts() -> impl IntoResponse {
    axum::Json(json!([{
        "accountId": "eco",
        "displayName": "Eco",
        "preferredLanguage": "en",
        "cabinedMode": false,
        "empty": false
    }]))
}

async fn sdk_v1() -> impl IntoResponse {
    let data: serde_json::Value = serde_json::from_str(SDK_JSON).unwrap_or(json!({}));
    axum::Json(data)
}

async fn epic_settings() -> impl IntoResponse {
    let data: serde_json::Value = serde_json::from_str(EPIC_SETTINGS_JSON).unwrap_or(json!({}));
    axum::Json(data)
}

async fn sso_domains() -> impl IntoResponse {
    axum::Json(json!(["unrealengine.com", "unrealtournament.com", "fortnite.com", "epicgames.com"]))
}

async fn try_play_on_platform() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/plain")], "true")
}

async fn profile_token_verify() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn enabled_features() -> impl IntoResponse {
    axum::Json(json!(["LiveEvents", "BattleRoyale", "Creative", "SaveTheWorld"]))
}

async fn br_inventory() -> impl IntoResponse {
    axum::Json(json!({ "stash": { "globalcash": 0 } }))
}

async fn world_info() -> impl IntoResponse {
    axum::Json(json!({}))
}

async fn privacy() -> impl IntoResponse {
    axum::Json(json!({}))
}

async fn content_controls() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn stats_account(Path(account_id): Path<String>) -> impl IntoResponse {
    axum::Json(json!({ "startTime": 0, "endTime": 0, "stats": {}, "accountId": account_id }))
}

async fn stats_query() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn stats_bulk(Path(account_id): Path<String>) -> impl IntoResponse {
    axum::Json(json!({ "startTime": 0, "endTime": 0, "stats": {}, "accountId": account_id }))
}

async fn socialban() -> impl IntoResponse {
    axum::Json(json!({ "bans": [], "warnings": [] }))
}

async fn presence_subscriptions() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn last_online() -> impl IntoResponse {
    axum::Json(json!({}))
}

async fn receipts() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn leaderboards_cohort() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn twitch() -> StatusCode {
    StatusCode::OK
}

async fn find_player() -> StatusCode {
    StatusCode::OK
}

async fn chat_rooms() -> impl IntoResponse {
    axum::Json(json!({}))
}

async fn recommend_chat() -> impl IntoResponse {
    axum::Json(json!({}))
}

async fn set_subgroup() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn user_setting() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn region() -> impl IntoResponse {
    axum::Json(json!({
        "continent": { "code": "EU", "names": { "en": "Europe" } },
        "country": { "iso_code": "GB", "names": { "en": "United Kingdom" } }
    }))
}

async fn distribution_points() -> impl IntoResponse {
    axum::Json(json!({
        "distributions": [
            "https://download.epicgames.com/",
            "https://download2.epicgames.com/",
            "https://download3.epicgames.com/",
            "https://download4.epicgames.com/",
            "https://epicgames-download1.akamaized.net/"
        ]
    }))
}
