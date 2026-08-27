use axum::{Router, routing::{get, post}, response::IntoResponse, extract::Path};
use serde_json::json;

static DISCOVERY_JSON: &str = include_str!("../responses/Discovery/discovery_frontend.json");

pub fn router() -> Router {
    Router::new()
        .route("/fortnite/api/discovery/accessToken/:branch", get(discovery_access_token))
        .route("/links/api/fn/mnemonic", post(mnemonic_list))
        .route("/links/api/fn/mnemonic/:playlist/related", get(mnemonic_related))
        .route("/links/api/fn/mnemonic/*path", get(mnemonic_get))
}

async fn discovery_access_token(Path(branch): Path<String>) -> impl IntoResponse {
    axum::Json(json!({
        "branchName": branch,
        "appId": "Fortnite",
        "token": "ecobackendtoken"
    }))
}

async fn mnemonic_list() -> impl IntoResponse {
    let discovery: serde_json::Value = serde_json::from_str(DISCOVERY_JSON).unwrap_or(json!({}));
    let empty = vec![];
    let results = discovery
        .get("Panels")
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("Pages"))
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("results"))
        .and_then(|r| r.as_array())
        .unwrap_or(&empty);

    let mnemonics: Vec<&serde_json::Value> = results
        .iter()
        .filter_map(|r| r.get("linkData"))
        .collect();

    axum::Json(json!(mnemonics))
}

async fn mnemonic_related(Path(playlist): Path<String>) -> impl IntoResponse {
    let discovery: serde_json::Value = serde_json::from_str(DISCOVERY_JSON).unwrap_or(json!({}));
    let empty = vec![];
    let results = discovery
        .get("Panels")
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("Pages"))
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("results"))
        .and_then(|r| r.as_array())
        .unwrap_or(&empty);

    let mut links = json!({});
    for result in results {
        if let Some(link_data) = result.get("linkData") {
            if link_data.get("mnemonic").and_then(|m| m.as_str()) == Some(&playlist) {
                links[&playlist] = link_data.clone();
            }
        }
    }

    axum::Json(json!({
        "parentLinks": [],
        "links": links
    }))
}

async fn mnemonic_get(Path(path): Path<String>) -> impl IntoResponse {
    let mnemonic = path.split('/').last().unwrap_or("");
    let discovery: serde_json::Value = serde_json::from_str(DISCOVERY_JSON).unwrap_or(json!({}));
    let empty = vec![];
    let results = discovery
        .get("Panels")
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("Pages"))
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("results"))
        .and_then(|r| r.as_array())
        .unwrap_or(&empty);

    for result in results {
        if let Some(link_data) = result.get("linkData") {
            if link_data.get("mnemonic").and_then(|m| m.as_str()) == Some(mnemonic) {
                return axum::Json(link_data.clone()).into_response();
            }
        }
    }

    axum::Json(json!({})).into_response()
}
