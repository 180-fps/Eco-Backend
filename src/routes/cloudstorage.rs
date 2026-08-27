use axum::{
    Router,
    routing::{get, put},
    response::IntoResponse,
    http::StatusCode,
    body::Bytes,
};
use serde_json::json;
use std::{fs, path::{Path, PathBuf}};
use sha1::{Sha1, Digest as Sha1Digest};
use sha2::Sha256;

/// Path to CloudStorage folder relative to the binary's working directory.
const CLOUD_DIR: &str = "CloudStorage";
const CLIENT_SETTINGS_DIR: &str = "ClientSettings";

pub fn router() -> Router {
    Router::new()
        .route("/fortnite/api/cloudstorage/system", get(cloudstorage_system))
        .route("/fortnite/api/cloudstorage/system/:file", get(cloudstorage_system_file))
        .route("/fortnite/api/cloudstorage/user/:account_id", get(cloudstorage_user))
        .route("/fortnite/api/cloudstorage/user/:account_id/:file", get(cloudstorage_user_file))
        .route("/fortnite/api/cloudstorage/user/:account_id/ClientSettings.Sav", put(cloudstorage_user_put))
}

// GET /fortnite/api/cloudstorage/system
async fn cloudstorage_system() -> impl IntoResponse {
    let dir = Path::new(CLOUD_DIR);
    if !dir.exists() {
        let _ = fs::create_dir_all(dir);
    }

    let mut cloud_files = vec![];

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.to_lowercase().ends_with(".ini") {
                    if let Ok(data) = fs::read(&path) {
                        if let Ok(meta) = fs::metadata(&path) {
                            let sha1 = hex::encode(Sha1::digest(&data));
                            let sha256 = hex::encode(Sha256::digest(&data));
                            let modified = meta.modified()
                                .ok()
                                .and_then(|t| {
                                    use std::time::UNIX_EPOCH;
                                    t.duration_since(UNIX_EPOCH).ok()
                                })
                                .map(|d| {
                                    let secs = d.as_secs();
                                    chrono::DateTime::<chrono::Utc>::from_timestamp(secs as i64, 0)
                                        .map(|dt| dt.to_rfc3339())
                                        .unwrap_or_default()
                                })
                                .unwrap_or_default();

                            cloud_files.push(json!({
                                "uniqueFilename": name,
                                "filename": name,
                                "hash": sha1,
                                "hash256": sha256,
                                "length": data.len(),
                                "contentType": "application/octet-stream",
                                "uploaded": modified,
                                "storageType": "S3",
                                "storageIds": {},
                                "doNotCache": true
                            }));
                        }
                    }
                }
            }
        }
    }

    axum::Json(json!(cloud_files))
}

// GET /fortnite/api/cloudstorage/system/:file
async fn cloudstorage_system_file(
    axum::extract::Path(file): axum::extract::Path<String>,
) -> impl IntoResponse {
    // Path traversal protection
    if file.contains("..") || file.contains('~') {
        return (StatusCode::NOT_FOUND, vec![]).into_response();
    }

    let path = Path::new(CLOUD_DIR).join(Path::new(&file).file_name().unwrap_or_default());
    match fs::read(&path) {
        Ok(data) => (StatusCode::OK, data).into_response(),
        Err(_) => (StatusCode::OK, vec![]).into_response(),
    }
}

// GET /fortnite/api/cloudstorage/user/:accountId
async fn cloudstorage_user(
    axum::extract::Path(account_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let settings_path = client_settings_path(&account_id);
    let file = settings_path.join("ClientSettings.Sav");

    if file.exists() {
        if let Ok(data) = fs::read(&file) {
            if let Ok(meta) = fs::metadata(&file) {
                let sha1 = hex::encode(Sha1::digest(&data));
                let sha256 = hex::encode(Sha256::digest(&data));
                let modified = meta.modified()
                    .ok()
                    .and_then(|t| {
                        use std::time::UNIX_EPOCH;
                        t.duration_since(UNIX_EPOCH).ok()
                    })
                    .map(|d| {
                        let secs = d.as_secs();
                        chrono::DateTime::<chrono::Utc>::from_timestamp(secs as i64, 0)
                            .map(|dt| dt.to_rfc3339())
                            .unwrap_or_default()
                    })
                    .unwrap_or_default();

                return axum::Json(json!([{
                    "uniqueFilename": "ClientSettings.Sav",
                    "filename": "ClientSettings.Sav",
                    "hash": sha1,
                    "hash256": sha256,
                    "length": data.len(),
                    "contentType": "application/octet-stream",
                    "uploaded": modified,
                    "storageType": "S3",
                    "storageIds": {},
                    "accountId": account_id,
                    "doNotCache": false
                }])).into_response();
            }
        }
    }

    axum::Json(json!([])).into_response()
}

// GET /fortnite/api/cloudstorage/user/:accountId/:file
async fn cloudstorage_user_file(
    axum::extract::Path((account_id, file)): axum::extract::Path<(String, String)>,
) -> impl IntoResponse {
    if file.to_lowercase() != "clientsettings.sav" {
        return (StatusCode::OK, vec![]).into_response();
    }

    let settings_path = client_settings_path(&account_id);
    ensure_dir(&settings_path);

    let file_path = settings_path.join("ClientSettings.Sav");
    match fs::read(&file_path) {
        Ok(data) => (StatusCode::OK, data).into_response(),
        Err(_) => (StatusCode::OK, vec![]).into_response(),
    }
}

// PUT /fortnite/api/cloudstorage/user/:accountId/ClientSettings.Sav
async fn cloudstorage_user_put(
    axum::extract::Path((account_id, _file)): axum::extract::Path<(String, String)>,
    body: Bytes,
) -> impl IntoResponse {
    if body.is_empty() {
        return StatusCode::NO_CONTENT.into_response();
    }
    if body.len() >= 400_000 {
        return (StatusCode::FORBIDDEN, axum::Json(json!({"error": "File size must be less than 400kb."}))).into_response();
    }

    let settings_path = client_settings_path(&account_id);
    ensure_dir(&settings_path);

    let file_path = settings_path.join("ClientSettings.Sav");
    let _ = fs::write(&file_path, &body);

    StatusCode::NO_CONTENT.into_response()
}

fn client_settings_path(account_id: &str) -> PathBuf {
    Path::new(CLIENT_SETTINGS_DIR).join(account_id)
}

fn ensure_dir(path: &Path) {
    if !path.exists() {
        let _ = fs::create_dir_all(path);
    }
}
