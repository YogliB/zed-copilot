# Testing Guide

Complete testing strategy and guidelines for Zed Copilot.

> **Part of:** [Zed Copilot Documentation](../README.md)
>
> **For Contributors:** See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution workflow.

---

## Overview

Zed Copilot uses comprehensive testing to ensure reliability. Tests are organized into unit tests (in `src/`) and integration tests (in `tests/`), following Rust best practices.

**Current Coverage:** 157 tests, all passing ✅

## Test Organization

### Test Types

Zed Copilot uses three types of tests:

1. **Unit Tests** (`src/`) — Component-level testing
2. **E2E Tests** (`tests/*_e2e.rs`) — Contract validation against real API specs
3. **Integration Tests** (`tests/integration_tests.rs`) — Component interaction testing



### Unit Tests

Unit tests are located in `src/lib.rs` and test individual components in isolation.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_behavior() {
        // Test implementation
    }
}
```

**Location:** `src/lib.rs`  
**Run with:** `cargo test --lib`

### Integration Tests

Integration tests are located in the `tests/` directory and test how components work together.

```
tests/
├── common/
│   └── mod.rs          # Shared test utilities and helpers
└── integration_tests.rs # Integration test cases
```

**Location:** `tests/integration_tests.rs` and `tests/common/mod.rs`  
**Run with:** `cargo test --test integration_tests`

### E2E Tests

E2E tests validate provider implementations against live OpenAPI specifications using contract-driven testing.

**Structure:**
```
tests/
├── openai_e2e.rs           # OpenAI provider tests (19 tests)
├── anthropic_e2e.rs        # Anthropic provider tests (21 tests)
├── e2e_helpers.rs          # Wiremock server setup
├── common/
│   └── openapi.rs          # OpenAPI spec parser & validator
└── schemas/
    └── openai.yml          # Downloaded OpenAI spec (auto-updated)
```

**Key Features:**
- Contract validation against real API specs
- HTTP mocking with wiremock (zero external calls)
- OpenAPI spec parser validates mock responses
- Deterministic, cost-free testing

**Setup:**
```bash
./scripts/download-openai-spec.sh  # Download fresh OpenAI spec
cargo test --test openai_e2e --test anthropic_e2e
```

**Writing E2E Tests:**

Every E2E test follows this pattern:

```tests/openai_e2e.rs#L1-20
#[tokio::test]
async fn test_name() {
    let ctx = E2ETestContext::new().await;
    
    let mock_response = json!({
        "id": "test-id",
        "object": "chat.completion",
        "choices": [...]
    });
    
    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&ctx.mock_server)
        .await;
}
```

**OpenAPI Spec Management:**

Download and validate the latest OpenAI API spec:
```bash
./scripts/download-openai-spec.sh
```

This script:
- Downloads spec from https://app.stainless.com/api/spec/documented/openai/openapi.documented.yml
- Saves to `tests/schemas/openai.yml`
- Calculates SHA256 checksum
- Validates YAML syntax

**Debugging E2E Tests:**

```bash
cargo test test_openai_completion_contract_validation -- --nocapture  # Single test
RUST_BACKTRACE=1 cargo test --test openai_e2e  # With backtrace
cargo test --test openai_e2e -- --test-threads=1  # No parallelization
```

**Common Issues:**

1. **OpenAPI spec fails to load**: Re-download with `./scripts/download-openai-spec.sh`
2. **Mock server fails to start**: Check port conflicts with `lsof -i :8000`
3. **Test times out**: Run with `timeout 30 cargo test --test openai_e2e`

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Unit Tests Only
```bash
cargo test --lib
```

### Run E2E Tests Only
```bash
cargo test --test openai_e2e --test anthropic_e2e
```

### Run Integration Tests Only
```bash
cargo test --test integration_tests
```

### Run Specific Test by Name
```bash
cargo test test_zed_copilot_new
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Tests in Single-threaded Mode
```bash
cargo test -- --test-threads=1 --nocapture
```

## Test Structure

### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zed_copilot_new() {
        let extension = ZedCopilot::new();
        let _ = extension;
    }

    #[test]
    fn test_zed_copilot_default() {
        let extension = ZedCopilot::default();
        let _ = extension;
    }
}
```

### Integration Test Example

```rust
mod common;

