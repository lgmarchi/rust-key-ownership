use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use shared::{
    crypto::verify_signature,
    types::{VerifySignaturePayload, VerifySignatureResponse},
};
use tracing::info;
use validator::Validate;

use super::error::HandlerError;
use crate::state::AppState;

#[utoipa::path(
    post,
    path = "/api/verify-signature",
    request_body = VerifySignaturePayload,
    responses(
        (status = 200, description = "OK", body = VerifySignatureResponse),
        (status = 400, description = "Invalid payload"),
        (status = 409, description = "Replay attack"),
        (status = 401, description = "Invalid signature"),
        (status = 429, description = "Rate limit exceeded")
    )
)]
pub async fn verify_signature_handler(
    State(state): State<Arc<AppState>>,
    Json(signature_payload): Json<VerifySignaturePayload>,
) -> Result<impl IntoResponse, HandlerError> {
    let nonce = signature_payload.nonce_payload.nonce.clone();

    if let Err(e) = signature_payload.nonce_payload.validate() {
        return Err(HandlerError::PayloadValidation(e));
    }

    {
        let mut nonces = state.seen_nonces.lock().await;

        if nonces.contains(&nonce.id) {
            return Err(HandlerError::ReplayAttack(Json(json!({ "status": "error", "reason": "Replay attack" }))));
        }

        nonces.insert(nonce.id.clone());
    }

    match verify_signature(
        &signature_payload.nonce_payload,
        &signature_payload.signature,
        &signature_payload.public_key,
    ) {
        Ok(_) => {
            let pubkey_short = &signature_payload.public_key.get(0..8).unwrap_or("???");

            info!(
                trace_id = %nonce.id,
                issued_at = %nonce.issued_at,
                pubkey = %pubkey_short,
                "Signature is valid and nonce accepted"
            );

            Ok((
                StatusCode::OK,
                Json(VerifySignatureResponse { message: "Signature is valid and nonce accepted".to_string() }),
            ))
        }
        Err(e) => Err(HandlerError::SignatureValidation(Json(
            serde_json::json!({ "status": "error", "reason": "Invalid signature", "details": e.to_string() }),
        ))),
    }
}
