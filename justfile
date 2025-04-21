# Run holder binary
holder:
    cargo run --package holder --bin holder

# Run several holder binaries
several-holders:
    cargo run --package holder --bin holder
    cargo run --package holder --bin holder
    cargo run --package holder --bin holder
    cargo run --package holder --bin holder
    cargo run --package holder --bin holder

# Run verifier binary
verifier:
    cargo run -p verifier

# Run clippy with warnings as errors
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
test:
    cargo test --workspace

test-replay-attack:
    cargo run --package holder --bin test_manual_replay_attack

test-expired-nonce:
    cargo run --package holder --bin test_manual_expired_nonce

test-rate-limit:
    cargo run --package holder --bin test_manual_rate_limit