use axum::{
    Router,
    routing::{get, post, patch, delete},
    response::IntoResponse,
    extract::{Path, State},
    Json,
    http::StatusCode,
};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use uuid::Uuid;
use chrono::Utc;

pub type PartyStore = Arc<Mutex<HashMap<String, Value>>>;

pub fn router(store: PartyStore) -> Router {
    Router::new()
        .route("/party/api/v1/Fortnite/user/:account_id/notifications/undelivered/count", get(notifications_count))
        .route("/party/api/v1/Fortnite/user/:account_id", get(user_parties))
        .route("/party/api/v1/Fortnite/parties", post(create_party))
        .route("/party/api/v1/Fortnite/parties/:pid", get(get_party))
        .route("/party/api/v1/Fortnite/parties/:pid", patch(patch_party))
        .route("/party/api/v1/Fortnite/parties/:pid/members/:account_id", delete(leave_party))
        .route("/party/api/v1/Fortnite/parties/:pid/members/:account_id/join", post(join_party))
        .route("/party/api/v1/Fortnite/parties/:pid/members/:account_id/promote", post(promote_member))
        .route("/party/api/v1/Fortnite/parties/:pid/invites/:account_id", post(send_invite))
        .route("/party/api/v1/Fortnite/parties/:pid/members/:account_id/meta", patch(patch_member_meta))
        .with_state(store)
}

async fn notifications_count(Path(_account_id): Path<String>) -> impl IntoResponse {
    axum::Json(json!({ "pings": 0, "invites": 0 }))
}

async fn user_parties(
    Path(account_id): Path<String>,
    State(store): State<PartyStore>,
) -> impl IntoResponse {
    let parties = store.lock().unwrap();
    let current: Vec<&Value> = parties.values()
        .filter(|p| {
            p.get("members")
                .and_then(|m| m.as_array())
                .map(|arr| arr.iter().any(|m| m.get("account_id").and_then(|id| id.as_str()) == Some(&account_id)))
                .unwrap_or(false)
        })
        .collect();

    axum::Json(json!({
        "current": current,
        "pending": [],
        "invites": [],
        "pings": []
    }))
}

async fn create_party(
    State(store): State<PartyStore>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string().replace('-', "");
    let now = Utc::now().to_rfc3339();

    let account_id = body
        .get("join_info")
        .and_then(|j| j.get("connection"))
        .and_then(|c| c.get("id"))
        .and_then(|id| id.as_str())
        .unwrap_or("")
        .split("@prod")
        .next()
        .unwrap_or("")
        .to_string();

    let party = json!({
        "id": id,
        "created_at": now,
        "updated_at": now,
        "config": body.get("config").cloned().unwrap_or(json!({})),
        "members": [{
            "account_id": account_id,
            "meta": body.get("join_info").and_then(|j| j.get("meta")).cloned().unwrap_or(json!({})),
            "connections": [{
                "id": body.get("join_info").and_then(|j| j.get("connection")).and_then(|c| c.get("id")).cloned().unwrap_or(json!("")),
                "connected_at": now,
                "updated_at": now,
                "yield_leadership": false,
                "meta": json!({})
            }],
            "revision": 0,
            "updated_at": now,
            "joined_at": now,
            "role": "CAPTAIN"
        }],
        "applicants": [],
        "meta": body.get("meta").cloned().unwrap_or(json!({})),
        "invites": [],
        "revision": 0,
        "intentions": []
    });

    store.lock().unwrap().insert(id.clone(), party.clone());

    axum::Json(party)
}

async fn get_party(
    Path(pid): Path<String>,
    State(store): State<PartyStore>,
) -> impl IntoResponse {
    let parties = store.lock().unwrap();
    match parties.get(&pid) {
        Some(party) => axum::Json(party.clone()).into_response(),
        None => (StatusCode::NOT_FOUND, axum::Json(json!({
            "errorCode": "errors.com.epicgames.party.not_found",
            "errorMessage": format!("Party {} does not exist!", pid),
            "numericErrorCode": 51002
        }))).into_response(),
    }
}

async fn patch_party(
    Path(pid): Path<String>,
    State(store): State<PartyStore>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    let mut parties = store.lock().unwrap();
    match parties.get_mut(&pid) {
        Some(party) => {
            let now = Utc::now().to_rfc3339();

            if let Some(config_update) = body.get("config") {
                if let (Some(existing), Some(updates)) = (party.get_mut("config"), config_update.as_object()) {
                    if let Some(obj) = existing.as_object_mut() {
                        for (k, v) in updates {
                            obj.insert(k.clone(), v.clone());
                        }
                    }
                }
            }

            if let Some(meta_update) = body.get("meta") {
                if let Some(meta) = party.get_mut("meta").and_then(|m| m.as_object_mut()) {
                    if let Some(deletes) = meta_update.get("delete").and_then(|d| d.as_array()) {
                        for key in deletes {
                            if let Some(k) = key.as_str() { meta.remove(k); }
                        }
                    }
                    if let Some(updates) = meta_update.get("update").and_then(|u| u.as_object()) {
                        for (k, v) in updates {
                            meta.insert(k.clone(), v.clone());
                        }
                    }
                }
            }

            party["updated_at"] = json!(now);
            StatusCode::NO_CONTENT.into_response()
        }
        None => (StatusCode::NOT_FOUND, axum::Json(json!({ "error": "party not found" }))).into_response(),
    }
}

