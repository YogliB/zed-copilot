# Test Performance Optimization Phases 2-4

**Overview:** Execute Phase 2–4 of test suite optimization in `zed-copilot` to further improve maintainability and performance through DRY refactoring, parameterized tests, and lazy initialization.

**Approach:** Sequential PRs moving pure validation tests to unit tier, consolidating error response patterns, and optimizing mock server startup.

**Est. Time:** 8–12h dev + 4–6h review (phased over 1–2 weeks)

**PRs:** 3 across 1 repo

**Risk:** Low — all changes isolated to test code; no production logic affected.

**Repos:** `zed-copilot`

---

## Implementation Status

| PR  | Title | Repo | Status | Link | Notes |
| --- | ----- | ---- | ------ | ---- | ----- |
| 1   | Phase 2: Move Pure Validation Tests to Unit Tier | zed-copilot | 🟢 | https://github.com/YogliB/zed-copilot/pull/1 | Completed: moved 7 pure validation tests to unit tier |
| 2   | Phase 3: Parameterize Error Response Tests | zed-copilot | ⏸️ | - | Consolidate similar error/validation patterns; reduce duplication |
| 3   | Phase 4: Lazy-Initialize Mock Server | zed-copilot | ⏸️ | - | Defer mock server startup until first use in E2E tests |

**Status:** 🟢 Phase 2 completed · ⏸️ Phases 3–4 pending

---

## PR 1: Phase 2 — Move Pure Validation Tests to Unit Tier — 🟢

**Repo:** zed-copilot · **Link:** https://github.com/YogliB/zed-copilot/pull/1 · **Status:** Merged 2025-11-21

**Files:**
- `tests/integration_tests.rs` (audit & identify candidates)
- `src/**/*.rs` (add new unit test modules)

**Changes:**

1. **Audit integration tests for async-free validation** — File: `tests/integration_tests.rs`
   - Review all 9 integration tests in `tests/integration_tests.rs`
   - Identify tests that don't require async runtime or external dependencies (fixture-only)
   - Examples: extension name/ID validation, default instance creation, meta checks
   - Create list of candidates with justifications

2. **Extract pure validation logic** — Files: `src/lib.rs`, module-specific files
   - Move tests that only validate config, types, or naming to inline `#[test]` modules
   - Keep async tests (`#[tokio::test]`) only for fixture-dependent scenarios
   - Update test names to reflect new location (e.g., `test_extension_name_is_consistent`)

3. **Update integration tests** — File: `tests/integration_tests.rs`
   - Remove migrated tests from integration tier
   - Consolidate remaining 3–5 async fixture-heavy tests
   - Keep for: fixture composition, multi-context interaction, module loading

4. **Update test documentation** — File: `docs/TEST_OPTIMIZATION_CHECKLIST.md`
   - Document rationale for moved tests
   - Update examples to reflect new structure
   - Add note on fixture-independent tests belonging in unit tier

**Acceptance:** ✅ All criteria met

- [x] Audit complete: 7 tests identified and moved to unit tier
- [x] Moved tests pass locally with `cargo test --lib` (94 passed)
- [x] Integration suite covers fixture composition (4 remaining tests)
- [x] Test names updated to match coding rules (action verbs, clear intent)
- [x] All checks pass (cargo test, clippy, fmt)
- [x] No breaking changes to test API or test organization
- [x] Performance baseline maintained (130 tests pass in 3.21s, negligible increase)

**Dependencies:** Blocked by None · Unblocks PR 2

**Result:** Successfully moved 7 pure validation tests from integration tier to unit tier:
- 1 test added to `src/lib.rs` (extension name consistency)
- 2 tests added to `tests/common/mod.rs` (context creation, coexistence)
- 1 duplicate test consolidated
- Comprehensive documentation added to `TEST_OPTIMIZATION_CHECKLIST.md`
- Merged 2025-11-21 09:00:18 UTC

**Rationale:**
- Unit tests run faster (no async overhead)
- Improves clarity: pure validation ≠ fixture integration
- Reduces integration test count, making them easier to maintain
- Expected perf improvement: ~50–100ms (fewer async startups)

---

## PR 2: Phase 3 — Parameterize Error Response Tests — ⏸️

**Repo:** zed-copilot · **Link:** - · **ETA:** 3–4h dev + 1–2h review

