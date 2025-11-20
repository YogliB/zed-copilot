# Zed Copilot

> AI-powered chat assistant for Zed IDE

[![CI](https://github.com/YogliB/zed-copilot/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/zed-copilot/actions/workflows/ci.yml)
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

# Build extension
cargo build --release

# Open Zed → Extensions → Install Dev Extension
# Select the zed-copilot directory
```

### Verify

Open Zed logs (`zed: open log`) and look for:
```
[Zed Copilot] Extension initialized
```

✅ **You're ready!** See [QUICKSTART.md](docs/QUICKSTART.md) for detailed setup.

---

## What Works Now

| Feature | Status | Docs |
|---------|--------|------|
| Extension foundation | ✅ Complete | [DEVELOPMENT.md](docs/development/DEVELOPMENT.md) |
| OpenAI integration | ✅ Complete | [PROVIDER_INTEGRATION.md](docs/technical/PROVIDER_INTEGRATION.md) |
| Anthropic integration | ✅ Complete | [PROVIDER_INTEGRATION.md](docs/technical/PROVIDER_INTEGRATION.md) |
| Configuration system | ✅ Complete | [EXAMPLES.md](docs/getting-started/EXAMPLES.md) |
| HTTP & streaming | ✅ Complete | [HTTP_INTEGRATION.md](docs/technical/HTTP_INTEGRATION.md) |
| Chat interface | 📅 Q2 2025 | [ROADMAP.md](docs/development/ROADMAP.md) |
| Code completions | 📅 Q3 2025+ | [GH_COPILOT_LSP_INTEGRATION.md](docs/technical/GH_COPILOT_LSP_INTEGRATION.md) |

---

## 📚 Documentation

### 📂 Documentation Structure

```
docs/
├── getting-started/          # New users start here
│   ├── QUICKSTART.md        # 5-minute setup
│   ├── SETUP.md             # Detailed installation
│   └── EXAMPLES.md          # Configuration examples
│
├── reference/               # Configuration reference
│   └── CONFIG.md            # Schema and options
│
├── development/             # For contributors
│   ├── CONTRIBUTING.md      # How to contribute
│   ├── DEVELOPMENT.md       # Architecture guide
│   ├── TESTING.md           # Testing strategy
│   └── ROADMAP.md           # Feature timeline
│
├── technical/               # Deep technical details
│   ├── PROVIDER_INTEGRATION.md
│   ├── GH_COPILOT_LSP_INTEGRATION.md
│   ├── HTTP_INTEGRATION.md
│   ├── CHAT_ARCHITECTURE.md
│   └── RETRY_STRATEGY.md
│
└── CHANGELOG.md             # Version history
```

### 🚀 Getting Started

New to Zed Copilot? Start here:

- **[QUICKSTART.md](docs/getting-started/QUICKSTART.md)** — 5-minute setup guide
- **[SETUP.md](docs/getting-started/SETUP.md)** — Detailed installation and troubleshooting
- **[EXAMPLES.md](docs/getting-started/EXAMPLES.md)** — 13+ configuration examples

### ⚙️ Configuration

Learn how to configure Zed Copilot:

- **[EXAMPLES.md](docs/getting-started/EXAMPLES.md)** — Practical examples (cookbook)
- **[CONFIG.md](docs/reference/CONFIG.md)** — Complete schema reference (manual)

### 👨‍💻 Development

For contributors and developers:

- **[CONTRIBUTING.md](docs/development/CONTRIBUTING.md)** — How to contribute
- **[DEVELOPMENT.md](docs/development/DEVELOPMENT.md)** — Architecture and workflow
- **[TESTING.md](docs/development/TESTING.md)** — Testing strategy and guidelines
- **[ROADMAP.md](docs/development/ROADMAP.md)** — Feature timeline and phases

### 🔧 Technical Details

Deep dives into specific areas:

- **[PROVIDER_INTEGRATION.md](docs/technical/PROVIDER_INTEGRATION.md)** — AI provider implementation
- **[GH_COPILOT_LSP_INTEGRATION.md](docs/technical/GH_COPILOT_LSP_INTEGRATION.md)** — GitHub Copilot LSP integration
- **[HTTP_INTEGRATION.md](docs/technical/HTTP_INTEGRATION.md)** — HTTP client and streaming
- **[CHAT_ARCHITECTURE.md](docs/technical/CHAT_ARCHITECTURE.md)** — Chat system design
- **[RETRY_STRATEGY.md](docs/technical/RETRY_STRATEGY.md)** — Retry logic details

### 📋 Other

- **[CHANGELOG.md](docs/CHANGELOG.md)** — Version history

---

## 🎯 Documentation by Goal

### I want to install and use Zed Copilot
1. [QUICKSTART.md](docs/getting-started/QUICKSTART.md) — Quick setup
2. [SETUP.md](docs/getting-started/SETUP.md) — If you run into issues
3. [EXAMPLES.md](docs/getting-started/EXAMPLES.md) — Configuration

### I want to configure for my use case
- [EXAMPLES.md](docs/getting-started/EXAMPLES.md) — Ready-to-use configs
- [CONFIG.md](docs/reference/CONFIG.md) — Schema reference
- Model comparison in [EXAMPLES.md](docs/getting-started/EXAMPLES.md#comparison-when-to-use-what)

### I want to contribute to the project
1. [CONTRIBUTING.md](docs/development/CONTRIBUTING.md) — Process
2. [DEVELOPMENT.md](docs/development/DEVELOPMENT.md) — Architecture
3. [TESTING.md](docs/development/TESTING.md) — Testing guidelines
4. [ROADMAP.md](docs/development/ROADMAP.md) — What's planned

### I want to understand the architecture
- [DEVELOPMENT.md](docs/development/DEVELOPMENT.md) — Start here
- [PROVIDER_INTEGRATION.md](docs/technical/PROVIDER_INTEGRATION.md) — Provider details
- [ROADMAP.md](docs/development/ROADMAP.md) — Project phases

### I want to troubleshoot issues
- [SETUP.md](docs/getting-started/SETUP.md#troubleshooting) — Common issues
- [EXAMPLES.md](docs/getting-started/EXAMPLES.md#troubleshooting-examples) — Configuration errors
- [CONFIG.md](docs/reference/CONFIG.md#troubleshooting) — Validation errors

---

## Configuration

Zed Copilot is configured via `settings.json` in Zed. API keys use environment variables for security.

### Example: OpenAI with GPT-4o

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

### Example: Anthropic Claude

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "anthropic",
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-sonnet-4-20250514"
    }
  }
}
```

Set your API key:
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

📖 **See [EXAMPLES.md](docs/getting-started/EXAMPLES.md)** for 13+ configuration examples covering all models and use cases.

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

### Future
- **GitHub Copilot LSP** — Inline completions (Q3 2025+)
- **Local models** — Ollama support
- **Custom providers** — Extensible architecture

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

### Test Coverage

- **63 tests** — All passing ✅
- **Unit tests** — Provider logic, configuration
- **Integration tests** — End-to-end workflows

Run tests: `cargo test`

---

## Roadmap

**Current Phase:** Phase 2.3 — HTTP Integration ✅

**Timeline:**
- ✅ **Phase 1** — Foundation complete
- ✅ **Phase 2.1** — AI provider abstraction complete
- ✅ **Phase 2.2** — Configuration system complete
- ✅ **Phase 2.3** — HTTP integration complete
- 📅 **Phase 3** — Chat interface (Q2 2025) 🎯 **Primary feature**
- 📅 **Phase 4** — GitHub Copilot LSP & completions (Q3 2025+)

📖 **See [ROADMAP.md](docs/ROADMAP.md)** for detailed timeline and features.

---

## Contributing

Contributions welcome! Please:

1. Read [CONTRIBUTING.md](docs/development/CONTRIBUTING.md)
2. Follow coding standards in [zed-rules/AGENTS.md](https://github.com/zed-industries/zed-rules/blob/main/AGENTS.md)
3. Run `make check-all` before submitting
4. Write tests for new features

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

- 📖 **Documentation** — Start with [QUICKSTART.md](docs/getting-started/QUICKSTART.md)
- 🐛 **Issues** — [GitHub Issues](https://github.com/zed-industries/zed-copilot/issues)
- 💬 **Discussions** — [GitHub Discussions](https://github.com/zed-industries/zed-copilot/discussions)

---

**Built with ❤️ for the Zed community**

**Next Milestone:** Chat Interface (Phase 3, Q2 2025) 🚀
