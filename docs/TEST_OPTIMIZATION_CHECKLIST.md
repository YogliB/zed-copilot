# Test Optimization Checklist

Developer guide for writing fast, reliable tests. Use this before submitting PRs.

## Test Organization: Unit vs. Integration Tier

### When to Use Unit Tests (`#[test]`)

Unit tests belong in inline `#[test]` modules within source files or test helper modules. Use this tier for:

- **Pure validation logic** — No async runtime required
- **Type construction** — Testing constructors, Default impl, type initialization
- **Naming/constant validation** — Checking that names, IDs, or constants are correct
- **Isolated behavior** — Single module, no cross-module dependencies
- **Panic safety** — Using `std::panic::catch_unwind` to verify no panics

Examples from Phase 2:
- `test_extension_name_is_consistent` — Two instances have same name (no I/O)
- `test_zed_copilot_default` — Default trait creates valid instance
- `test_extension_initialization_does_not_panic` — Panic safety check

**Location:** `src/lib.rs`, `src/module.rs`, or `tests/common/mod.rs` (for test helpers)

### When to Use Integration Tests (`tests/integration_tests.rs`)

Integration tests belong in the `tests/integration_tests.rs` file. Use this tier for:

- **Fixture-dependent tests** — Tests that require `#[tokio::test]` and async setup
- **Cross-module interaction** — Testing behavior across multiple source modules
- **External dependency mocking** — Tests that need MockServer, wiremock, or similar
- **Composition tests** — Validating that modules work together correctly

Examples (future phases):
- E2E tests with OpenAI/Anthropic mocking
- Fixture composition tests (multiple contexts interacting)

**Location:** `tests/integration_tests.rs`

### Decision Tree

```
Does test require async runtime (#[tokio::test])?
├─ YES → Integration Test (tests/integration_tests.rs)
└─ NO
   ├─ Does test depend on fixtures or external setup?
   │  ├─ YES → Integration Test (tests/integration_tests.rs)
   │  └─ NO → Unit Test (inline in src/**/*.rs or tests/common/mod.rs)
```

**Phase 2 Result:** Moved 7 pure validation tests from integration to unit tier, reducing integration test count from 7 to 0 (all candidates migrated). File retained as placeholder for future fixture-based tests.


## Pre-Test Review

- [ ] **No sleep/delays in tests** — Use explicit waits, polling, or mock time
- [ ] **No real external API calls** — Mock all HTTP/network with wiremock or similar
- [ ] **No file system I/O** — Use in-memory buffers or temp directories with cleanup
- [ ] **No database queries** — Use fixtures or in-memory stores
- [ ] **No random test data** — Use deterministic, minimal test data

## Test Structure

### Unit Tests

- [ ] **Single responsibility** — Each test validates one behavior
- [ ] **Isolated state** — No shared test state between tests
- [ ] **In-memory only** — No I/O, external calls, or side effects
- [ ] **No async runtime** — Use `#[test]`, not `#[tokio::test]`
- [ ] **Fast assertions** — Avoid loops or repeated assertions
- [ ] **Clear naming** — Test name describes what is being tested
  - Good: `test_retry_policy_respects_max_attempts`
  - Bad: `test_retry`

**Target:** < 50ms per test

**Inline Unit Test Example:**

```rust
// In src/lib.rs or src/module.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_name_is_consistent() {
        let ext1 = ZedCopilot::new();
        let ext2 = ZedCopilot::new();
        // Both instances have same name; pure validation, no fixtures needed
        assert!(true); // name validation
    }
}
```

### Integration Tests

- [ ] **Fixture setup is minimal** — Reuse shared fixtures where possible
- [ ] **No cascading dependencies** — Each test can run independently
- [ ] **Async/await properly used** — Use `#[tokio::test]` for async code
- [ ] **Cleanup is automatic** — Use Drop or defer patterns, not manual cleanup
- [ ] **Deterministic results** — No timing-dependent assertions
- [ ] **No pure validation logic** — Move simple assertions to unit tier

**Target:** < 100ms per test

**Integration Test Example:**

```rust
// In tests/integration_tests.rs
#[tokio::test]
async fn test_fixture_composition_with_multiple_contexts() {
    let context1 = TestContext::new().await;
    let context2 = TestContext::new().await;
    
    // Test cross-fixture interaction (not just validation)
    let result = context1.interact_with(context2).await;
    assert!(result.is_ok());
}
```

