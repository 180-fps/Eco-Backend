use axum::{Router, routing::get, response::IntoResponse};
use serde_json::json;

static CATALOG_JSON: &str = include_str!("../responses/catalog.json");
static KEYCHAIN_JSON: &str = include_str!("../responses/keychain.json");

pub fn router() -> Router {
    Router::new()
        .route("/fortnite/api/storefront/v2/catalog", get(catalog))
        .route("/fortnite/api/storefront/v2/keychain", get(keychain))
        .route("/catalog/api/shared/bulk/offers", get(bulk_offers))
}

// GET /fortnite/api/storefront/v2/catalog
async fn catalog() -> impl IntoResponse {
    let shop: serde_json::Value = serde_json::from_str(CATALOG_JSON)
        .unwrap_or(json!({ "storefronts": [] }));
    axum::Json(shop)
}

// GET /fortnite/api/storefront/v2/keychain
async fn keychain() -> impl IntoResponse {
    let data: serde_json::Value = serde_json::from_str(KEYCHAIN_JSON)
        .unwrap_or(json!([]));
    axum::Json(data)
}

// GET /catalog/api/shared/bulk/offers
async fn bulk_offers() -> impl IntoResponse {
    axum::Json(json!({}))
}
