# Tests

Test suite for Zed Copilot: unit tests, E2E tests with mocking, and integration tests.

## Structure

```
tests/
├── README.md                # This file
├── common/
│   └── mod.rs              # Shared test utilities
├── e2e_helpers.rs          # Wiremock server setup
├── openai_e2e.rs           # OpenAI E2E tests (16)
├── anthropic_e2e.rs        # Anthropic E2E tests (21)
└── integration_tests.rs    # Integration tests (14)
```

## Quick Start

```bash
# Run all tests
cargo test

# Run only E2E tests
cargo test --test openai_e2e --test anthropic_e2e

# Run tests with output
cargo test -- --nocapture
```

## E2E Tests

Contract-driven testing with wiremock and explicit assertions.

**Key Features:**
- Zero external API calls (uses wiremock)
- Deterministic, cost-free testing
- Fast execution (~200ms for 37 tests)
- Manual assertions for clarity

**Example Test:**

```rust
#[tokio::test]
async fn test_openai_completion_contract_validation() {
    let ctx = E2ETestContext::new().await;
    
    let response = json!({
        "id": "chatcmpl-123",
        "object": "chat.completion",
        "choices": [...]
    });
    
    assert!(response.get("id").is_some());
    
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .mount(&ctx.mock_server)
        .await;
}
```

## Test Organization

- `openai_e2e.rs` — OpenAI API client tests
  - Response format validation
  - Error handling
  - Request building
  - Message validation

- `anthropic_e2e.rs` — Anthropic API client tests
  - Similar coverage to OpenAI

- `integration_tests.rs` — End-to-end provider tests
  - Full workflow testing

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