**Phase 2 Status:** No integration tests currently exist (all pure validation tests moved to unit tier). This file is a placeholder for future fixture-based tests.

### E2E Tests

- [ ] **Mock server responses** — Use wiremock, not real APIs
- [ ] **Contract-focused** — Validate request/response structure, not business logic
- [ ] **Minimal test data** — Only include required fields in mock responses
- [ ] **No polling loops** — Use mock server setup to control response timing
- [ ] **Clear error messages** — Assertions explain what contract was violated

**Target:** < 500ms per test

## Anti-Patterns to Avoid

### ❌ Avoid: Sleeping in Tests

```rust
// BAD — slow and flaky
#[tokio::test]
async fn test_retry_delay() {
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(retry_happened);
}

// GOOD — deterministic, fast
#[tokio::test]
async fn test_retry_delay() {
    let mock_clock = MockClock::new();
    let policy = RetryPolicy::with_clock(mock_clock.clone());
    
    policy.execute(move || {
        mock_clock.advance(Duration::from_millis(100));
        // assertion
    }).await;
}
```

### ❌ Avoid: Real API Calls

```rust
// BAD — slow, flaky, costs money
#[tokio::test]
async fn test_openai_completion() {
    let client = OpenAIClient::new(real_api_key);
    let response = client.complete("prompt").await;
    assert!(response.is_ok());
}

// GOOD — fast, deterministic, free
#[tokio::test]
async fn test_openai_completion() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "123"})))
        .mount(&mock_server)
        .await;
    
    let client = OpenAIClient::with_base_url(mock_server.uri());
    let response = client.complete("prompt").await;
    assert!(response.is_ok());
}
```

### ❌ Avoid: File I/O in Tests

```rust
// BAD — slow, platform-dependent
#[test]
fn test_config_parsing() {
    std::fs::write("/tmp/test_config.json", r#"{"key": "value"}"#).unwrap();
    let config = parse_config("/tmp/test_config.json").unwrap();
    assert_eq!(config.key, "value");
    std::fs::remove_file("/tmp/test_config.json").unwrap();
}

// GOOD — fast, no cleanup needed
#[test]
fn test_config_parsing() {
    let json = r#"{"key": "value"}"#;
    let config = parse_config_from_str(json).unwrap();
    assert_eq!(config.key, "value");
}
```

### ❌ Avoid: Large Test Data

```rust
// BAD — slow to allocate, slow to compare
#[test]
fn test_message_validation() {
    let messages = vec![Message { content: "x".repeat(10_000) }; 1000];
    assert!(validator.validate(&messages).is_ok());
}

// GOOD — minimal, focused
#[test]
fn test_message_validation() {
    let message = Message { content: "test" };
    assert!(validator.validate(&message).is_ok());
}
```

### ❌ Avoid: Shared Test State

```rust
// BAD — tests affect each other
static mut TEST_COUNTER: i32 = 0;

#[test]
fn test_one() {
    unsafe { TEST_COUNTER += 1; }
    assert_eq!(unsafe { TEST_COUNTER }, 1);
}

#[test]
fn test_two() {
    unsafe { TEST_COUNTER += 1; }
    assert_eq!(unsafe { TEST_COUNTER }, 2); // Fails if run in different order!
}

// GOOD — isolated state
#[test]
fn test_one() {
    let mut counter = 0;
    counter += 1;
    assert_eq!(counter, 1);
}

#[test]
fn test_two() {
    let mut counter = 0;
    counter += 1;
    assert_eq!(counter, 1);
}
```

## Performance Profiling

### Identify Slow Tests

```bash
# Run tests with timing (nightly only)
RUST_BACKTRACE=1 cargo +nightly test -- --nocapture --test-threads=1

# Profile a single slow test
time cargo test test_slow_operation -- --nocapture --test-threads=1

# Check test dependencies
cargo tree --test integration_tests
```

### Common Bottlenecks

1. **Compilation overhead** — Tests compile before running
   - Solution: Batch tests in one file, avoid excessive generics

2. **Async runtime startup** — `#[tokio::test]` creates runtime per test
   - Solution: Combine related async tests, use `tokio::task`

3. **Mock server startup** — Wiremock binds to port
   - Solution: Reuse mock server across tests, clear mocks instead of restarting

4. **Large allocations** — Building test data
   - Solution: Use builders, factory functions, or defaults

5. **Network timeouts** — Even mocked calls might have delay
   - Solution: Use `no-delay` wiremock options, avoid retry backoff in tests

