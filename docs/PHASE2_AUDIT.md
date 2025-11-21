# Phase 2 Audit: Pure Validation Test Candidates

**Date:** 2025-01-XX  
**Scope:** Identify async-free, fixture-independent tests in `tests/integration_tests.rs` and `src/lib.rs`  
**Goal:** Move candidates to unit tier (`#[test]` modules in source)

---

## Summary

- **Total integration tests audited:** 7 (in `tests/integration_tests.rs`)
- **Total unit tests audited:** 5 (in `src/lib.rs` via `tests` module)
- **Candidates for migration:** 6
- **Tests to remain in integration tier:** 1

---

## Integration Tests (`tests/integration_tests.rs`)

### Analysis

All 7 tests use `#[test]` (sync, no async) and depend only on `TestContext`, which has no external I/O, fixtures, or async runtime.

| Test Name | Uses Async? | External Dependencies? | Pure Validation? | Candidate? | Justification |
|-----------|-----------|----------------------|------------------|-----------|---------------|
| `test_extension_compiles_and_loads` | ❌ | ❌ | ✅ | ✅ | Validates extension name constant; no fixture logic |
| `test_extension_can_be_created_via_default` | ❌ | ❌ | ✅ | ✅ | Tests default constructor; pure type validation |
| `test_extension_does_not_panic_on_creation` | ❌ | ❌ | ✅ | ✅ | Panic safety check; no external dependencies |
| `test_test_context_can_be_created` | ❌ | ❌ | ✅ | ✅ | Validates TestContext creation; pure validation |
| `test_multiple_contexts_can_coexist` | ❌ | ❌ | ✅ | ✅ | Tests multiple instances; no shared state or fixtures |
| `test_test_context_default_implementation` | ❌ | ❌ | ✅ | ✅ | Validates Default trait impl; pure type behavior |
| `test_extension_name_is_consistent` | ❌ | ❌ | ✅ | ✅ | Tests naming consistency across instances; pure validation |

**Decision:** All 7 integration tests are candidates for migration.

---

## Source Tests (`src/lib.rs`)

### Analysis

All 5 unit tests in the `tests` module are sync, no async, no external dependencies.

| Test Name | Uses Async? | External Dependencies? | Pure Validation? | Candidate? | Location |
|-----------|-----------|----------------------|------------------|-----------|----------|
| `test_zed_copilot_new` | ❌ | ❌ | ✅ | ✅ | `src/lib.rs` inline |
| `test_zed_copilot_default` | ❌ | ❌ | ✅ | ✅ | `src/lib.rs` inline |
| `test_extension_trait_new` | ❌ | ❌ | ✅ | ✅ | `src/lib.rs` inline |
| `test_multiple_instances` | ❌ | ❌ | ✅ | ✅ | `src/lib.rs` inline |
| `test_extension_initialization_does_not_panic` | ❌ | ❌ | ✅ | ✅ | `src/lib.rs` inline |

**Status:** Already in unit tier (inline `#[test]` modules). These serve as reference for moved tests.

---

## Migration Plan

### Candidates to Move to `src/lib.rs` Unit Tier

**Source:** `tests/integration_tests.rs`

1. **`test_extension_compiles_and_loads`**
   - **Target Module:** `src/lib.rs` (new test in existing `tests` module or new module)
   - **New Name:** `test_extension_name_initialization`
   - **Rationale:** Validates that extension name is correctly set on creation; no fixture dependency
   - **Complexity:** Low (simple assertion)

2. **`test_extension_can_be_created_via_default`**
   - **Target Module:** `src/lib.rs` or `tests/common/mod.rs`
   - **New Name:** `test_context_default_creates_valid_name`
   - **Rationale:** Pure type validation of Default trait; already tested in `src/lib.rs` (similar)
   - **Complexity:** Low (simple assertion)

3. **`test_extension_does_not_panic_on_creation`**
   - **Target Module:** `src/lib.rs` (expand panic tests)
   - **New Name:** Already matches coding rules
   - **Rationale:** Panic safety is a pure property; no fixture needed
   - **Complexity:** Low (panic::catch_unwind)

4. **`test_test_context_can_be_created`**
   - **Target Module:** `tests/common/mod.rs` (move to existing test module)
   - **New Name:** `test_context_creation_succeeds`
   - **Rationale:** TestContext creation is pure validation; no async or I/O
   - **Complexity:** Low (simple assertion)

5. **`test_multiple_contexts_can_coexist`**
   - **Target Module:** `tests/common/mod.rs` (move to existing test module)
   - **New Name:** Already descriptive
   - **Rationale:** Validates that multiple instances work independently; no fixture state
   - **Complexity:** Low (multiple assertions)

6. **`test_test_context_default_implementation`**
   - **Target Module:** `tests/common/mod.rs` (consolidate with existing `test_context_default`)
   - **New Name:** Merge with existing `test_context_default` (reduce duplication)
   - **Rationale:** Duplicate of existing test in `tests/common/mod.rs`; remove from integration
   - **Complexity:** Low (duplicate removal)

7. **`test_extension_name_is_consistent`**
   - **Target Module:** `src/lib.rs` (add to extension tests)
   - **New Name:** Already follows coding rules
   - **Rationale:** Pure validation of naming consistency across instances; no external deps
   - **Complexity:** Low (assertion)

---

## Integration Tests Remaining

**Count:** 1 (if we consolidate and move all identified candidates)

**Outcome:** `tests/integration_tests.rs` becomes empty or contains fixture-composition tests (if added in future phases).

**Action:** After migration, either:
- Remove `tests/integration_tests.rs` entirely (no integration tests currently needed)
- Keep file as placeholder for future fixture-based integration tests
- Consolidate remaining into a single "fixture composition" test if needed

---

## Consolidation Opportunities

1. **TestContext Tests:** 
   - Move 4 tests from `tests/integration_tests.rs` to `tests/common/mod.rs`
   - Consolidate `test_test_context_default_implementation` with existing `test_context_default`

2. **Extension Tests:**
   - Move 3 tests from `tests/integration_tests.rs` to `src/lib.rs`
   - Align naming with existing unit tests

---

## Risk Assessment

| Test | Risk Level | Mitigation |
|------|-----------|-----------|
| All 7 candidates | 🟢 Low | All are pure validation; no hidden async/fixture deps |
| TestContext consolidation | 🟡 Medium | Ensure merged test covers both Default + creation paths |
| File cleanup | 🟢 Low | Delete only after verifying no other code imports `tests/integration_tests.rs` |

---

## Verification Checklist

- [ ] No tests use `#[tokio::test]` or `#[async_std::test]`
- [ ] No tests import from modules that require async (e.g., `http`, `providers`)
- [ ] No tests use `TestContext::setup_fixtures()` or similar
- [ ] All moved tests pass with `cargo test --lib`
- [ ] All moved tests pass with `cargo test` (full suite)
- [ ] No test count loss (should maintain 12+ tests, just relocated)
- [ ] No breaking changes to `tests/common/mod.rs` or `src/lib.rs` public API

---

## Next Steps

1. **Proceed to Step 2:** Extract pure validation logic and create inline modules
2. **Consolidate:** Merge duplicate tests in `tests/common/mod.rs`
3. **Verify:** Run `cargo test` to ensure no regressions
4. **Clean up:** Remove migrated tests from `tests/integration_tests.rs`
