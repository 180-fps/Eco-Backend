use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;


#[allow(dead_code)]
pub fn create_error(
    error_code: &str,
    error_message: &str,
    message_vars: Option<Vec<&str>>,
    numeric_error_code: i64,
    error: Option<&str>,
    status_code: u16,
) -> Response {
    let status = StatusCode::from_u16(status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let body = json!({
        "errorCode": error_code,
        "errorMessage": error_message,
        "messageVars": message_vars.unwrap_or_default(),
        "numericErrorCode": numeric_error_code,
        "originatingService": "any",
        "intent": "prod",
        "error_description": error_message,
        "error": error
    });

    (status, Json(body)).into_response()
}