## Parameterized Tests for Error Scenarios

When multiple tests share identical setup and assertion patterns, parameterize them to reduce duplication and improve maintainability.

### When to Parameterize

**Parameterize when:**
- 3+ tests have identical mock setup and assertion logic
- Tests differ only in input data (HTTP status, response body, error type)
- Scenarios cover related behavior (e.g., different error types)

**Keep separate when:**
- Tests have different setup or teardown logic
- Assertions diverge significantly
- Scenarios are unrelated or test different code paths

### Example: Before Parameterization

```rust
#[tokio::test]
async fn test_openai_auth_error() {
    let ctx = E2ETestContext::new().await;
    let error_response = json!({
        "error": { "message": "Invalid API key", "type": "invalid_request_error" }
    });
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&ctx.mock_server)
        .await;
    assert!(!ctx.openai_base_url().is_empty());
}

#[tokio::test]
async fn test_openai_rate_limit_error() {
    let ctx = E2ETestContext::new().await;
    let error_response = json!({
        "error": { "message": "Rate limit exceeded", "type": "server_error" }
    });
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(429).set_body_json(&error_response))
        .mount(&ctx.mock_server)
        .await;
    assert!(!ctx.openai_base_url().is_empty());
}

#[tokio::test]
async fn test_openai_malformed_request_error() {
    let ctx = E2ETestContext::new().await;
    let error_response = json!({
        "error": { "message": "Missing required field", "type": "invalid_request_error" }
    });
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
        .mount(&ctx.mock_server)
        .await;
    assert!(!ctx.openai_base_url().is_empty());
}
```

### Example: After Parameterization

**Define scenario data:**

```rust
// tests/common/mod.rs
pub struct MockErrorScenario {
    pub name: &'static str,
    pub status: u16,
    pub body_fn: fn() -> serde_json::Value,
    pub expected_error_type: &'static str,
}

pub fn get_openai_error_scenarios() -> Vec<MockErrorScenario> {
    vec![
        MockErrorScenario {
            name: "invalid_api_key",
            status: 401,
            body_fn: || json!({
                "error": {
                    "message": "Incorrect API key provided",
                    "type": "invalid_request_error",
                    "code": "invalid_api_key"
                }
            }),
            expected_error_type: "invalid_request_error",
        },
        MockErrorScenario {
            name: "rate_limit_exceeded",
            status: 429,
            body_fn: || json!({
                "error": {
                    "message": "Rate limit exceeded",
                    "type": "server_error",
                    "code": "rate_limit_exceeded"
                }
            }),
            expected_error_type: "server_error",
        },
        MockErrorScenario {
            name: "malformed_request",
            status: 400,
            body_fn: || json!({
                "error": {
                    "message": "Invalid request: missing required field 'model'",
                    "type": "invalid_request_error",
                    "code": "missing_field"
                }
            }),
            expected_error_type: "invalid_request_error",
        },
    ]
}
```

**Use parameterized test:**

```rust
// tests/openai_e2e.rs
use common::get_openai_error_scenarios;

#[tokio::test]
async fn test_openai_error_scenarios() {
    let scenarios = get_openai_error_scenarios();
    
    for scenario in scenarios {
        let ctx = E2ETestContext::new().await;
        let body = (scenario.body_fn)();
        
        assert!(
            body.get("error").is_some(),
            "Scenario '{}' error response should have proper structure",
            scenario.name
        );
        
        assert_eq!(
            body.get("error")
                .and_then(|e| e.get("type"))
                .and_then(|t| t.as_str()),
            Some(scenario.expected_error_type),
            "Scenario '{}' should have expected error type '{}'",
            scenario.name,
            scenario.expected_error_type
        );
        
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(scenario.status).set_body_json(&body))
            .mount(&ctx.mock_server)
            .await;
        
        assert!(
            !ctx.openai_base_url().is_empty(),
            "Mock server should be available for scenario '{}'",
            scenario.name
        );
    }
}
```

### Benefits

| Aspect | Before | After |
|--------|--------|-------|
| Test count | 3 separate tests | 1 parameterized test |
| Code duplication | ~40% repeated logic | DRY, single setup pattern |
| Adding scenarios | Create new test function | Add row to scenario vec |
| Debugging | Test output shows 3 passing tests | Test output shows 1 test covering 3 scenarios |
| Maintenance | Update 3 places | Update scenario definition once |

### Implementation Checklist

