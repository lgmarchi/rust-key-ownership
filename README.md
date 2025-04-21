# Rust Key Ownership

A distributed system implemented in Rust that manages key ownership through multiple microservices. The system implements secure key management with replay attack protection and nonce-based verification.

## Project Overview

This project demonstrates advanced Rust concepts including:

- Microservices architecture
- Secure key management
- Replay attack protection
- Asynchronous programming
- REST API development
- Comprehensive error handling

## Project Structure

The project is organized as a Rust workspace with the following components:

- `holder/`: Service responsible for key management and storage
  - Handles key creation and validation
  - Implements replay attack protection
  - Manages nonce verification
- `verifier/`: Service for key verification
  - Validates key ownership
  - Processes verification requests
  - Implements security checks
- `shared/`: Shared library between services
  - Common types and utilities
  - Shared validation logic
  - Error types

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- Just (Command runner) - Install with `cargo install just` (optional but recommended)

## Main Dependencies

- `axum`: Modern web framework for building APIs
- `tokio`: Asynchronous runtime
- `tracing`: Logging and diagnostics
- `utoipa`: OpenAPI/Swagger documentation generation
- `validator`: Data validation
- Additional dependencies can be found in `Cargo.toml`

## Environment Setup

1. Clone the repository:

```bash
git clone https://github.com/lgmarchi/rust-key-ownership.git
cd rust-key-ownership
```

2. Environment configuration:
   - Create a `.env` file in the project root
   - Configure required environment variables:

     ```env
     VERIFY_SIGNATURE_API_URL=http://localhost:3000/api/verify-signature
     LOG_LEVEL=info
     ```

## Running the Application

### Using Just Commands (Recommended)

1. Start the verifier service first:

```bash
just verifier
```

2. In a new terminal, start the holder service:

```bash
just holder
```

If preferred, you can also run several holder instances:

```bash
just several-holders
```

3. You are going to see the results in the verifier terminal with logs.

### Manual Testing Scenarios

The project includes pre-configured test scenarios for security testing:

1. Test Replay Attack Scenario:

```bash
just test-replay-attack
```

2. Test Expired Nonce Scenario:

```bash
just test-expired-nonce
```

### Alternative: Manual Execution

You can also run services individually using Cargo:

```bash
# Run verifier
cargo run -p verifier

# Run holder
cargo run --package holder --bin holder
```

## Development

### Code Quality Tools

The project uses several tools to maintain code quality:

- `rustfmt`: Code formatting
- `rusty-hook`: Git hooks for pre-commit checks
- `clippy`: Rust linter
- Automated tests

### Common Development Commands

```bash
# Format code
cargo fmt

# Run linter
just clippy

# Run tests
just test

# Run specific test
cargo test -p <package-name> <test-name>
```

## API Documentation

The API is documented using OpenAPI/Swagger. Once the services are running, you can access the API json at:

- `http://localhost:3000/api-docs/openapi.json`

## Logging

The project implements comprehensive logging following specific policies detailed in [log_policy.md](log_policy.md). Log levels can be configured through the environment variables.

## Error Handling

The system implements robust error handling with:

- Custom error types
- Detailed error messages
- Proper HTTP status codes
- Error logging and tracing

## Security Features

- Replay attack protection
- Nonce-based verification
- Secure key storage
- Request validation
- Rate limiting

## Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines

- Follow Rust best practices
- Add tests for new features
- Update documentation as needed
- Use descriptive commit messages

## Troubleshooting

Common issues and solutions:

1. Service Connection Issues
   - Ensure verifier is running before holder
   - Check port configurations
   - Verify environment variables

2. Build Errors
   - Run `cargo clean` and rebuild
   - Update Rust to latest stable version
   - Check dependency versions

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

The MIT License is a permissive license that is short and to the point. It lets people do anything they want with your code as long as they provide attribution back to you and don't hold you liable.
t
