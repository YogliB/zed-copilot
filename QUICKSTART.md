# Zed Copilot - Quick Start

Get up and running with Zed Copilot in 5 minutes.

## 1️⃣ Prerequisites Check

```bash
# Verify Rust is installed via rustup (NOT Homebrew)
rustup --version
cargo --version

# Verify Zed IDE is installed
zed --version
```

If not installed, see [SETUP.md](SETUP.md) for detailed instructions.

## 2️⃣ Clone & Navigate

```bash
git clone https://github.com/zed-industries/zed-copilot.git
cd zed-copilot
```

## 3️⃣ Build the Extension

```bash
cargo build --release
```

Expected output: `Finished release [optimized] target(s) in X.XXs`

## 4️⃣ Install in Zed

1. Open **Zed IDE**
2. Open **Extensions** panel (Command: `zed: open extensions`)
3. Click **Install Dev Extension**
4. Select the `zed-copilot` directory
5. Wait for installation to complete

## 5️⃣ Verify It Works

1. Check Extensions panel - "Zed Copilot" should be listed
2. Open Zed log: `zed: open log`
3. Look for: `[Zed Copilot] Extension initialized`

✅ **Done!** Your extension is running.

---

## Common Commands

```bash
# Build for release
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Check for warnings
cargo clippy

# View documentation
cargo doc --no-deps --open

# Run Zed with debug output
zed --foreground
```

## File Guide

| File | Purpose |
|------|---------|
| `extension.toml` | Extension metadata |
| `Cargo.toml` | Rust dependencies |
| `src/lib.rs` | Main code + unit tests |
| `tests/` | Integration tests |
| `README.md` | User guide |
| `DEVELOPMENT.md` | Architecture & roadmap |
| `SETUP.md` | Detailed setup guide |
| `LICENSE` | MIT License |

## Next Steps

- 📖 Read [DEVELOPMENT.md](DEVELOPMENT.md) for architecture
- 🚀 See [DEVELOPMENT.md#planned-features](DEVELOPMENT.md#planned-features-roadmap) for roadmap
- 🐛 Check [SETUP.md#troubleshooting](SETUP.md#troubleshooting) for common issues
- 💻 Start editing `src/lib.rs` to add features

## Troubleshooting

**Extension won't load?**
- Verify: `rustup --version` works
- Rebuild: `cargo clean && cargo build --release`
- Check logs: `zed: open log`
- See [SETUP.md#troubleshooting](SETUP.md#troubleshooting) for more

**Build fails?**
- Update Rust: `rustup update stable`
- Check error message in [SETUP.md#build-fails-on-first-try](SETUP.md#build-fails-on-first-try)

**Need help?**
- [SETUP.md](SETUP.md) - Detailed setup & troubleshooting
- [DEVELOPMENT.md](DEVELOPMENT.md) - Technical details & roadmap
- [README.md](README.md) - Features & documentation

---

**Status:** Ready to develop  
**Version:** 0.0.1  
**License:** MIT

For more details, see [README.md](README.md) or [SETUP.md](SETUP.md).