use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::{error, warn};
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Error to validate payload")]
    PayloadValidation(ValidationErrors),

    #[error("Tried to use same nonce")]
    ReplayAttack(Json<serde_json::Value>),

    #[error("Signature verification failed")]
    SignatureValidation(Json<serde_json::Value>),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            HandlerError::PayloadValidation(e) => {
                error!(
                    error = ?e,
                    "Payload validation failed: {}",
                    e.to_string()
                );
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            HandlerError::ReplayAttack(response) => {
                warn!(
                    response = ?response,
                    "Replay attack detected: {:?}",
                    response
                );
                (StatusCode::CONFLICT, format!("{:?}", response))
            }
            HandlerError::SignatureValidation(response) => {
                error!(
                    response = ?response,
                    "Signature validation failed: {:?}",
                    response
                );
                (StatusCode::UNAUTHORIZED, format!("{:?}", response))
            }
            HandlerError::RateLimitExceeded => {
                warn!("Rate limit exceeded for request");
                (
                    StatusCode::TOO_MANY_REQUESTS,
                    json!({
                        "status": "error",
                        "reason": "Rate limit exceeded",
                        "message": "Too many requests. Please try again later."
                    })
                    .to_string(),
                )
            }
        };

        let body = Json(json!({
            "status": "error",
            "message": error_message
        }));

        (status, body).into_response()
    }
}