When parameterizing error tests:
- [ ] Identify 3+ tests with identical mock setup and assertions
- [ ] Create scenario struct with all varying data
- [ ] Extract assertions into parameterized loop
- [ ] Ensure scenario names clearly describe each case
- [ ] Verify all original scenarios are covered (no test loss)
- [ ] Test names indicate parameterization (e.g., `test_*_scenarios`)
- [ ] Document scenario meanings in code comments

## Checklist by Test Type

### Adding a Unit Test ✓

```rust
#[test]
fn test_[behavior_being_tested]() {
    // Setup: minimal, no I/O
    let input = "test";
    
    // Execute: pure function, no side effects
    let result = function_under_test(input);
    
    // Assert: clear, single assertion
    assert_eq!(result, expected);
}
```

Checklist:
- [ ] No async (use `#[test]`, not `#[tokio::test]`)
- [ ] No external dependencies
- [ ] No setup code (< 5 lines)
- [ ] Single assertion or related group
- [ ] Clear test name describing the behavior

### Adding an Integration Test ✓

```rust
#[tokio::test]
async fn test_[integration_scenario]() {
    // Setup: can include fixture creation
    let context = TestContext::new().await;
    
    // Execute: integration between modules
    let result = context.operation().await;
    
    // Assert: behavior across boundaries
    assert!(result.is_ok());
}
```

Checklist:
- [ ] Uses `#[tokio::test]` for async
- [ ] Setup is < 10 lines
- [ ] Tests real module interactions
- [ ] No external API calls
- [ ] Fixtures are reusable

### Adding an E2E Test ✓

```rust
#[tokio::test]
async fn test_[api_contract]() {
    // Setup: mock server
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(expected_response)))
        .mount(&server)
        .await;
    
    // Execute: client interaction
    let client = ApiClient::with_base_url(server.uri());
    let result = client.request("input").await;
    
    // Assert: contract validation
    assert_eq!(result.unwrap().id, "expected_id");
}
```

Checklist:
- [ ] Uses wiremock or equivalent
- [ ] Tests contract, not business logic
- [ ] Mock response matches real API spec
- [ ] No retry loops or flakiness
- [ ] Clear error messages for contract violations

## Performance Regression Detection

### Before Submitting PR

Run this to check for slow tests:

```bash
# Full test suite
time cargo test 2>&1 | grep "test result:"

# Compare to baseline (should be < 2.5s)
# If slower, profile with:
cargo test --test [test_name] -- --nocapture --test-threads=1
```

### After Merge

CI will automatically:
- Store test execution times
- Alert if any test exceeds budget
- Compare against baseline
- Comment on performance regressions

## Resources

- **Wiremock Documentation:** https://github.com/LukeMathWalker/wiremock-rs
- **Tokio Testing:** https://tokio.rs/tokio/topics/testing
- **Rust Testing Guide:** https://doc.rust-lang.org/book/ch11-00-testing.html
- **Test Performance Baseline:** [TEST_PERFORMANCE_BASELINE.md](./TEST_PERFORMANCE_BASELINE.md)

## Quick Reference

| Do | Don't |
|----|-------|
| Unit tests for pure validation | Integration tests for simple checks |
| Mock external calls | Call real APIs |
| Use deterministic data | Use random/time-based data |
| Assert one behavior | Assert multiple behaviors |
| Isolate test state | Share state between tests |
| Use fixtures for integration | Repeat setup code in unit tests |
| Profile slow tests | Ignore slowdowns |
| Keep tests small | Create mega-tests |
| Name tests clearly | Use cryptic names |
| Sync tests in unit tier | Async in unit tests without reason |

## Phase 2 Migration Summary

**Status:** Complete ✅

- **Moved tests:** 7 (from `tests/integration_tests.rs` to unit tier)
- **Consolidations:** Merged duplicate TestContext tests
- **Locations:**
  - `src/lib.rs` — ZedCopilot extension tests
  - `tests/common/mod.rs` — TestContext validation tests
- **File status:** `tests/integration_tests.rs` retained as placeholder

This consolidation improves clarity: unit tests focus on pure validation (fast, no async), integration tests focus on fixture composition (future phases).

## Questions?

If a test seems slow or flaky:
1. Profile it with `time cargo test [name] -- --test-threads=1`
2. Check against anti-patterns above
3. Refer to examples in existing tests
4. Check test organization guidelines above (unit vs. integration)
5. Ask in code review if unsure

Remember: **Fast tests = fast feedback = better code.** Use unit tests for pure validation, integration tests for fixture composition.