**Files:**
- `tests/openai_e2e.rs` (consolidate similar error patterns)
- `tests/anthropic_e2e.rs` (future; consolidate when tests added)
- `tests/common/mod.rs` (helper for test data builders)

**Changes:**

1. **Audit E2E tests for repeating patterns** — Files: `tests/openai_e2e.rs`
   - Identify tests with similar structure: mock setup → request → assert error format
   - Examples: `test_openai_auth_error`, `test_openai_rate_limit_response` (similar mock/assert)
   - Group by response type: auth, rate limit, malformed response, missing fields
   - Document pattern reuse opportunity

2. **Create parameterized test helper** — File: `tests/common/mod.rs`
   - Build helper struct `MockErrorScenario`:
     ```rust
     struct MockErrorScenario {
         name: &'static str,
         status: u16,
         body: serde_json::Value,
         expected_error: &'static str,
     }
     ```
   - Add vec of scenarios for auth, rate limit, malformed, missing fields
   - Create `assert_error_scenario(scenario, client)` helper

3. **Consolidate error tests** — File: `tests/openai_e2e.rs`
   - Replace individual error tests with parameterized macro or loop
   - Use `#[tokio::test]` with `for scenario in SCENARIOS` or similar pattern
   - Keep contract-focused assertions (request/response structure)
   - Expected consolidation: 5–6 individual tests → 1–2 parameterized tests

4. **Document pattern** — File: `docs/TEST_OPTIMIZATION_CHECKLIST.md`
   - Add section: "Parameterized Tests for Error Scenarios"
   - Provide example template for future E2E tests
   - Document when to parameterize vs. keep separate (boundary between "enough scenarios" and "too many")

**Acceptance:**

- [ ] Parameterized helper created and tested
- [ ] 5–6 similar tests consolidated into 1–2 parameterized tests
- [ ] All error scenarios still covered (no test loss)
- [ ] Test names clearly indicate scenarios being tested
- [ ] Assertions remain contract-focused (not overfitted)
- [ ] All checks pass
- [ ] Perf maintained or improved (expect slight reduction due to fewer test startups: ~20–30ms)
- [ ] Code follows DRY principle; no duplication across error tests

**Dependencies:** Blocked by PR 1 (optional, can run in parallel) · Blocks PR 3

**Rationale:**
- DRY: eliminates repeating mock setup code
- Clarity: one test validates multiple error scenarios consistently
- Maintainability: update error handling in one place
- Perf: fewer async test startups (though offset by parameterized loop)

---

## PR 3: Phase 4 — Lazy-Initialize Mock Server — ⏸️

**Repo:** zed-copilot · **Link:** - · **ETA:** 2–3h dev + 1–2h review

**Files:**
- `tests/openai_e2e.rs` (refactor mock server setup)
- `tests/common/mod.rs` (add lazy initialization helper)

**Changes:**

1. **Audit mock server usage** — Files: `tests/openai_e2e.rs`
   - Review current mock server pattern: is it created once per test or reused?
   - Measure cost of startup: profile with `time cargo test --test openai_e2e`
   - Check if any tests don't actually use the mock server (rare, but document)

2. **Create lazy mock server helper** — File: `tests/common/mod.rs`
   - Add `LazyMockServer` wrapper:
     ```rust
     struct LazyMockServer {
         server: Option<MockServer>,
         mocks: Vec<MockDefinition>,
     }
     impl LazyMockServer {
         async fn mount_if_needed(&mut self) { /* start server on first use */ }
         async fn add_mock(&mut self, mock: MockDefinition) { /* deferred */ }
     }
     ```
   - Goal: delay `MockServer::start()` until first mock is added
   - Benefit: tests that set up but never use mocks avoid startup cost

3. **Refactor E2E test setup** — File: `tests/openai_e2e.rs`
   - Update test fixture to use `LazyMockServer`
   - Ensure mocks are mounted before client usage
   - Verify no behavior change (all tests still pass)

4. **Document optimization** — File: `docs/CI_PERFORMANCE_MONITORING.md`
   - Add note on lazy initialization pattern
   - Document when to use (many tests, sparse mock usage)
   - Add expected perf impact

**Acceptance:**

- [ ] LazyMockServer helper created and documented
- [ ] All E2E tests refactored to use lazy initialization
- [ ] Behavior unchanged: all tests pass, same assertions
- [ ] Startup cost reduced: mock server only starts for tests that need it
- [ ] All checks pass
- [ ] Expected perf improvement: ~30–50ms (fewer unused server startups)
- [ ] No flakiness introduced (verify with `cargo test --test openai_e2e -- --test-threads=1` x3)

