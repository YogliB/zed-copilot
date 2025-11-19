# Zed Copilot

AI-powered code completion and assistance for Zed IDE.

## Overview

Zed Copilot is an extension for the Zed code editor that brings intelligent AI-powered features to your coding workflow. This extension provides a foundation for integrating various AI providers to enhance productivity through code completions, suggestions, and analysis.

## Features

- 🤖 AI-powered code assistance
- ⚡ Fast, responsive interactions
- 🔧 Extensible architecture for multiple AI providers
- 📝 Clean, maintainable codebase
- 🛠️ Built with Rust and WebAssembly

## Installation

### Prerequisites

- **Zed IDE** (latest version from [zed.dev](https://zed.dev))
- **Rust** (installed via rustup - **not** Homebrew)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  ```

### Install as Dev Extension

1. Clone this repository:
   ```bash
   git clone https://github.com/zed-industries/zed-copilot.git
   cd zed-copilot
   ```

2. Open Zed IDE

3. Go to **Extensions** panel (or use `zed: open extensions` command)

4. Click **Install Dev Extension** (or use `zed: install dev extension` action)

5. Select the `zed-copilot` directory

6. The extension will load immediately

### Verify Installation

- Check the Extensions page - you should see "Zed Copilot" listed
- Open Zed log with `zed: open log` to verify no errors on startup
- Look for `[Zed Copilot] Extension initialized` in the logs

## Development

### Project Structure

```
zed-copilot/
├── extension.toml          # Extension metadata and configuration
├── Cargo.toml              # Rust project configuration
├── src/
│   └── lib.rs              # Main extension implementation
├── README.md               # This file
├── DEVELOPMENT.md          # Development guide and architecture
├── LICENSE                 # MIT License
├── CHANGELOG.md            # Version history
└── .gitignore              # Git ignore rules
```

### Building

```bash
# Build the extension
cargo build --release

# The compiled WebAssembly will be in target/release/
```

### Debug Mode

Run Zed in foreground mode to see extension logs and debug output:

```bash
zed --foreground
```

This will display `println!` output and detailed logging from the extension.

### Running Tests

```bash
cargo test
```

## Troubleshooting

### Extension fails to load

**Error:** "Failed to install dev extension"

**Solutions:**
1. Verify Rust is installed via rustup:
   ```bash
   rustup --version
   ```
   
2. Make sure you don't have Rust installed via Homebrew. If you do:
   ```bash
   brew uninstall rust  # if installed via Homebrew
   rustup update stable
   ```

3. Check Zed logs: `zed: open log`

4. Rebuild the extension:
   ```bash
   cargo clean
   cargo build --release
   ```

### Build compilation errors

**Error:** "error: could not compile `zed-copilot`"

**Solutions:**
1. Ensure you're using Rust edition 2021:
   ```bash
   rustc --version
   ```

2. Update dependencies:
   ```bash
   cargo update
   ```

3. Check `zed_extension_api` version compatibility in `Cargo.toml`

### No log output

**Problem:** Can't see extension logs

**Solutions:**
1. Run Zed in foreground mode: `zed --foreground`
2. Make sure the extension is actually loaded by checking Extensions page
3. Check that extension builds without errors: `cargo build --release`

## Configuration

Currently, Zed Copilot has minimal configuration. Future versions will support:
- AI provider selection (OpenAI, Anthropic, etc.)
- API key management
- Feature toggles
- Custom prompts and behaviors

See `DEVELOPMENT.md` for planned features and architecture.

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Test locally as a dev extension
5. Commit with clear messages
6. Push and open a pull request

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) file for details.

## Documentation

- [DEVELOPMENT.md](DEVELOPMENT.md) - Architecture, planned features, and development guidelines
- [CHANGELOG.md](CHANGELOG.md) - Version history and release notes
- [Zed Extension Docs](https://zed.dev/docs/extensions) - Official Zed extension documentation

## Support

For issues, questions, or suggestions:
- Open an issue on GitHub
- Check existing issues for solutions
- Review troubleshooting section above

## Acknowledgments

- [Zed Industries](https://zed.dev) for the amazing editor and extension API
- All contributors and community members

---

**Status:** Early Development (v0.0.1)

This is the foundation for AI-powered features in Zed. More features coming soon!