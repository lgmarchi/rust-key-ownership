use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
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
        id: "test-replay-123".to_string(),
        issued_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64,
    };

    let nonce_payload = NoncePayload { nonce: nonce.clone(), message: "Testing replay attack".into() };

    let signature = sign_payload(&nonce_payload, &keypair);

    let public_key = BASE64_ENGINE.encode(verifying_key.as_bytes());

    let signed = VerifySignaturePayload { nonce_payload, signature, public_key };

    let client = Client::new();

    let url = std::env::var("VERIFY_SIGNATURE_API_URL")
        .unwrap_or_else(|_| "http://localhost:3000/api/verify-signature".into());

    let res1 = client.post(&url).json(&signed).send().await?;

    println!("✅ First response: {}", res1.status());

    let res2 = client.post(&url).json(&signed).send().await?;

    println!("⛔ Second response (should be 409): {}", res2.status());

    Ok(())
}
