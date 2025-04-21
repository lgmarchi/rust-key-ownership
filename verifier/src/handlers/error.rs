use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Error to validate payload")]
    PayloadValidation(ValidationErrors),

    #[error("Tried to use same nonce")]
    ReplayAttack(Json<serde_json::Value>),

    #[error("Signature verification failed")]
    SignatureValidation(Json<serde_json::Value>),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> Response {
        match self {
            HandlerError::PayloadValidation(e) => {
                let body = json!({
                    "status": "error",
                    "reason": "Invalid payload",
                    "details": e.to_string()
                });

                error!("Payload validation error: {}", body);

                (StatusCode::BAD_REQUEST, Json(body)).into_response()
            }
            HandlerError::ReplayAttack(json) => {
                error!("Replay attack error: {}", json.to_string());

                (StatusCode::CONFLICT, json).into_response()
            }
            HandlerError::SignatureValidation(json) => {
                error!("Signature validation error: {}", json.to_string());

                (StatusCode::UNAUTHORIZED, json).into_response()
            }
        }
    }
}
