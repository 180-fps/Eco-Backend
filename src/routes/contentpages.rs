use axum::{Router, routing::{get, post}, response::IntoResponse, http::HeaderMap};
use serde_json::json;

static CONTENTPAGES_JSON: &str = include_str!("../responses/contentpages.json");
static SPARK_TRACKS_JSON: &str = "[]"; // placeholder

pub fn router() -> Router {
    Router::new()
        .route("/content/api/pages/fortnite-game/spark-tracks", get(spark_tracks))
        .route("/content/api/pages/fortnite-game", get(fortnite_game))
        .route("/content/api/pages/*path", get(content_pages))
        .route("/api/v1/fortnite-br/surfaces/motd/target", post(motd_target))
}

async fn spark_tracks() -> impl IntoResponse {
    axum::Json(serde_json::from_str::<serde_json::Value>(SPARK_TRACKS_JSON)
        .unwrap_or(serde_json::Value::Array(vec![])))
}

async fn content_pages(headers: HeaderMap) -> impl IntoResponse {
    let pages = get_content_pages(&headers);
    axum::Json(pages)
}

async fn fortnite_game(headers: HeaderMap) -> impl IntoResponse {
    let pages = get_content_pages(&headers);
    axum::Json(pages)
}

async fn motd_target() -> impl IntoResponse {
    axum::Json(json!({
        "contentItems": [],
        "contentItemResponse": [],
        "contentSchemaVersion": 1
    }))
}

fn get_content_pages(headers: &HeaderMap) -> serde_json::Value {
    let ua = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let mut pages: serde_json::Value = serde_json::from_str(CONTENTPAGES_JSON)
        .unwrap_or(json!({}));

    // Parse season from User-Agent
    let season = crate::utils::version_info::VersionInfo::from_user_agent(ua).season;
    let build = crate::utils::version_info::VersionInfo::from_user_agent(ua).build;

    // Set dynamic background stage per season
    let stage = if season == 10 {
        "seasonx".to_string()
    } else {
        format!("season{}", season)
    };

    if let Some(bg) = pages
        .get_mut("dynamicbackgrounds")
        .and_then(|v| v.get_mut("backgrounds"))
        .and_then(|v| v.get_mut("backgrounds"))
        .and_then(|v| v.as_array_mut())
    {
        for entry in bg.iter_mut() {
            if let Some(s) = entry.get_mut("stage") {
                *s = json!(stage);
            }
        }
    }

    // Season-specific lobby background images
    if season == 20 {
        if let Some(bg) = pages
            .get_mut("dynamicbackgrounds")
            .and_then(|v| v.get_mut("backgrounds"))
            .and_then(|v| v.get_mut("backgrounds"))
            .and_then(|v| v.as_array_mut())
        {
            if let Some(first) = bg.first_mut() {
                first["backgroundimage"] = json!("https://cdn2.unrealengine.com/t-bp20-lobby-2048x1024-d89eb522746c.png");
            }
        }
    }

    // For old builds (< 5.3) swap news images
    if build < 5.3 && build > 0.0 {
        for mode in &["battleroyalenews", "savetheworldnews"] {
            if let Some(news) = pages.get_mut(*mode)
                .and_then(|v| v.get_mut("news"))
                .and_then(|v| v.get_mut("messages"))
                .and_then(|v| v.as_array_mut())
            {
                if let Some(msg) = news.get_mut(0) {
                    msg["image"] = json!("https://cdn.discordapp.com/attachments/927739901540188200/930879507496308736/discord.png");
                }
                if let Some(msg) = news.get_mut(1) {
                    msg["image"] = json!("https://i.imgur.com/ImIwpRm.png");
                }
            }
        }
    }

    pages
}
