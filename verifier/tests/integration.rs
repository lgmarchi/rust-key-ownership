use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use base64::Engine;
use chrono::Utc;
use shared::{
    crypto::{generate_keypair, sign_payload},
    types::{Nonce, NoncePayload, VerifySignaturePayload},
    BASE64_ENGINE,
};
use verifier::{handlers::verify_handler_request::verify_signature_handler, state::AppState};

#[tokio::test]

async fn test_replay_attack_blocked() {
    let state = Arc::new(AppState::default());

    let keypair = generate_keypair();

    let verifying_key = keypair.verifying_key();

    let nonce = Nonce { id: uuid::Uuid::new_v4().to_string(), issued_at: Utc::now().timestamp_millis() };

    let message = "Hello from test!".to_string();

    let nonce_payload = NoncePayload { nonce: nonce.clone(), message };

    let signature = sign_payload(&nonce_payload, &keypair);

    let public_key = BASE64_ENGINE.encode(verifying_key.as_bytes());

    let signed_payload = VerifySignaturePayload { nonce_payload, signature, public_key };

    let first_response =
        verify_signature_handler(State(state.clone()), Json(signed_payload.clone())).await.into_response();

    assert_eq!(first_response.status(), 200);

    let second_response = verify_signature_handler(State(state.clone()), Json(signed_payload)).await.into_response();

    assert_eq!(second_response.status(), 409);
}