use common::TestContext;

#[test]
fn test_extension_compiles_and_loads() {
    let context = TestContext::new();
    assert_eq!(context.extension_name, "zed-copilot");
}
```

## Test Utilities

### TestContext

The `TestContext` struct provides a reusable test harness for integration tests.

**Usage:**
```rust
let context = TestContext::new();
assert_eq!(context.extension_name, "zed-copilot");
```

**Location:** `tests/common/mod.rs`

### Common Test Helpers

**`create_test_extension()`**
- Initializes test fixtures
- Useful for setup in multiple tests

**Location:** `tests/common/mod.rs`

## Writing New Tests

### Guidelines

1. **Name tests descriptively**: Test names should indicate what behavior is being tested
   - ✅ `test_zed_copilot_new`
   - ❌ `test_1`

2. **Keep tests focused**: Each test should verify one behavior
   - ✅ One assertion per test (or tightly related assertions)
   - ❌ Multiple unrelated behaviors in one test

3. **Use clear assertions**: Make expected behavior explicit
   - ✅ `assert_eq!(context.extension_name, "zed-copilot")`
   - ❌ `assert!(true)`

4. **Avoid test interdependencies**: Tests should run independently
   - ✅ Each test creates its own fixtures
   - ❌ Tests relying on order or shared state

### Adding a Unit Test

Add tests directly to the module in `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_behavior() {
        // Arrange
        let extension = ZedCopilot::new();

        // Act
        let _ = extension;

        // Assert
        // Verify behavior
    }
}
```

### Adding an Integration Test

Add tests to `tests/integration_tests.rs`:

```rust
#[test]
fn test_new_integration() {
    // Setup
    let context = TestContext::new();

    // Test
    assert_eq!(context.extension_name, "zed-copilot");
}
```

Or use common utilities:

```rust
#[test]
fn test_with_helpers() {
    common::create_test_extension();
    // Test behavior
}
```

## Assertions

### Common Assertions

| Assertion | Use Case |
|-----------|----------|
| `assert!(condition)` | Verify a boolean condition is true |
| `assert_eq!(left, right)` | Verify two values are equal |
| `assert_ne!(left, right)` | Verify two values are not equal |
| `panic!()` | Force a test to fail with a message |

### Assertion Examples

```rust
#[test]
fn test_assertions() {
    // Boolean assertion
    assert!(true);

    // Equality assertion
    assert_eq!(2 + 2, 4);

    // Inequality assertion
    assert_ne!(2 + 2, 5);

    // With custom message
    assert!(true, "This is a custom failure message");
}
```

## Testing Panic Behavior

To verify that code panics correctly:

```rust
#[test]
fn test_extension_initialization_does_not_panic() {
    let result = std::panic::catch_unwind(|| {
        let _extension = ZedCopilot::new();
    });
    assert!(result.is_ok());
}
```

## Code Quality Checks

### Format Code
```bash
cargo fmt
```

### Check for Warnings
```bash
cargo clippy
```

### Full Quality Check
```bash
cargo fmt && cargo clippy && cargo test
```

## Current Test Coverage

**Phase 2.4: E2E Tests (40)** ✅
- OpenAI provider E2E tests (19)
- Anthropic provider E2E tests (21)
- Contract validation against OpenAPI specs
- HTTP mocking with wiremock

**Phase 2.1: Provider Tests (31)** ✅
- Provider instantiation and trait implementation
- OpenAI and Anthropic provider implementations
- Provider factory pattern
- Error handling with ProviderError enum

**Phase 1: Extension Tests (9)** ✅
- Extension struct creation and initialization
- Trait implementation verification
- Integration with Zed extension API

**Total: 157 tests, 100% pass rate** ✅

## Test Expansion by Phase

### Phase 2.2: Configuration & Credentials (Current)

**Tests to Add:**
- `test_config_loading_from_zed_settings` — Load settings.json correctly
- `test_config_validation_required_fields` — Fail on missing required fields
- `test_environment_variable_interpolation` — Replace `${VAR_NAME}` in config
- `test_api_key_validation` — Validate API key format
- `test_provider_selection_from_config` — Load correct provider from settings
- `test_invalid_config_error_messages` — Helpful error messages

**Target:** 50+ total tests, 90%+ coverage
**Note:** Configuration tests use mocked Zed settings API

### Phase 2.3: HTTP Integration & Streaming

**Tests to Add:**
- `test_http_request_formatting_openai` — Build correct OpenAI API request
- `test_http_request_formatting_anthropic` — Build correct Anthropic API request
- `test_streaming_response_parsing` — Parse SSE streaming responses
- `test_streaming_token_buffering` — Buffer and emit tokens correctly
- `test_retry_logic_exponential_backoff` — Retry with correct delays
- `test_retry_max_attempts` — Stop after max retries
- `test_network_error_handling` — Handle connection failures gracefully
- `test_timeout_handling` — Respect configured timeouts
- `test_rate_limit_handling` — Respect rate limit headers

**Target:** 65+ total tests, 90%+ coverage
**Note:** HTTP tests use mocked reqwest client

### Phase 3: Chat Interface (Primary Feature)

**Tests to Add:**
- `test_message_struct_creation` — Create Message with role and content
- `test_message_history_fifo_order` — Messages stored in correct order
- `test_history_truncation_on_limit` — Truncate old messages when limit exceeded
- `test_chat_engine_single_turn` — Handle one-turn conversation
- `test_chat_engine_multi_turn` — Handle multi-turn conversation with context
- `test_streaming_response_to_ui` — Stream responses update UI in real-time
- `test_context_extraction_file` — Extract current file content
- `test_context_extraction_selection` — Extract selected text
- `test_context_injection_into_prompt` — Include context in system prompt
- `test_chat_error_handling_recoverable` — Retry on transient errors
- `test_chat_error_handling_permanent` — Display error message on permanent failure
- `test_chat_state_persistence` — Save and restore conversation history
- `test_chat_ui_message_display` — Display messages in panel with formatting
- `test_chat_ui_user_input` — Handle user typing and submission
- `test_chat_ui_streaming_display` — Update display as tokens arrive

**Target:** 100+ total tests, 85%+ coverage
**Note:** Chat tests will require Zed UI API mocking

### Phase 4+: Advanced Features (Optional)

**Potential Tests:**
- Code completion trigger and display
- Custom system prompts
- Refactoring suggestions
- Test generation
- Documentation generation
- Multi-language support

## Debugging Tests

### View Test Output
```bash
cargo test -- --nocapture
```

### Run Single Test with Output
```bash
cargo test test_name -- --nocapture
```

### Enable Backtrace
```bash
RUST_BACKTRACE=1 cargo test
```

### Run Tests in Order (No Parallelization)
```bash
cargo test -- --test-threads=1
```

## Continuous Integration

Tests automatically run in CI on:
- Pull requests
- Commits to main branch
- Releases

See `.github/workflows/` for CI configuration (to be set up in Phase 1).

## Performance Testing

For performance-critical code, use Rust's built-in benchmarks:

```rust
#![feature(test)]
extern crate test;

#[bench]
fn bench_expensive_operation(b: &mut test::Bencher) {
    b.iter(|| {
        // Code to benchmark
    });
}
```

Run with:
```bash
cargo bench --lib
```

## Known Limitations

### WebAssembly (WASM) Testing

Since this is a `cdylib` crate (compiled to WASM), some standard library features are unavailable:
- No networking in unit tests
- Limited file I/O in unit tests
- No process spawning

Workaround: Use integration tests and mock external dependencies.

### Zed API Testing

The Zed extension API cannot be fully tested in isolation:
- Manual testing required with actual Zed IDE
- Run as dev extension: `zed --foreground`
- Check logs: `zed: open log`

## Resources

- [Rust Book: Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust Reference: Testing](https://doc.rust-lang.org/reference/attributes/testing.html)
- [Zed Extension API](https://zed.dev/docs/extensions)
- [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs) — HTTP mocking for E2E tests
- [OpenAI API Reference](https://platform.openai.com/docs/api-reference)
- [Anthropic Claude API](https://docs.anthropic.com/claude/reference)

---

**Last Updated:** 2024  
**Maintained by:** Zed Copilot Contributors

---

**Back to:** [Development](../README.md#quick-navigation)