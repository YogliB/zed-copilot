# Test Optimization Checklist

Developer guide for writing fast, reliable tests. Use this before submitting PRs.

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
- [ ] **Fast assertions** — Avoid loops or repeated assertions
- [ ] **Clear naming** — Test name describes what is being tested
  - Good: `test_retry_policy_respects_max_attempts`
  - Bad: `test_retry`

**Target:** < 50ms per test

### Integration Tests

- [ ] **Fixture setup is minimal** — Reuse shared fixtures where possible
- [ ] **No cascading dependencies** — Each test can run independently
- [ ] **Async/await properly used** — Use `#[tokio::test]` for async code
- [ ] **Cleanup is automatic** — Use Drop or defer patterns, not manual cleanup
- [ ] **Deterministic results** — No timing-dependent assertions

**Target:** < 100ms per test

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
| Mock external calls | Call real APIs |
| Use deterministic data | Use random/time-based data |
| Assert one behavior | Assert multiple behaviors |
| Isolate test state | Share state between tests |
| Use fixtures | Repeat setup code |
| Profile slow tests | Ignore slowdowns |
| Keep tests small | Create mega-tests |
| Name tests clearly | Use cryptic names |

## Questions?

If a test seems slow or flaky:
1. Profile it with `time cargo test [name] -- --test-threads=1`
2. Check against anti-patterns above
3. Refer to examples in existing tests
4. Ask in code review if unsure

Remember: **Fast tests = fast feedback = better code.**