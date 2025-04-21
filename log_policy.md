# Logging Policy

This project uses structured logging (via the `tracing` crate) to provide clear observability and debugging capabilities. This policy defines what gets logged, what doesn't, and why — to ensure logs are helpful, secure, and production-safe.

---

## 🎯 Goals of Logging

- ✅ Trace requests and actions across services (holder, verifier)
- ✅ Debug cryptographic verification flows
- ✅ Detect replay attempts using nonces
- ✅ Provide context for failures without leaking sensitive data

---

## 🧠 What We Log

| Field         | Logged? | Rationale                                         |
|---------------|---------|---------------------------------------------------|
| `trace_id` (nonce UUID) | ✅      | Unique per request. Useful for tracking and debugging. |
| `issued_at` (timestamp) | ✅      | Temporal context. Can be used for TTL validation.       |
| Partial `public_key` (first 8 chars) | ✅      | To identify the originator without exposing full key.   |
| Verification result (`ok` / `error`) | ✅      | Status of cryptographic validation.                     |
| Request body / signature / full keys | ❌      | Sensitive or redundant. Never logged.                  |

---

## ❌ What We Never Log

- ❌ Full `public_key`
- ❌ Private key (obviously)
- ❌ Full signed payload
- ❌ Raw signature
- ❌ Stack traces in production (unless explicitly allowed)

---

## 🔒 Security Notes

- Nonces are UUIDv4 + timestamp for freshness and uniqueness.
- Logged data is public by nature and cannot be used to impersonate or derive private keys.
- This policy aligns with best practices for cryptographic services and verifiable authentication systems.

---

## 🔧 Logging Format

Logs are formatted in multi-line structured blocks using `tracing_subscriber` with `.pretty()` enabled.

Example:

  2025-04-20T18:05:26.277488Z  INFO verifier::verify_handler_request: Signature is valid and nonce accepted, trace_id: b993e541-9d20-4592-b155-71fa8f061892, issued_at: 1745172326267, pubkey: wbWUQaDy
    at verifier/src/verify_handler_request.rs:43

## 🛠️ Dev Logging vs Production

| Environment  | Log Level | Notes                                      |
|--------------|-----------|--------------------------------------------|
| Local / Dev  | `info`, `debug` | Includes payload summaries for dev use |
| CI / Staging | `info`    | No sensitive values logged                |
| Production   | `warn`, `error` | Logs reduced to essential failures     |

---

## ✅ Summary

This logging policy provides clear insight during development and debugging without compromising user security or cryptographic integrity. It's designed for real-world systems and scalable service architectures.
