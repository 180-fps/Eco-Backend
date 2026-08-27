mod config;
mod routes;
mod utils;

use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use config::Config;
use routes::party::PartyStore;
use routes::friends::FriendsStore;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "eco=info,tower_http=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Arc::new(Config::load());
    let port = config.port;

    // Shared in-memory stores
    let party_store: PartyStore = Arc::new(Mutex::new(HashMap::new()));
    let friends_store: FriendsStore = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        // Core Fortnite routes
        .merge(routes::version::router())
        .merge(routes::contentpages::router())
        .merge(routes::lightswitch::router())
        .merge(routes::auth::router())
        .merge(routes::datarouter::router())
        .merge(routes::storefront::router())
        .merge(routes::cloudstorage::router())
        .merge(routes::discovery::router())
        .merge(routes::mcp::router())
        .merge(routes::legal::router())
        .merge(routes::matchmaking::router())
        .merge(routes::misc::router())
        // User / account
        .merge(routes::user::router())
        .merge(routes::account::router())
        // Stateful routes (need their stores)
        .merge(routes::party::router(party_store))
        .merge(routes::friends::router(friends_store))
        // 404 fallback
        .fallback(not_found)
        // Shared config accessible to all handlers
        .layer(Extension(config))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind port");

    tracing::info!("Eco Backend started on port {}", port);

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::Json(serde_json::json!({
            "errorCode": "errors.com.epicgames.common.not_found",
            "errorMessage": "Sorry the resource you were trying to find could not be found",
            "numericErrorCode": 1004,
            "originatingService": "any",
            "intent": "prod"
        })),
    )
}
