# Zed Copilot Testing Guide

## Overview

This guide documents the testing strategy for Zed Copilot. Tests are organized into unit tests (in `src/`) and integration tests (in `tests/`), following Rust best practices.

## Test Organization

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
**Run with:** `cargo test --test '*'`

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Unit Tests Only
```bash
cargo test --lib
```

### Run Integration Tests Only
```bash
cargo test --test '*'
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

**Unit Tests (5)**
- `test_zed_copilot_new` — Verify basic instantiation
- `test_zed_copilot_default` — Verify Default trait implementation
- `test_extension_trait_new` — Verify Extension trait implementation
- `test_multiple_instances` — Verify multiple instances can coexist
- `test_extension_initialization_does_not_panic` — Verify no panic on init

**Integration Tests (9)**
- `test_extension_compiles_and_loads` — Verify compilation
- `test_extension_can_be_created_via_default` — Verify default creation
- `test_extension_does_not_panic_on_creation` — Verify error handling
- `test_test_context_can_be_created` — Verify test utilities
- `test_multiple_contexts_can_coexist` — Verify context independence
- `test_test_context_default_implementation` — Verify context defaults
- `test_extension_name_is_consistent` — Verify consistency
- Plus 2 additional tests from common module

**Total: 14 tests**

## Future Test Expansion

As features are added, new tests should cover:

- **Phase 2 (v0.1.0)**: AI Provider Integration
  - `test_openai_provider_initialization`
  - `test_anthropic_provider_initialization`
  - `test_api_key_configuration`
  - `test_provider_error_handling`

- **Phase 3 (v0.2.0)**: Code Completion
  - `test_completion_trigger_logic`
  - `test_context_extraction`
  - `test_response_formatting`
  - `test_caching_strategy`

- **Phase 4+**: Advanced Features
  - `test_multi_language_support`
  - `test_custom_prompts`
  - `test_refactoring_suggestions`

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

---

**Last Updated:** 2024  
**Maintained by:** Zed Copilot Contributors