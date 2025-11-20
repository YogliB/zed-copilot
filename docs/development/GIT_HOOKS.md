# Git Hooks & Pre-Push Validation

> Prevent test failures from reaching CI with smart local validation

**Part of:** [Development Documentation](../README.md#-development--contributing)

---

## TL;DR

**Install once:**
```bash
bash scripts/setup-hooks.sh
```

**Then commit/push normally:**
- Pre-commit validates format & linting (~5-10 sec)
- Pre-push runs smart tests for changed modules (~20-45 sec)

**That's it.** Hooks prevent bugs like the backoff test failure from reaching CI.

---

## Overview

Git hooks enforce code quality before commits and run tests before pushes. The system catches issues early while maintaining developer velocity through intelligent test filtering.

**How it works:**

```
git commit → Pre-commit (fmt + clippy, ~5-10s)
git push   → Pre-push (smart tests, ~20-45s)
             ├─ Detect changed modules
             ├─ Run targeted tests
             └─ Validate WASM build
```

## Quick Start

### Installation

After cloning the repository, install git hooks:

```bash
bash scripts/setup-hooks.sh
```

This copies the hook scripts to `.git/hooks/` and makes them executable.

### Usage

Once installed, hooks run automatically:

```bash
# Pre-commit hook runs automatically
git commit -m "Your message"

# Pre-push hook runs automatically
git push
```

Or run tests manually anytime:

```bash
# Auto-detect changed modules and test
scripts/smart-test.sh

# Run full test suite
scripts/smart-test.sh full

# Preview what would run (dry run)
scripts/smart-test.sh dry-run
```

## Hook Details

### Pre-Commit Hook

**Trigger:** `git commit`

**What it does:**
- Checks code formatting with `cargo fmt --check`
- Runs clippy linter with `cargo clippy -- -D warnings`
- Fails if formatting or lint issues are found

**Duration:** ~5-10 seconds

**Exit behavior:**
- ✅ Pass: Commit proceeds
- ❌ Fail: Commit blocked; fix errors and retry

**Example output:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Pre-Commit Hook: Format & Lint Checks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🎨 Checking code formatting...
✓ Formatting OK

📎 Running clippy linter...
✓ Linting OK

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ Pre-commit checks passed!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Next: Push to trigger full test validation
```

### Pre-Push Hook

**Trigger:** `git push`

**What it does:**

Uses **intelligent test filtering** (--changed-only mode) to:
1. Detect which modules you've modified
2. Run tests only for those modules
3. Fall back to full suite if dependencies or config changes
4. Validate WASM compilation

**Smart fallback conditions:**
- Cargo.toml or Cargo.lock changed → full suite
- tests/ directory modified → full suite
- Build infrastructure changed (Makefile, .github/, scripts/) → full suite
- Otherwise → targeted tests only

**Duration:** ~20-45 seconds (varies by changed modules)

**Exit behavior:**
- ✅ Pass: Push proceeds
- ❌ Fail: Push blocked; fix failures and retry

**Example output (changed-only mode):**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Pre-Push Hook: Smart Test Validation
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Mode: Smart testing (changed modules only)
Tip: Use --force-full to run complete test suite

Running validation...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Detecting Changed Modules

📋 Changed files:
    src/http/retry.rs

✓ Changes safe for targeted testing

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running Native Rust Tests

📋 Targeted tests (changed modules)...
running 10 tests
test http::retry::tests::test_backoff_respects_max_delay ... ok
...
✅ Native tests passed

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Running WASM Validation

📋 Building WASM (release)...
   Compiling zed-copilot v0.0.1
✅ WASM build passed

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All Tests Passed! ✨

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✅ All validations passed!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Proceeding with push...
```

## Smart Test Script

### Overview

The `scripts/smart-test.sh` script intelligently detects which modules you've changed and runs only the relevant tests. This balances **speed** (fast iteration) with **safety** (catching bugs before CI).

### Modes

#### changed-only (default)

```bash
scripts/smart-test.sh
# or explicitly
scripts/smart-test.sh changed-only
```

**Behavior:**
- Analyzes git diff to detect changed files
- Maps changes to test modules (src/http/retry, src/providers, etc.)
- Runs tests for changed modules only
- Falls back to full suite if ambiguous

**When to use:** Daily development, most commits

#### full

```bash
scripts/smart-test.sh full
```

**Behavior:**
- Runs complete native test suite
- Runs WASM build validation
- No filtering

**When to use:**
- Merging branches with multiple changes
- Before major pushes
- When full coverage is needed
- When `changed-only` mode triggers fallback

#### dry-run

```bash
scripts/smart-test.sh dry-run
```

**Behavior:**
- Shows what would run without executing
- Useful for understanding what tests will be invoked
- No actual compilation or testing

**When to use:** Understanding hook behavior, debugging test selection

#### help

```bash
scripts/smart-test.sh help
scripts/smart-test.sh --help
scripts/smart-test.sh -h
```

Shows usage documentation.

### Example Scenarios

**Scenario 1: Fix a retry backoff bug**

```bash
# Edit src/http/retry.rs
git add src/http/retry.rs
git commit -m "Fix: Cap delay after jitter application"

# Pre-commit runs: fmt + clippy (~5 sec)
# Passes ✓

git push

# Pre-push runs: Smart detection → found src/http/retry change
# Runs: tests for http::retry module + WASM build
# Time: ~25 seconds
# Result: ✓ All tests pass, push succeeds
```

**Scenario 2: Upgrade dependencies**

```bash
# Update dependencies in Cargo.toml
git add Cargo.toml Cargo.lock
git commit -m "chore: upgrade serde to 1.1"

# Pre-commit runs: fmt + clippy (~5 sec)
# Passes ✓

git push

# Pre-push detects: Cargo.toml changed!
# Triggers: FALLBACK to full suite
# Runs: All native tests + WASM build
# Time: ~45 seconds
# Result: ✓ All tests pass, push succeeds
```

**Scenario 3: Preview tests without running**

```bash
# Made changes to providers module
scripts/smart-test.sh dry-run

# Output:
# Would run: Targeted native tests (changed modules)
# Would run: WASM build validation
# Dry-run complete (no tests executed)

# Then run them for real
scripts/smart-test.sh
```

## Bypass Procedures

### Pre-Commit Bypass

Skip formatting/lint check (not recommended):

```bash
git commit --no-verify
```

⚠️ **Warning:** This skips important quality checks. Only use if absolutely necessary.

### Pre-Push Bypass

Skip test validation (strongly discouraged):

```bash
git push --no-verify
```

⚠️ **Warning:** Bypassing tests may allow failures to reach CI. Use only in emergencies.

### Force Full Test Suite

Run complete test suite before push:

```bash
scripts/smart-test.sh full
# Then push normally
git push
```

This runs full tests but doesn't bypass the hook—the hook still runs on push, but since tests already passed, it's typically fast.

## Troubleshooting

### Hook not executing

**Problem:** Hooks aren't running on commit/push

**Solution:**
1. Verify installation: `ls -la .git/hooks/`
2. Reinstall: `bash scripts/setup-hooks.sh`
3. Check permissions: `chmod +x .git/hooks/pre-commit .git/hooks/pre-push`

### "smart-test.sh not found"

**Problem:** Pre-push hook can't find the smart test script

**Solution:**
1. Ensure you're in the project root: `cd /path/to/zed-copilot`
2. Verify script exists: `ls scripts/smart-test.sh`
3. Make executable: `chmod +x scripts/smart-test.sh`
4. Reinstall hooks: `bash scripts/setup-hooks.sh`

### Cargo commands fail in hooks

**Problem:** `cargo fmt`, `cargo test`, etc. fail only in hooks

**Solution:**
1. Verify Rust toolchain: `rustup show`
2. Check WASM target: `rustup target list | grep wasm32`
3. Run command manually: `cargo test` (outside hook)
4. If manual fails, fix the error first, then retry commit/push

### WASM target not installed

**Problem:** "WASM target not installed" warning

**Solution:**
```bash
rustup target add wasm32-unknown-unknown
```

Then retry push.

### Hooks slow down my workflow

**Problem:** Pre-push takes too long (>1 min)

**Solutions:**
1. Use `--changed-only` (default) to test only modified modules
2. Commit frequently to keep changes small
3. Check if full suite is triggering: `scripts/smart-test.sh dry-run`
4. If necessary, bypass with `git push --no-verify` (last resort)

### Tests fail in hook but pass locally

**Problem:** Hook tests fail, but manual `cargo test` passes

**Possible causes:**
1. Different working directory context
2. Staged vs unstaged file differences
3. Hook running different test configuration

**Debug:**
1. Check hook output carefully for exact error
2. Run hook manually: `bash .git/hooks/pre-push`
3. Run manual tests: `scripts/smart-test.sh full`
4. Compare output; look for environmental differences

## Development

### Hook Files

Located in `scripts/`:
- `pre-commit` — Format and lint checks
- `pre-push` — Smart test validation
- `smart-test.sh` — Core test detection and execution logic
- `setup-hooks.sh` — Installation script

### Modifying Hooks

To change hook behavior:

1. Edit the hook script in `scripts/`
2. Reinstall: `bash scripts/setup-hooks.sh`
3. Test on next commit/push

**Note:** Changes to hook templates only affect new installations. Existing hooks are copies, so you must reinstall.

### Extending Smart Test Detection

To add new module patterns to `smart-test.sh`:

1. Open `scripts/smart-test.sh`
2. Find the `detect_changed_modules()` function
3. Add grep pattern for your module:
   ```bash
   if echo "$changed_files" | grep -qE '^src/my_module/'; then
       log_info "my_module detected"
   fi
   ```
4. Reinstall hooks: `bash scripts/setup-hooks.sh`

## FAQ

**Q: Can I disable hooks?**

A: Yes, use `--no-verify`, but this is discouraged. Hooks exist to prevent CI failures.

**Q: Do hooks run in CI?**

A: No, CI has its own checks. Hooks are for local development.

**Q: Can I customize which tests run?**

A: Edit `scripts/smart-test.sh` to adjust detection logic or run modes.

**Q: What if I'm on a slow network?**

A: Hooks run locally; network speed shouldn't affect them. If `cargo` is slow, it's a toolchain issue.

**Q: Do hooks work on all platforms?**

A: Yes, bash is available on macOS, Linux, and Windows (via Git Bash or WSL).

**Q: Can I use different hook names?**

A: Git only recognizes standard names (pre-commit, pre-push, etc.). Custom names won't work.

## References

- [Git Hooks Documentation](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Rust Code Formatting](https://rust-lang.github.io/rfcs/2436-style-guide.html)
- [Clippy Linter](https://github.com/rust-lang/rust-clippy)

## Related Documentation

- [CONTRIBUTING.md](./CONTRIBUTING.md) — How to contribute
- [DEVELOPMENT.md](./DEVELOPMENT.md) — Architecture guide
- [TESTING.md](./TESTING.md) — Testing strategy

---

**Back to:** [Development Documentation](../README.md#-development--contributing)