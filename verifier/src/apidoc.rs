use shared::types::{Nonce, NoncePayload, VerifySignaturePayload, VerifySignatureResponse};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::verify_handler_request::verify_signature_handler,
    ),
    components(
        schemas(VerifySignaturePayload, NoncePayload, VerifySignatureResponse, Nonce)
    ),
    tags(
        (name = "Verification", description = "API for verifying signed payloads")
    )
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use serde_json::Value;
    use utoipa::OpenApi;

    use super::ApiDoc;

    #[test]
    fn all_refs_are_resolvable() {
        let openapi = ApiDoc::openapi();
        let json = serde_json::to_value(&openapi).expect("Failed to convert OpenAPI to JSON");

        let mut refs = vec![];
        collect_refs(&json, &mut refs);

        let defined_schemas: HashSet<String> = json
            .pointer("/components/schemas")
            .and_then(|v| v.as_object())
            .map(|map| map.keys().cloned().collect())
            .unwrap_or_default();

        for r in refs {
            if let Some(name) = r.strip_prefix("#/components/schemas/") {
                assert!(defined_schemas.contains(name), "Missing schema definition for $ref: {}", r);
            }
        }
    }

    fn collect_refs(value: &Value, refs: &mut Vec<String>) {
        match value {
            Value::Object(map) => {
                for (key, val) in map {
                    if key == "$ref" {
                        if let Value::String(s) = val {
                            refs.push(s.clone());
                        }
                    } else {
                        collect_refs(val, refs);
                    }
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    collect_refs(v, refs);
                }
            }
            _ => {}
        }
    }
}