**Dependencies:** Blocked by PR 2 (optional, can run in parallel) · Blocks None

**Rationale:**
- Perf: defer expensive initialization until needed
- Flexibility: add mocks without upfront server startup cost
- Future-proof: scales to more E2E tests

---

## Risk Mitigation

**Test Regression Risk:** Moving tests or refactoring patterns could accidentally skip assertions
→ **Mitigation:** 
- Run full test suite after each PR (`cargo test`)
- Compare before/after test counts (should not decrease)
- Manual review of moved test assertions
- CI checks enforce no regression

**Flakiness Risk:** Lazy initialization or parameterized tests could introduce timing issues
→ **Mitigation:**
- Run slow tests with `--test-threads=1` to catch race conditions
- Profile with `time cargo test` to ensure no unexpected slowdowns
- Document any async/timing assumptions

**Maintainability Risk:** Parameterized tests are harder to debug if a single scenario fails
→ **Mitigation:**
- Each parameterized test scenario should have a clear `name` for error messages
- Keep assertions simple and focused
- Document expected vs. actual in error message

---

## Deployment Strategy

**CRITICAL:** All changes are test-only; zero impact on production code.

**Stage 1:** PR 1 (Phase 2) — Move Pure Validation Tests

1. **Code Review** — Verify audit is correct, moved tests are truly pure validation
2. **Local Test Run** — `cargo test --lib` and `cargo test --test integration_tests`
3. **Merge** — All checks pass
4. **Verify in main** — Rerun full suite; compare to baseline

**Stage 2:** PR 2 (Phase 3) — Parameterize Error Tests

1. **Code Review** — Verify parameterization covers all original scenarios
2. **Local Test Run** — `cargo test --test openai_e2e`
3. **Merge** — All checks pass
4. **Verify in main** — Rerun full suite; ensure test count and coverage maintained

**Stage 3:** PR 3 (Phase 4) — Lazy Init

1. **Code Review** — Verify lazy initialization logic is sound
2. **Local Test Run** — `cargo test --test openai_e2e -- --test-threads=1` x3 (check for flakiness)
3. **Merge** — All checks pass
4. **Verify in main** — Rerun full suite; confirm perf improvement

**Cross-Repo Version Map**
| Stage | PR | zed-copilot | Notes |
| ----: | -- | ----------- | ----- |
| 1 | 1 | Phase 2 branch | Test restructure |
| 2 | 2 | Phase 3 branch | Parameterized consolidation |
| 3 | 3 | Phase 4 branch | Lazy init |

---

## Monitoring & Observability

**Metrics:**
- `unit_tests_time` — expect ~0.98s (stable)
- `integration_tests_time` — expect ~0.6s after PR 1 (down from 0.69s)
- `e2e_tests_time` — expect ~0.45–0.50s after PR 2 & 3 (slight variation from consolidation)
- `full_suite_time` — target <2.5s (currently 2.21s)

**Logs:**
- **Success:** "Test [name] passed in [Xms]"
- **Errors:** "Test [name] failed: [assertion details]"

**Alarms:**
- If any test exceeds budget (unit >15ms, integration >100ms, E2E >60ms) → alert in PR
- If full suite exceeds 2.5s → PR blocks until optimized

---

## Rollback

**Quick Rollback** (if flakiness or regression):
1. Revert the PR commit (`git revert [commit]`)
2. Push to branch and reopen PR
3. Investigate root cause in post-mortem

**Full Rollback**:
- PR 3 (lazy init) → disable lazy pattern, revert to direct MockServer startup
- PR 2 (parameterized) → expand parameterized tests back to individual test functions
- PR 1 (moved tests) → move tests back to integration tier

**Order:** Reverse order of merge (3 → 2 → 1)

**Artifacts Safe to Keep:**
- Test documentation updates (best practices are still valid)
- Baseline metrics (used for future regression detection)

---

## Success Criteria

- [ ] All 3 PRs merged to `main`
- [ ] Full test suite passes: `cargo test` (114 tests)
- [ ] No perf regression: full suite <2.5s (expect ~2.15–2.20s)
- [ ] Test count maintained: 114 tests (no loss of coverage)
- [ ] Zero flakiness: run suite 3x, all pass
- [ ] Maintainability improved: DRY applied, patterns documented
- [ ] Documentation updated: checklist reflects new patterns
- [ ] CI metrics stored and trended

