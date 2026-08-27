use axum::{
    Router,
    routing::{get, post, delete},
    response::IntoResponse,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use chrono::Utc;

/// In-memory friends store: accountId -> FriendList
#[derive(Clone, Default)]
pub struct FriendList {
    pub accepted: Vec<FriendEntry>,
    pub incoming: Vec<FriendEntry>,
    pub outgoing: Vec<FriendEntry>,
    pub blocked: Vec<FriendEntry>,
}

#[derive(Clone)]
pub struct FriendEntry {
    pub account_id: String,
    pub created: String,
    pub alias: String,
}

pub type FriendsStore = Arc<Mutex<HashMap<String, FriendList>>>;

pub fn router(store: FriendsStore) -> Router {
    Router::new()
        .route("/friends/api/v1/:account_id/settings", get(settings))
        .route("/friends/api/v1/:account_id/blocklist", get(blocklist_get))
        .route("/friends/api/public/list/fortnite/:account_id/recentPlayers", get(recent_players))
        .route("/friends/api/v1/:account_id/friends/:friend_id/alias", post(alias).delete(alias))
        .route("/friends/api/:version/friends/:receiver_id", post(send_or_accept_friend))
        .route("/friends/api/:version/friends/:receiver_id", delete(remove_friend))
        .route("/friends/api/:version/blocklist/:receiver_id", post(block_friend))
        .route("/friends/api/:version/blocklist/:receiver_id", delete(unblock_friend))
        .route("/friends/api/v1/:account_id/summary", get(summary))
        .route("/friends/api/public/blocklist/:account_id", get(public_blocklist))
        .with_state(store)
}

async fn settings() -> impl IntoResponse {
    axum::Json(json!({}))
}

async fn blocklist_get() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn recent_players() -> impl IntoResponse {
    axum::Json(json!([]))
}

async fn alias() -> StatusCode {
    StatusCode::NO_CONTENT
}

// POST /friends/api/:version/friends/:receiverId
// Sends a friend request, or accepts if there's an incoming request
async fn send_or_accept_friend(
    Path((_version, receiver_id)): Path<(String, String)>,
    State(store): State<FriendsStore>,
) -> StatusCode {
    let mut friends = store.lock().unwrap();
    let now = Utc::now().to_rfc3339();

    // We don't have auth in Section 1 scope, so we use a placeholder sender
    let sender_id = "eco_user".to_string();

    let receiver = friends.entry(receiver_id.clone()).or_default();
    let incoming_idx = receiver.incoming.iter().position(|f| f.account_id == sender_id);

    if let Some(idx) = incoming_idx {
        // Accept the request
        let entry = receiver.incoming.remove(idx);
        receiver.accepted.push(entry.clone());

        let sender = friends.entry(sender_id.clone()).or_default();
        if let Some(pos) = sender.outgoing.iter().position(|f| f.account_id == receiver_id) {
            sender.outgoing.remove(pos);
        }
        sender.accepted.push(FriendEntry {
            account_id: receiver_id,
            created: now,
            alias: String::new(),
        });
    } else {
        // Send request
        let sender = friends.entry(sender_id.clone()).or_default();
        if !sender.outgoing.iter().any(|f| f.account_id == receiver_id) {
            sender.outgoing.push(FriendEntry {
                account_id: receiver_id.clone(),
                created: now.clone(),
                alias: String::new(),
            });
        }

        let receiver = friends.entry(receiver_id).or_default();
        if !receiver.incoming.iter().any(|f| f.account_id == sender_id) {
            receiver.incoming.push(FriendEntry {
                account_id: sender_id,
                created: now,
                alias: String::new(),
            });
        }
    }

    StatusCode::NO_CONTENT
}

// DELETE /friends/api/:version/friends/:receiverId
async fn remove_friend(
    Path((_version, receiver_id)): Path<(String, String)>,
    State(store): State<FriendsStore>,
) -> StatusCode {
    let mut friends = store.lock().unwrap();
    let sender_id = "eco_user".to_string();

    if let Some(sender) = friends.get_mut(&sender_id) {
        sender.accepted.retain(|f| f.account_id != receiver_id);
        sender.outgoing.retain(|f| f.account_id != receiver_id);
    }
    if let Some(receiver) = friends.get_mut(&receiver_id) {
        receiver.accepted.retain(|f| f.account_id != sender_id);
        receiver.incoming.retain(|f| f.account_id != sender_id);
    }

    StatusCode::NO_CONTENT
}

// POST /friends/api/:version/blocklist/:receiverId
async fn block_friend(
    Path((_version, receiver_id)): Path<(String, String)>,
    State(store): State<FriendsStore>,
) -> StatusCode {
    let mut friends = store.lock().unwrap();
    let sender_id = "eco_user".to_string();
    let now = Utc::now().to_rfc3339();

    let sender = friends.entry(sender_id).or_default();
    if !sender.blocked.iter().any(|f| f.account_id == receiver_id) {
        sender.blocked.push(FriendEntry {
            account_id: receiver_id,
            created: now,
            alias: String::new(),
        });
    }

    StatusCode::NO_CONTENT
}

// DELETE /friends/api/:version/blocklist/:receiverId
async fn unblock_friend(
    Path((_version, receiver_id)): Path<(String, String)>,
    State(store): State<FriendsStore>,
) -> StatusCode {
    let mut friends = store.lock().unwrap();
    let sender_id = "eco_user".to_string();

    if let Some(sender) = friends.get_mut(&sender_id) {
        sender.blocked.retain(|f| f.account_id != receiver_id);
    }

    StatusCode::NO_CONTENT
}

// GET /friends/api/v1/:accountId/summary
async fn summary(
    Path(account_id): Path<String>,
    State(store): State<FriendsStore>,
) -> impl IntoResponse {
    let friends = store.lock().unwrap();
    let empty = FriendList::default();
    let list = friends.get(&account_id).unwrap_or(&empty);

    let accepted: Vec<Value> = list.accepted.iter().map(|f| json!({
        "accountId": f.account_id,
        "groups": [],
        "mutual": 0,
        "alias": f.alias,
        "note": "",
        "favorite": false,
        "created": f.created
    })).collect();

    let incoming: Vec<Value> = list.incoming.iter().map(|f| json!({
        "accountId": f.account_id,
        "mutual": 0,
        "favorite": false,
        "created": f.created
    })).collect();

    let outgoing: Vec<Value> = list.outgoing.iter().map(|f| json!({
        "accountId": f.account_id,
        "favorite": false
    })).collect();

    let blocked: Vec<Value> = list.blocked.iter().map(|f| json!({
        "accountId": f.account_id
    })).collect();

    axum::Json(json!({
        "friends": accepted,
        "incoming": incoming,
        "outgoing": outgoing,
        "suggested": [],
        "blocklist": blocked,
        "settings": { "acceptInvites": "public" }
    }))
}

// GET /friends/api/public/blocklist/:accountId
async fn public_blocklist(
    Path(account_id): Path<String>,
    State(store): State<FriendsStore>,
) -> impl IntoResponse {
    let friends = store.lock().unwrap();
    let blocked_users: Vec<&str> = friends
        .get(&account_id)
        .map(|f| f.blocked.iter().map(|b| b.account_id.as_str()).collect())
        .unwrap_or_default();

    axum::Json(json!({ "blockedUsers": blocked_users }))
}
