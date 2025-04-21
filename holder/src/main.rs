use base64::Engine;
use chrono::Utc;
use reqwest::Client;
use shared::{
    crypto::{generate_keypair, sign_payload},
    types::{Nonce, NoncePayload},
    BASE64_ENGINE,
};
use tracing::{debug, info};
use uuid::Uuid;

mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    shared::init_tracing();
    info!("Holder started");

    let config = config::AppConfig::from_env();

    let keypair = generate_keypair();
    info!("Keypair generated");

    let verifying_key = keypair.verifying_key();
    let nonce = Nonce { id: Uuid::new_v4().to_string(), issued_at: Utc::now().timestamp_millis() };
    let message = "Hello Verifier!".to_string();
    let nonce_payload = NoncePayload { nonce: nonce.clone(), message };

    debug!(nonce = ?nonce_payload.nonce, "Payload created");

    let signature = sign_payload(&nonce_payload, &keypair);
    debug!("Payload signed");

    let public_key = BASE64_ENGINE.encode(verifying_key.as_bytes());
    debug!(key = %public_key, "Public key encoded");

    let signed_payload = shared::types::VerifySignaturePayload { nonce_payload, signature, public_key };
    info!("Sending payload to verifier at {}", config.api_url);

    let client = Client::new();
    let res = client.post(config.api_url).json(&signed_payload).send().await?;
    let status = res.status();

    let body = res.text().await?;
    info!(%status, "Response received from verifier");
    debug!(%body, "Response content");

    Ok(())
}
