use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq, Eq, Hash, Validate)]
pub struct Nonce {
    #[schema(example = "a734bb9e-9f7b-4f8d-b2b2-5a6e7f3f3021")]
    pub id: String, // UUID v4

    #[schema(example = 1713038460000i64)]
    #[validate(custom = "Nonce::validate_issued_at")]
    pub issued_at: i64, // Timestamp in milliseconds (UTC)
}

impl Nonce {
    pub fn validate_issued_at(issued_at: i64) -> Result<(), ValidationError> {
        let now = Utc::now().timestamp_millis();

        let max_skew = 30_000; // 30s
        if issued_at > now {
            return Err(ValidationError::new("issued_at_in_future"));
        }

        if now - issued_at > max_skew {
            return Err(ValidationError::new("expired_nonce"));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct NoncePayload {
    #[validate]
    pub nonce: Nonce,
    #[validate(length(min = 1))]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VerifySignaturePayload {
    pub nonce_payload: NoncePayload,
    pub signature: String,
    pub public_key: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifySignatureResponse {
    pub message: String,
}