async fn patch_member_meta(
    Path((pid, account_id)): Path<(String, String)>,
    State(store): State<PartyStore>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    let mut parties = store.lock().unwrap();
    match parties.get_mut(&pid) {
        Some(party) => {
            let now = Utc::now().to_rfc3339();
            if let Some(members) = party.get_mut("members").and_then(|m| m.as_array_mut()) {
                if let Some(member) = members.iter_mut()
                    .find(|m| m.get("account_id").and_then(|id| id.as_str()) == Some(&account_id))
                {
                    if let Some(meta) = member.get_mut("meta").and_then(|m| m.as_object_mut()) {
                        if let Some(deletes) = body.get("delete").and_then(|d| d.as_object()) {
                            for k in deletes.keys() { meta.remove(k); }
                        }
                        if let Some(updates) = body.get("update").and_then(|u| u.as_object()) {
                            for (k, v) in updates { meta.insert(k.clone(), v.clone()); }
                        }
                    }
                    member["updated_at"] = json!(now);
                }
            }
            party["updated_at"] = json!(now);
            StatusCode::NO_CONTENT.into_response()
        }
        None => (StatusCode::NOT_FOUND, axum::Json(json!({ "error": "party not found" }))).into_response(),
    }
}

async fn leave_party(
    Path((pid, account_id)): Path<(String, String)>,
    State(store): State<PartyStore>,
) -> impl IntoResponse {
    let mut parties = store.lock().unwrap();
    match parties.get_mut(&pid) {
        Some(party) => {
            if let Some(members) = party.get_mut("members").and_then(|m| m.as_array_mut()) {
                members.retain(|m| m.get("account_id").and_then(|id| id.as_str()) != Some(&account_id));
            }
            let is_empty = party.get("members").and_then(|m| m.as_array()).map(|a| a.is_empty()).unwrap_or(true);
            if is_empty {
                parties.remove(&pid);
            }
            StatusCode::NO_CONTENT.into_response()
        }
        None => StatusCode::NO_CONTENT.into_response(),
    }
}

async fn join_party(
    Path((pid, account_id)): Path<(String, String)>,
    State(store): State<PartyStore>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    let mut parties = store.lock().unwrap();
    match parties.get_mut(&pid) {
        Some(party) => {
            let now = Utc::now().to_rfc3339();
            let conn_id = body.get("connection").and_then(|c| c.get("id")).and_then(|id| id.as_str()).unwrap_or("").to_string();
            let member_account_id = conn_id.split("@prod").next().unwrap_or(&account_id).to_string();

            let new_member = json!({
                "account_id": member_account_id,
                "meta": body.get("meta").cloned().unwrap_or(json!({})),
                "connections": [{
                    "id": conn_id,
                    "connected_at": now,
                    "updated_at": now,
                    "yield_leadership": false,
                    "meta": body.get("connection").and_then(|c| c.get("meta")).cloned().unwrap_or(json!({}))
                }],
                "revision": 0,
                "updated_at": now,
                "joined_at": now,
                "role": "MEMBER"
            });

            if let Some(members) = party.get_mut("members").and_then(|m| m.as_array_mut()) {
                // Don't double-add
                if !members.iter().any(|m| m.get("account_id").and_then(|id| id.as_str()) == Some(&member_account_id)) {
                    members.push(new_member);
                }
            }

            party["updated_at"] = json!(now);
            axum::Json(json!({ "status": "JOINED", "party_id": pid })).into_response()
        }
        None => (StatusCode::NOT_FOUND, axum::Json(json!({ "error": "party not found" }))).into_response(),
    }
}

async fn promote_member(
    Path((pid, account_id)): Path<(String, String)>,
    State(store): State<PartyStore>,
) -> impl IntoResponse {
    let mut parties = store.lock().unwrap();
    if let Some(party) = parties.get_mut(&pid) {
        if let Some(members) = party.get_mut("members").and_then(|m| m.as_array_mut()) {
            for member in members.iter_mut() {
                let is_target = member.get("account_id").and_then(|id| id.as_str()) == Some(&account_id);
                member["role"] = if is_target { json!("CAPTAIN") } else { json!("MEMBER") };
            }
        }
    }
    StatusCode::NO_CONTENT.into_response()
}

async fn send_invite(
    Path((_pid, _account_id)): Path<(String, String)>,
    State(_store): State<PartyStore>,
) -> impl IntoResponse {
    // Just 204 — XMPP not implemented yet
    StatusCode::NO_CONTENT.into_response()
}
