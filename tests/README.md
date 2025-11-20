# Tests

Test suite for Zed Copilot: unit tests, E2E tests with contract validation, and integration tests.

## Structure

```
tests/
├── README.md                # This file
├── common/
│   ├── mod.rs              # Shared test utilities
│   └── openapi.rs          # OpenAPI spec parser & validator
├── e2e_helpers.rs          # Wiremock server setup
├── openai_e2e.rs           # OpenAI E2E tests (19)
├── anthropic_e2e.rs        # Anthropic E2E tests (21)
├── integration_tests.rs    # Integration tests (14)
└── schemas/
    ├── openai.yml          # Downloaded OpenAI spec
    └── .openai-spec.sha256 # Spec checksum
```

## Quick Start

```bash
# Run all tests
cargo test

# Run only E2E tests
cargo test --test openai_e2e --test anthropic_e2e

# Download fresh OpenAI spec
../scripts/download-openai-spec.sh
```

## E2E Tests

Contract-driven testing with wiremock and OpenAPI spec validation.

**Key Features:**
- Validates mocks against live OpenAI API specs
- Zero external API calls (uses wiremock)
- Deterministic, cost-free testing
- ~300ms execution time for 40 tests

**Example Test:**

```rust
#[tokio::test]
async fn test_openai_completion() {
    let ctx = E2ETestContext::new().await;
    
    let response = json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "choices": [...]
    });
    
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&ctx.mock_server)
        .await;
}
```

## OpenAPI Spec Parser

The `common/openapi.rs` module provides:

```rust
// Load spec
let spec = OpenApiSpec::from_file("tests/schemas/openai.yml")?;

// Get schema
let schema = spec.get_schema("CreateChatCompletionResponse");

// Validate JSON against schema
spec.validate_json(&response, &schema)?;

// Find endpoint schema
let schema = spec.get_endpoint_schema("/chat/completions", "POST");
```

**Preprocessing:**
- Handles large integers (> i64::MAX)
- YAML to JSON conversion
- Schema reference resolution

## Debugging

```bash
# Single test with output
cargo test test_openai_completion_contract_validation -- --nocapture

# With backtrace
RUST_BACKTRACE=1 cargo test --test openai_e2e

# No parallelization (debugging)
cargo test --test openai_e2e -- --test-threads=1
```

## Common Issues

**OpenAPI spec fails to load:**
```bash
../scripts/download-openai-spec.sh
```

**Mock server port conflict:**
```bash
lsof -i :8000
kill -9 <PID>
```

**Test timeout:**
```bash
timeout 30 cargo test --test openai_e2e
```

## CI Integration

Tests run automatically on push/PR via `.github/workflows/ci.yml`:

```yaml
- name: Run integration tests
  run: cargo test --test '*' --verbose
```

This includes all E2E tests (`*_e2e.rs` files).

## Resources

- [TESTING.md](../docs/development/TESTING.md) - Complete testing guide
- [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) - HTTP mocking
- [OpenAI API Spec](https://app.stainless.com/api/spec/documented/openai/)