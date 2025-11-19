# Zed Copilot - Setup Guide

This guide walks you through setting up your development environment to build and test Zed Copilot locally.

## Prerequisites

Before you begin, ensure you have:
- macOS, Linux, or Windows with WSL2
- About 2GB of free disk space
- Git installed
- Terminal/command line familiarity

## Step 1: Install Rust via rustup

Rust must be installed via **rustup**, not Homebrew or other package managers. This ensures proper WebAssembly toolchain support.

### On macOS/Linux:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompts and select the default installation.

### On Windows (with WSL2):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Verify Installation

After installation, reload your shell or run:

```bash
source $HOME/.cargo/env
rustc --version
cargo --version
```

You should see version information for both `rustc` and `cargo`.

## Step 2: Install Zed IDE

Download and install Zed from [zed.dev](https://zed.dev/download).

Zed requires macOS 11+ or Linux (various distributions supported).

### Verify Zed Installation

Open Zed and check:
1. The application starts without errors
2. You can open the Command Palette (`Cmd+Shift+P` on Mac, `Ctrl+Shift+P` on Linux)
3. You can access the Extensions panel

## Step 3: Clone and Prepare Zed Copilot

```bash
# Clone the repository
git clone https://github.com/zed-industries/zed-copilot.git
cd zed-copilot

# Verify project structure
ls -la
# You should see: extension.toml, Cargo.toml, src/, README.md, LICENSE, etc.
```

## Step 4: Build the Extension

```bash
cd /path/to/zed-copilot
cargo build --release
```

This compiles the Rust code to WebAssembly. The first build may take a few minutes as it downloads dependencies.

### Expected Output

```
   Compiling zed-copilot v0.0.1
    Finished release [optimized] target(s) in 5.23s
```

### Troubleshooting Build Issues

**Error: "error: linker 'cc' not found"**
- macOS: Install Xcode Command Line Tools: `xcode-select --install`
- Linux (Ubuntu): `sudo apt-get install build-essential`
- Linux (Fedora): `sudo dnf install gcc`

**Error: "error\[E0514\]: type mismatch resolving... `zed_extension_api`"**
- Update Rust: `rustup update stable`
- Clear build cache: `cargo clean && cargo build --release`

**Error: "failed to fetch repository"**
- Check your internet connection
- Try: `cargo update`

## Step 5: Install as Dev Extension in Zed

1. **Open Zed IDE**

2. **Open the Extensions panel** using:
   - Command: `zed: open extensions` (Cmd+Shift+P / Ctrl+Shift+P)
   - Or: Click the Extensions icon in the sidebar

3. **Click "Install Dev Extension"** button (or use `zed: install dev extension` command)

4. **Select the directory** where you cloned `zed-copilot`

5. **Wait for installation** - Zed will compile and install the extension

6. **Verify Installation**:
   - Extensions page should show "Zed Copilot" with status "Installed"
   - Look for text indicating "dev extension"
   - If there's a published version, it shows "Overridden by dev extension"

## Step 6: Verify Everything Works

### Check Logs

Open the Zed log file using:
- Command: `zed: open log` (Cmd+Shift+P / Ctrl+Shift+P)

Look for a line like:
```
[Zed Copilot] Extension initialized
```

If you see this message, the extension loaded successfully!

### Debug Mode

Run Zed in foreground mode to see real-time debug output:

```bash
# From the terminal where Zed is installed
zed --foreground
```

This shows:
- Extension initialization logs
- Any errors or warnings
- `println!` output from the extension code

Close Zed with Ctrl+C when done.

## Step 7: Development Workflow

### Making Changes

1. Edit files in `src/lib.rs` or other source files
2. Run: `cargo build --release`
3. Reload extension in Zed (may need to restart Zed: `zed: restart electron`)
4. Check logs: `zed: open log`

### Running Tests

```bash
cargo test
```

Tests verify your code compiles and basic functionality works.

### Code Quality Checks

```bash
# Format code
cargo fmt

# Check for warnings and issues
cargo clippy

# Generate documentation
cargo doc --no-deps --open
```

## Troubleshooting

### Extension Won't Load

**Symptom:** Extensions page shows error or extension doesn't appear

**Solutions:**
1. Check that Rust is installed via rustup (not Homebrew):
   ```bash
   which rustup
   # Should show: /Users/yourname/.cargo/bin/rustup
   ```

2. Verify the build succeeded:
   ```bash
   cd /path/to/zed-copilot
   cargo build --release
   ```

3. Check Zed logs: `zed: open log`

4. Try reinstalling as dev extension

### Build Fails on First Try

**Symptom:** `cargo build --release` shows errors

**Solutions:**
1. Update Rust: `rustup update stable`
2. Update dependencies: `cargo update`
3. Clean and rebuild:
   ```bash
   cargo clean
   cargo build --release
   ```

### Zed Crashes When Extension Loads

**Symptom:** Zed closes when installing/loading the extension

**Solutions:**
1. Check Zed logs before crash: `zed: open log`
2. Try a simpler version: `cargo build --release`
3. Report issue with logs to GitHub issues

### Can't Find Cargo Command

**Symptom:** "cargo: command not found"

**Solutions:**
1. Check if rustup is installed: `rustup --version`
2. If not installed, run the installer again
3. Load cargo environment:
   ```bash
   source $HOME/.cargo/env
   cargo --version
   ```
4. Add to your shell profile (~/.bashrc, ~/.zshrc, etc.):
   ```bash
   . "$HOME/.cargo/env"
   ```

## Next Steps

Once setup is complete:

1. **Read the Documentation**:
   - [README.md](README.md) - User guide
   - [DEVELOPMENT.md](DEVELOPMENT.md) - Architecture and roadmap

2. **Start Contributing**:
   - Look at `DEVELOPMENT.md` for the roadmap
   - Check GitHub issues for things to work on
   - Follow the development workflow above

3. **Join the Community**:
   - Star the repository
   - Open issues for bugs or suggestions
   - Submit pull requests with improvements

## Getting Help

- **Rust Issues**: Check [rust-lang.org](https://www.rust-lang.org/tools/install)
- **Zed Issues**: See [zed.dev/docs](https://zed.dev/docs)
- **Project Issues**: Open a GitHub issue in this repository
- **Zed Community**: Ask in [Zed Discord](https://discord.gg/zed)

## Environment Details

To see your system information:

```bash
rustup --version
rustc --version
cargo --version
uname -a  # macOS/Linux
```

Include this output when reporting issues.

---

**Still stuck?** Check the troubleshooting sections in:
- [README.md](README.md#troubleshooting)
- [DEVELOPMENT.md](DEVELOPMENT.md#debugging)

Happy coding! 🚀