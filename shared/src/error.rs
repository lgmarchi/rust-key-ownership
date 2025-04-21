use base64::DecodeError;
use ed25519_dalek::SignatureError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Invalid base64")]
    Base64(#[from] DecodeError),

    #[error("Invalid key or signature")]
    Signature(#[from] SignatureError),

    #[error("Invalid JSON")]
    Serde(#[from] SerdeError),

    #[error("Signature verification failed")]
    InvalidSignature,

    #[error("Error converting to slice")]
    ErrorToSlice,
}
