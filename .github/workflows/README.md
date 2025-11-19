# GitHub Actions Workflows

This directory contains automated CI/CD workflows for Zed Copilot.

## Workflows

### `ci.yml` - Main CI Pipeline

**Trigger**: Push to `main`/`develop`, pull requests, manual dispatch

**Jobs**:

1. **test** - Run all tests
   - Unit tests: `cargo test --lib`
   - Integration tests: `cargo test --test '*'`
   - Output captured for debugging

2. **fmt** - Check code formatting
   - Command: `cargo fmt -- --check`
   - Fails if formatting issues found
   - Fix locally with: `cargo fmt`

3. **clippy** - Lint code
   - Command: `cargo clippy -- -D warnings`
   - Warnings treated as errors
   - Ensures code quality and best practices

4. **build** - Build WASM binary
   - Target: `wasm32-unknown-unknown`
   - Verifies binary exists
   - Checks size < 1.5MB
   - Uploads artifact for 7 days

5. **summary** - Report overall status
   - Aggregates all job results
   - Fails if any job failed
   - Reports to workflow summary

## Running Locally

### Using `act`

[act](https://github.com/nektos/act) simulates GitHub Actions locally.

```bash
# Install
brew install act

# Run all jobs
act

# Run specific job
act -j test
act -j build

# Run with verbose output
act -v
```

### Manual Commands

```bash
# Test
cargo test --lib
cargo test --test '*'

# Format
cargo fmt -- --check
cargo fmt  # Fix

# Lint
cargo clippy -- -D warnings

# Build WASM
cargo build --release --target wasm32-unknown-unknown
```

## Configuration

### Branches Monitored

- `main` - production branch
- `develop` - development branch

### Paths Triggering CI

- `src/**` - source code
- `tests/**` - test code
- `Cargo.toml`, `Cargo.lock` - dependencies
- `extension.toml` - extension metadata
- `.github/workflows/ci.yml` - workflow itself

**Skipped**:
- `**.md` - documentation
- `docs/**` - documentation
- `LICENSE` - license file

### Environment Variables

- `CARGO_TERM_COLOR=always` - Colored output
- `RUST_BACKTRACE=1` - Full stack traces on panic

## Artifacts

The `build` job uploads the compiled WASM binary:

- **Name**: `zed-copilot-wasm`
- **Path**: `target/wasm32-unknown-unknown/release/zed_copilot.wasm`
- **Retention**: 7 days

Download from the workflow run summary.

## Caching

All jobs use GitHub Actions cache to speed up builds:

- `~/.cargo/registry` - Crate registry
- `~/.cargo/git` - Git dependencies
- `target/` - Build artifacts

Cache keys include `Cargo.lock` hash to invalidate on dependency changes.

## Troubleshooting

### Tests Fail in CI but Pass Locally

```bash
# Run with verbose output
cargo test -- --nocapture

# Single-threaded (may reveal race conditions)
cargo test -- --test-threads=1

# Specific test
cargo test test_name -- --nocapture
```

### WASM Build Fails

```bash
# Ensure target installed
rustup target add wasm32-unknown-unknown

# Verbose build
cargo build --release --target wasm32-unknown-unknown --verbose
```

### Binary Size Exceeds 1.5MB

```bash
# Check actual size
ls -lh target/wasm32-unknown-unknown/release/zed_copilot.wasm

# Analyze dependencies
cargo tree

# Check for optimization opportunities
cargo build --release -Z timings
```

### Formatting Issues

```bash
# Check what needs fixing
cargo fmt -- --check

# Auto-fix
cargo fmt
```

### Clippy Warnings

```bash
# See specific warnings
cargo clippy -- -D warnings

# Fix automatically where possible
cargo clippy --fix
```

## Modifying Workflows

When updating workflows:

1. Test locally with `act`
2. Make changes to `.github/workflows/ci.yml`
3. Commit and push to trigger new workflow
4. Monitor workflow run in GitHub Actions tab
5. Update this README if behavior changes

## Performance

Typical CI run times:

- **Quality checks** (fmt + clippy): ~30-60s
- **Unit tests**: ~20-40s
- **Integration tests**: ~10-20s
- **WASM build**: ~2-3 minutes
- **Total**: ~5-6 minutes

Caching significantly improves subsequent runs.

## Security

- No secrets stored in workflow file
- API keys managed through GitHub Secrets (not yet used)
- WASM artifacts downloaded over HTTPS
- No external script execution

## Future Enhancements

- [ ] Cross-platform testing (Linux, macOS, Windows)
- [ ] Performance benchmarking
- [ ] Dependency security scanning
- [ ] Automated publishing to registry
- [ ] Code coverage reporting
- [ ] Docker build and test