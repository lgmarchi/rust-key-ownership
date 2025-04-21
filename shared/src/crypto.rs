use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{error::CryptoError, types::NoncePayload, BASE64_ENGINE};

pub fn generate_keypair() -> SigningKey {
    let mut csprng = OsRng;

    SigningKey::generate(&mut csprng)
}

pub fn sign_payload(nonce_payload: &NoncePayload, keypair: &SigningKey) -> String {
    let message = serde_json::to_vec(nonce_payload).expect("Serialization failed");

    let signature = keypair.sign(&message);

    BASE64_ENGINE.encode(signature.to_bytes())
}

pub fn verify_signature(
    nonce_payload: &NoncePayload,
    signature_b64: &str,
    pubkey_b64: &str,
) -> Result<(), CryptoError> {
    let message = serde_json::to_vec(nonce_payload)?;

    let pubkey_bytes =
        BASE64_ENGINE.decode(pubkey_b64)?.as_slice().try_into().map_err(|_| CryptoError::ErrorToSlice)?;

    let signature_bytes = BASE64_ENGINE.decode(signature_b64)?;

    let pubkey = VerifyingKey::from_bytes(&pubkey_bytes)?;

    let signature_array: &[u8; 64] = signature_bytes.as_slice().try_into().map_err(|_| CryptoError::ErrorToSlice)?;

    let signature = Signature::from_bytes(signature_array);

    pubkey.verify(&message, &signature).map_err(|_| CryptoError::InvalidSignature)?;

    Ok(())
}
