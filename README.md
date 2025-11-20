# Zed Copilot

> AI-powered chat assistant for Zed IDE

[![CI](https://github.com/YogliB/zed-copilot/actions/workflows/ci.yml/badge.svg)](https://github.com/YogliB/zed-copilot/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Zed Copilot** brings conversational AI directly into your editor. Ask questions about code, get explanations, and collaborate with AI—all without leaving Zed.

---

## What Can It Do?

- 💬 **Interactive Chat** — Converse with AI about your code
- 🤖 **Multiple AI Providers** — Choose OpenAI or Anthropic Claude
- 📚 **Conversation History** — Maintain context across exchanges
- 🚀 **Real-time Streaming** — See responses as they're generated
- 📍 **Code Context** — Reference files, selections, and cursor position

> **Status:** Early development. Chat interface coming Q2 2025. Foundation and provider integration complete.

---

## Quick Start

**Takes 5 minutes** — Get the extension running in Zed.

### Prerequisites

- [Zed IDE](https://zed.dev) (latest version)
- [Rust](https://rustup.rs) via rustup
- API key from [OpenAI](https://platform.openai.com/api-keys) or [Anthropic](https://console.anthropic.com/keys)

### Install

```bash
# Clone repository
git clone https://github.com/zed-industries/zed-copilot.git
cd zed-copilot

# Install git hooks (optional but recommended)
bash scripts/setup-hooks.sh

# Build extension
cargo build --release

# Open Zed → Extensions → Install Dev Extension
# Select the zed-copilot directory
```

### Configure

Add to your Zed `settings.json`:

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o"
    }
  }
}
```

Set your API key:
```bash
export OPENAI_API_KEY="sk-..."
```

### Verify

Open Zed logs (`zed: open log`) and look for:
```
[Zed Copilot] Extension initialized
```

✅ **You're ready!** See **[docs/](docs/)** for detailed setup and configuration.

---

## Development Setup

After cloning, install git hooks to ensure code quality:

```bash
bash scripts/setup-hooks.sh
```

This enables **pre-commit** and **pre-push** hooks that:
- ✅ Format and lint checks on commit (fast, ~5-10 sec)
- ✅ Smart test validation on push (~20-45 sec, tests changed modules only)

**Learn more:** [Git Hooks Documentation](docs/development/GIT_HOOKS.md)

---

## What Works Now

| Feature | Status |
|---------|--------|
| Extension foundation | ✅ Complete |
| OpenAI integration | ✅ Complete |
| Anthropic integration | ✅ Complete |
| Configuration system | ✅ Complete |
| HTTP & streaming | ✅ Complete |
| Chat interface | 📅 Q2 2025 |
| Code completions | 📅 Q3 2025+ |

---

## 📚 Documentation

**Complete documentation:** [docs/](docs/)

### Quick Links

**Getting Started:**
- [5-Minute Setup](docs/getting-started/QUICKSTART.md) — Quick installation
- [Detailed Setup](docs/getting-started/SETUP.md) — Troubleshooting included
- [Configuration Examples](docs/getting-started/EXAMPLES.md) — 13+ ready-to-use configs

**Configuration:**
- [Examples Cookbook](docs/getting-started/EXAMPLES.md) — Copy-paste configurations
- [Schema Reference](docs/reference/CONFIG.md) — Complete options guide

**Development:**
- [Contributing Guide](docs/development/CONTRIBUTING.md) — How to contribute
- [Architecture](docs/development/DEVELOPMENT.md) — Understanding the codebase
- [Testing Strategy](docs/development/TESTING.md) — Testing guidelines
- [Roadmap](docs/development/ROADMAP.md) — Feature timeline

**Technical:**
- [Provider Integration](docs/technical/PROVIDER_INTEGRATION.md) — AI provider implementation
- [HTTP Integration](docs/technical/HTTP_INTEGRATION.md) — Network layer
- [All Technical Docs](docs/technical/) — Deep dives

**Other:**
- [Changelog](docs/CHANGELOG.md) — Version history

---

## Supported AI Models

### OpenAI
- **GPT-4o** — Best for general coding (recommended)
- **o1** — Advanced reasoning for complex problems
- **o3-mini** — Lightweight reasoning model

### Anthropic Claude
- **Claude Opus 4.1** — Most powerful, best for complex analysis
- **Claude Sonnet 4** — Balanced speed and quality (recommended)
- **Claude Haiku 4.5** — Fastest, most affordable

📖 **See [Configuration Examples](docs/getting-started/EXAMPLES.md)** for model comparison and configs.

---

## Roadmap

**Current Phase:** Phase 2.3 — HTTP Integration ✅

**Timeline:**
- ✅ **Phase 1** — Foundation complete
- ✅ **Phase 2** — Provider integration complete
- 📅 **Phase 3** — Chat interface (Q2 2025) 🎯 **Next milestone**
- 📅 **Phase 4** — GitHub Copilot LSP & completions (Q3 2025+)

📖 **See [ROADMAP.md](docs/development/ROADMAP.md)** for detailed timeline.

---

## Contributing

Contributions welcome! Please:

1. Read [CONTRIBUTING.md](docs/development/CONTRIBUTING.md)
2. Follow coding standards in [zed-rules/AGENTS.md](https://github.com/zed-industries/zed-rules/blob/main/AGENTS.md)
3. Run `make check-all` before submitting
4. Write tests for new features

---

## Development

### Build Commands

```bash
make fmt          # Format code
make clippy       # Check warnings
make test         # Run tests
make check-all    # Run all checks
```

### Project Structure

```
zed-copilot/
├── src/
│   ├── lib.rs              # Main extension
│   ├── providers/          # AI provider implementations
│   └── config/             # Configuration system
├── tests/                  # Integration tests
├── docs/                   # Documentation
└── Cargo.toml              # Dependencies
```

📖 **See [DEVELOPMENT.md](docs/development/DEVELOPMENT.md)** for architecture details.

---

## Troubleshooting

### Extension won't load?
- Verify Rust is installed: `rustup --version`
- Rebuild: `cargo clean && cargo build --release`
- Check logs: `zed: open log`

### API errors?
- Verify API key is set: `echo $OPENAI_API_KEY`
- Check settings.json syntax
- Ensure model name is correct

📖 **See [SETUP.md](docs/getting-started/SETUP.md#troubleshooting)** for comprehensive troubleshooting.

---

## License

[MIT License](LICENSE) — Free to use, modify, and distribute.

---

## Support

- 📖 **Documentation** — [docs/](docs/)
- 🐛 **Issues** — [GitHub Issues](https://github.com/zed-industries/zed-copilot/issues)
- 💬 **Discussions** — [GitHub Discussions](https://github.com/zed-industries/zed-copilot/discussions)

---

**Built with ❤️ for the Zed community**

**Next Milestone:** Chat Interface (Phase 3, Q2 2025) 🚀
