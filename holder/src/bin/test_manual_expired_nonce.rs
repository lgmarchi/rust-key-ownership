use base64::Engine;
use chrono::{Duration, Utc};
use reqwest::Client;
use shared::{
    crypto::{generate_keypair, sign_payload},
    types::{Nonce, NoncePayload, VerifySignaturePayload},
    BASE64_ENGINE,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let keypair = generate_keypair();

    let verifying_key = keypair.verifying_key();

    let nonce = Nonce {
        id: uuid::Uuid::new_v4().to_string(),
        issued_at: (Utc::now() - Duration::milliseconds(31_000)).timestamp_millis(),
    };

    let nonce_payload = NoncePayload { nonce: nonce.clone(), message: "Testing expired nonce".into() };

    let signature = sign_payload(&nonce_payload, &keypair);

    let public_key = BASE64_ENGINE.encode(verifying_key.as_bytes());

    let signed = VerifySignaturePayload { nonce_payload, signature, public_key };

    let client = Client::new();

    let url = std::env::var("VERIFY_SIGNATURE_API_URL")
        .unwrap_or_else(|_| "http://localhost:3000/api/verify-signature".into());

    let _ = client.post(&url).json(&signed).send().await?;

    Ok(())
}