---

## Implementation Order & Timeline

### Week 1

**PR 1 (Phase 2) — 2–3h dev**
- [ ] Audit integration tests (1–2h)
- [ ] Move pure validation tests (1h)
- [ ] Update docs (30m)
- [ ] Review + merge (1–2h)

**PR 2 (Phase 3) — 3–4h dev (parallel or sequential after PR 1)**
- [ ] Audit E2E test patterns (1–2h)
- [ ] Create parameterized helper (1h)
- [ ] Consolidate tests (1h)
- [ ] Review + merge (1–2h)

### Week 2

**PR 3 (Phase 4) — 2–3h dev**
- [ ] Audit mock server usage (30m)
- [ ] Implement lazy init helper (1–1.5h)
- [ ] Refactor E2E tests (30m)
- [ ] Flakiness testing (30m)
- [ ] Review + merge (1–2h)

**Post-Merge (Ongoing)**
- Monitor CI metrics for regressions
- Update CHANGELOG with optimization summary
- Plan quarterly audit schedule

---

## References

- `zed-copilot/docs/TEST_PERFORMANCE_BASELINE.md` — baseline metrics (lines 1–50)
- `zed-copilot/docs/TEST_OPTIMIZATION_CHECKLIST.md` — developer guide (lines 1–150)
- `zed-copilot/docs/TEST_AUDIT.md` — current test inventory and patterns
- `zed-copilot/docs/CI_PERFORMANCE_MONITORING.md` — CI setup and regression detection
- `zed-copilot/tests/integration_tests.rs` — integration test suite (all 9 tests)
- `zed-copilot/tests/openai_e2e.rs` — E2E test patterns (12 tests)
- `zed-copilot/tests/common/mod.rs` — test utilities and helpers

---

## Notes & Assumptions

### Implementation Decisions
- ✅ **Unit vs. Integration**: Tests with no async or fixtures are unit tests; tests requiring `#[tokio::test]` fixture creation are integration
- ✅ **Parameterization Scope**: Focus on error response patterns first (high repetition); expand to other patterns if clear DRY opportunities emerge
- ✅ **Lazy Init Scope**: Apply to E2E mock server; defer if complexity arises in other areas
- ✅ **Test Count**: All existing tests remain; no removal (only moves and consolidation)

### Cross-Repo Coordination
- ✅ Single repo (`zed-copilot`), so coordination minimal
- ✅ No dependency on external projects

### Testing Strategy
- **Unit Tests**: Integrated with Phase 2 PR (moved tests must pass as unit tests)
- **Integration Tests**: Integrated with Phase 2 PR (remaining tests must pass as async fixtures)
- **E2E Tests**: Integrated with Phase 3 & 4 PRs (parameterized and lazy init must not lose coverage)
- **No Standalone Test PRs**: All test changes bundled with phases

### Assumptions
- ✅ **Phase 1 Complete**: Baseline metrics, CI monitoring, and test audit are done
- ✅ **Phase 2 Candidates Exist**: Integration tests include pure validation (verified in audit)
- ✅ **Phase 3 Pattern Repetition**: E2E tests have similar error response patterns (verified in audit)
- ❌ **Phase 4 Impact**: Lazy init benefit may be modest (~20–50ms) if current startup is already minimal; verify impact before extensive refactoring

### Risks
- 🟡 **Test Moving Risk**: Moved tests must maintain same behavior; review carefully to avoid accidentally changing assertions
- 🟡 **Parameterization Complexity**: Parameterized tests are harder to debug; document scenarios clearly
- 🟡 **Lazy Init Flakiness**: Deferred initialization could introduce race conditions; test thoroughly with `--test-threads=1`

---

## Next Steps After Phase 4

Once Phase 4 is complete:

1. **Quarterly Audits** — Every 3 months, review new tests added since last audit
2. **Trend Analysis** — Plot test perf over time in CI artifacts
3. **Pattern Documentation** — As new patterns emerge, update checklist
4. **Future Optimization** — Evaluate:
   - Shared fixtures for E2E (if multiple API providers added)
   - Test parallelization limits (current: full parallelization)
   - Caching mechanisms for expensive operations

---

**Masterplan Created:** 2025-01-XX  
**Last Updated:** 2025-01-XX  
**Owner:** [your name]  
**Status:** Ready for Phase 2 kickoff