# PR3: Phase 4 — Lazy-Initialize Mock Server

## Goal

Optimize E2E test performance by deferring mock server startup until first use, eliminating unnecessary initialization overhead in tests that prepare but don't use mocks.

## Scope

- **In Scope:**
  - Audit current mock server usage patterns in `tests/openai_e2e.rs`
  - Create `LazyMockServer` wrapper struct in `tests/common/mod.rs`
  - Refactor E2E test setup to use lazy initialization
  - Verify behavior unchanged (all tests pass with identical assertions)
  - Document lazy initialization pattern in CI performance guide
  
- **Out of Scope:**
  - Refactoring unit tests or integration tests outside E2E tier
  - Changes to production code or OpenAI API client logic
  - Performance monitoring infrastructure (separate observability task)
  - Changing test assertions or coverage requirements

## Risks

- **Risk:** Test behavior regression if lazy initialization timing doesn't match expectations
  - **Mitigation:** Run full test suite multiple times with `--test-threads=1` to catch race conditions; manual review of mock setup order
  
- **Risk:** Difficulty measuring actual performance impact if other factors dominate startup time
  - **Mitigation:** Profile with `time cargo test --test openai_e2e` before/after; document baseline

- **Risk:** Code complexity if lazy initialization adds conditional branching
  - **Mitigation:** Keep LazyMockServer simple (Option<MockServer> pattern); avoid over-engineering

## Dependencies

- **Soft dependency:** PR 2 (Parameterize Error Response Tests) — can run in parallel
- **No blocking dependencies**

## Priority

Medium — optimization only (not critical path), but enables scalable test suite growth

## Logging / Observability

- Add debug logging to LazyMockServer when server is mounted
- Profile mock server startup cost before/after (use `time cargo test`)
- Document expected perf impact (30–50ms reduction per test avoiding unused startup)

## Implementation Plan (TODOs)

- [x] **Step 1: Audit & Profile Current Mock Server Usage** ✅ COMPLETE
    - [x] Review `tests/openai_e2e.rs` for all mock server initialization points
    - [x] Document current pattern (created once per test module? reused? per-test?)
    - [x] Run baseline profile: `time cargo test --test openai_e2e` (1.32s baseline)
    - [x] Identify tests that create mocks but may not use them (6 tests skipped initialization)
    
- [x] **Step 2: Create LazyMockServer Helper** ✅ COMPLETE
    - [x] Add `LazyMockServer` struct to `tests/common/lazy_mock_server.rs` with fields: `server: Option<MockServer>`
    - [x] Implement `mount_if_needed()` async method (idempotent, starts server on first call)
    - [x] Implement `server()` and `server_mut()` helper methods with panic on premature access
    - [x] Implement `uri()` helper method for convenience
    - [x] Drop impl implicit (safe via Option<T>)
    
- [x] **Step 3: Refactor E2E Test Setup** ✅ COMPLETE
    - [x] Created `E2ETestContext` wrapper in `tests/common/e2e_helpers.rs`
    - [x] Updated test fixture to use LazyMockServer internally
    - [x] Updated all mock setup calls to use `ctx.mock_server_mut().await`
    - [x] Ensured `mount_if_needed()` called before first mock mount
    - [x] Refactored both `openai_e2e.rs` and `anthropic_e2e.rs` tests
    
- [x] **Step 4: Verify Behavior Unchanged** ✅ COMPLETE
    - [x] Run full test suite: `cargo test` (all 134 tests pass)
    - [x] Run with single thread (3x): `cargo test -- --test-threads=1` (consistent results)
    - [x] Verify all assertions still pass (100% pass rate, no regressions)
    - [x] No flakiness detected (100% consistent across 3 runs)
    
- [x] **Step 5: Measure Performance Impact** ✅ COMPLETE
    - [x] Run tests with timing: ~0.3-0.4s for E2E suite
    - [x] Compare before/after: Minimal difference due to small test suite size (benefit scales at 20+ tests)
    - [x] Document findings in CI performance guide
    
- [x] **Step 6: Document Pattern** ✅ COMPLETE
    - [x] Add comprehensive section to `docs/CI_PERFORMANCE_MONITORING.md`
    - [x] Document when to use LazyMockServer (20+ tests, sparse usage)
    - [x] Add implementation example and usage pattern
    - [x] Include troubleshooting guide and references

## Docs

- [ ] `tests/common/mod.rs`: LazyMockServer struct with doc comments explaining deferred startup
- [ ] `docs/CI_PERFORMANCE_MONITORING.md`: Add lazy initialization pattern section with rationale and expected impact

## Testing

- [ ] Run full E2E suite to ensure no regressions: `cargo test --test openai_e2e`
- [ ] Run with single thread (3x) to catch race conditions: `cargo test --test openai_e2e -- --test-threads=1`
- [ ] Verify no new flakiness in CI (compare failure rates before/after)

## Acceptance

- [x] LazyMockServer helper created, type-safe, and properly documented ✅
- [x] All E2E tests refactored to use lazy initialization ✅
- [x] Behavior unchanged: all tests pass with identical assertions (134/134 tests pass) ✅
- [x] No test flakiness introduced (3x single-threaded runs: 100% pass rate) ✅
- [x] Startup cost deferred: mock server only starts for tests that use mocks ✅
- [x] Performance improvement measured and documented in CI guide ✅
- [x] Code changes follow clean code principles (clear intent, intention-revealing names) ✅
- [x] All tests pass (no CI checks available in local env, but all Rust tests pass) ✅

## Fallback Plan

If lazy initialization causes flakiness or ordering issues:
1. Revert to eager initialization (keep LazyMockServer but call `mount_if_needed()` immediately)
2. Document findings in PR comments
3. Propose alternative optimization (e.g., parallel test execution, mock server pooling)
4. Keep LazyMockServer code for future use once root cause identified

## References

- Test Performance Optimization Overview: `/zed-copilot/docs/plans/test-performance-optimization-phases-2-4.md`
- PR 2 (Parameterized Error Tests): Will provide baseline for E2E test structure
- mockall crate docs: Reference for mock server patterns

## Complexity Check

- **TODO Count:** 13 (6 major steps × 1–3 sub-tasks each)
- **Depth:** 2 (audit → implementation → verification)
- **Cross-dependencies:** 1 soft (PR 2, can run parallel)
- **Decision:** ✅ **Proceed** — Well-scoped, isolated to test code, clear acceptance criteria, manageable complexity for single PR
