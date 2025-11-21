# Makefile Commands Reference

The Makefile provides convenient shortcuts for common development tasks. Use it to simplify your workflow without memorizing cargo commands.

## Quick Start

Run all quality checks with one command:

```bash
make check-all
```

This runs `cargo fmt`, `cargo clippy`, and `cargo test` sequentially.

## Available Commands

### Code Quality

#### `make fmt`
Format code using Rustfmt. Automatically fixes formatting issues.

```bash
make fmt
```

**Equivalent to:** `cargo fmt`

#### `make clippy`
Run Clippy linter with strict warnings. All warnings are treated as errors.

```bash
make clippy
```

**Equivalent to:** `cargo clippy -- -D warnings`

#### `make test`
Run all unit and integration tests.

```bash
make test
```

**Equivalent to:** `cargo test`

#### `make test-lib`
Run unit tests only (tests in `src/lib.rs`).

```bash
make test-lib
```

**Equivalent to:** `cargo test --lib`

#### `make test-int`
Run integration tests only (tests in `tests/`).

```bash
make test-int
```

**Equivalent to:** `cargo test --test '*'`

#### `make check-all`
Run all quality checks in sequence: `fmt` → `clippy` → `test`.

```bash
make check-all
```

**Use before:** Committing code or pushing to GitHub.

### Building

#### `make build`
Build debug WASM binary (faster, larger binary).

```bash
make build
```

**Equivalent to:** `cargo build --target wasm32-unknown-unknown`

#### `make release`
Build optimized release WASM binary (slower build, smaller binary).

```bash
make release
```

**Equivalent to:** `cargo build --release --target wasm32-unknown-unknown`

### Maintenance

#### `make clean`
Remove all build artifacts and temporary files.

```bash
make clean
```

**Equivalent to:** `cargo clean && rm -rf target/`

#### `make help`
Display all available commands with descriptions.

```bash
make help
```

## Common Workflows

### Before Committing

Ensure all checks pass:

```bash
make check-all
```

### During Development

Run specific checks as needed:

```bash
# Just format code
make fmt

# Just check for warnings
make clippy

# Just run tests
make test
```

### Before Pushing to GitHub

Run the full CI locally:

```bash
make check-all
```

### Building for Testing in Zed

Build and test:

```bash
make release
make test
```

### After Major Changes

Clean build with all checks:

```bash
make clean
make check-all
make release
```

## Requirements

The Makefile requires:
- **GNU Make** (usually pre-installed on macOS and Linux)
- **Rust toolchain** (via rustup)
- **Cargo** (included with Rust)

### Installing Make (if needed)

**macOS:**
```bash
# Make is included with Xcode Command Line Tools
xcode-select --install
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install build-essential
```

**Windows:**
Use WSL (Windows Subsystem for Linux) or install [GNU Make for Windows](http://gnuwin32.sourceforge.net/packages/make.htm).

## Tips

### Parallel Execution
Make can run some tasks in parallel. Use `-j` flag:

```bash
make -j check-all
```

### Dry Run
See what commands will execute without running them:

```bash
make -n check-all
```

### Silent Mode
Suppress command output (only show results):

```bash
make -s check-all
```

### Custom Flags
Pass additional flags to cargo by setting variables:

```bash
make test TEST_ARGS="-- --nocapture"
```

## Troubleshooting

### Command not found: make

Install GNU Make for your operating system (see Requirements above).

### make: *** No rule to make target 'check-all'

Ensure you're in the `zed-copilot` directory where `Makefile` is located.

### Tests hang or timeout

Run in single-threaded mode:

```bash
make test TEST_ARGS="-- --test-threads=1"
```

## Extending the Makefile

To add custom commands, edit `Makefile` and add:

```makefile
.PHONY: your-command

your-command:
	@echo "Running your command..."
	your actual command here
```

Use `.PHONY` to mark targets that don't create files.

## Reference

- [GNU Make Manual](https://www.gnu.org/software/make/manual/)
- [Cargo Commands](https://doc.rust-lang.org/cargo/)
- [Rustfmt Formatting Guide](https://rust-lang.github.io/rustfmt/)
- [Clippy Linter Guide](https://docs.rs/clippy/latest/clippy/)

---

**Last Updated:** 2025  
**Related:** `DEVELOPMENT.md`, `.github/CONTRIBUTING.md`
