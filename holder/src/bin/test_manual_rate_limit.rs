use base64::Engine;
use chrono::Utc;
use reqwest::Client;
use shared::{
    crypto::{generate_keypair, sign_payload},
    types::{Nonce, NoncePayload, VerifySignaturePayload},
    BASE64_ENGINE,
};
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared::init_tracing();
    info!("Starting rate limit test...");

    let keypair = generate_keypair();
    let verifying_key = keypair.verifying_key();
    let client = Client::new();

    let url = std::env::var("VERIFY_SIGNATURE_API_URL")
        .unwrap_or_else(|_| "http://localhost:3000/api/verify-signature".into());

    // Make 15 requests in quick succession (more than the rate limit of 10 per minute)
    for i in 0..15 {
        let nonce = Nonce { id: Uuid::new_v4().to_string(), issued_at: Utc::now().timestamp_millis() };

        let nonce_payload =
            NoncePayload { nonce: nonce.clone(), message: format!("Rate limit test request {}", i + 1) };

        let signature = sign_payload(&nonce_payload, &keypair);
        let public_key = BASE64_ENGINE.encode(verifying_key.as_bytes());

        let signed = VerifySignaturePayload { nonce_payload, signature, public_key };

        let res = client.post(&url).json(&signed).send().await?;
        println!("Request {}: Status {}", i + 1, res.status());
    }

    Ok(())
}
