# Zed Copilot

[![CI](https://github.com/yourusername/zed-copilot/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/zed-copilot/actions/workflows/ci.yml)

An AI-powered chat assistant for Zed IDE. Ask questions about code, get explanations, and collaborate with AI—all within your editor.

## Overview

Zed Copilot brings an interactive AI chat interface directly into Zed IDE. **Ask your AI assistant anything about your code** — get explanations, debugging help, refactoring suggestions, and more. The extension supports multiple AI providers (OpenAI, Anthropic Claude) and is designed for a seamless, conversation-driven development workflow.

## Features

### Primary Feature: AI Chat
- 💬 **Interactive Chat Panel** — Ask questions about code in a conversation
- 🚀 **Real-time Streaming** — See AI responses as they're generated
- 📚 **Conversation History** — Maintain context across multiple exchanges
- 🔄 **Multi-turn Conversations** — Natural back-and-forth dialogue with AI
- 📍 **Code Context Awareness** — Reference current file, cursor position, or selected code
- 🤖 **Multiple AI Providers** — Choose between OpenAI (GPT-4, GPT-3.5) or Anthropic (Claude 3)
- 💾 **Session Persistence** — Chat history persists across editor sessions
- ⚡ **Fast & Responsive** — Sub-second response latency with streaming updates

### Must-Have Integration: GitHub Copilot LSP ⭐
- 🔧 **Inline Code Completions** — Real-time suggestions as you type (Phase 4)
- 🔐 **GitHub Authentication** — Seamless integration with GitHub Copilot subscription
- ⚡ **Native LSP Protocol** — Standard Language Server Protocol for reliability
- 🎯 **Smart Completions** — Context-aware suggestions across 50+ languages
- 🔄 **Copilot Chat Integration** — Route chat requests through Copilot API for consistency

See [GH_COPILOT_LSP_INTEGRATION.md](docs/GH_COPILOT_LSP_INTEGRATION.md) for implementation details.

### Optional Features (Future)
- 🔧 Advanced code refactoring assistance
- 📝 Documentation generation
- ✅ Test generation
- 🐛 Debugging helpers

## Quick Start

### Prerequisites

- **Zed IDE** (latest version from [zed.dev](https://zed.dev))
- **Rust** (installed via rustup)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source $HOME/.cargo/env
  ```
- **API Key** for at least one provider:
  - OpenAI: https://platform.openai.com/api-keys
  - Anthropic: https://console.anthropic.com/keys

### Install as Dev Extension

1. Clone this repository:
   ```bash
   git clone https://github.com/zed-industries/zed-copilot.git
   cd zed-copilot
   ```

2. Build the extension:
   ```bash
   cargo build --release
   ```

3. Open **Zed IDE**

4. Open **Extensions** panel (`zed: open extensions`)

5. Click **Install Dev Extension**

6. Select the `zed-copilot` directory

7. The extension will load immediately — look for the chat panel

### Configure Your AI Provider

1. Open Zed settings: `cmd+,` (macOS) or `ctrl+,` (Linux/Windows)

2. Add your API key (currently requires manual `settings.json` edit in Phase 2.2):
   ```json
   {
     "zed_copilot": {
       "enabled": true,
       "provider": "openai",
       "openai": {
         "api_key": "${OPENAI_API_KEY}",
         "model": "gpt-4"
       }
     }
   }
   ```

3. Set environment variable with your API key:
   ```bash
   export OPENAI_API_KEY="sk-..."
   ```

4. Restart Zed

5. Open the chat panel and start chatting! (Chat UI coming in Phase 3)

### Verify Installation

- Check the Extensions page — "Zed Copilot" should be listed ✅
- Open Zed log: `zed: open log`
- Look for: `[Zed Copilot] Extension initialized`

## Project Status

### Current Phase: 2.2 (Configuration & Credentials)

**Completed:**
- ✅ Extension foundation (Phase 1)
- ✅ AI provider abstraction (Phase 2.1)
- ✅ OpenAI & Anthropic provider implementations (Phase 2.1)
- ✅ 40+ tests, all passing

**In Progress:**
- 🔄 Configuration system (Phase 2.2)
- 🔄 Secure credential management (Phase 2.2)

**Coming Soon:**
- 📅 HTTP integration & streaming (Phase 2.3) — Q1 2025
- 📅 **Chat UI & Core Feature** (Phase 3) — Q2 2025
- 📅 Code completions (Phase 4) — Q3 2025+

See the [ROADMAP.md](docs/ROADMAP.md) for the complete project timeline and vision.

## Development

### Project Structure

```
zed-copilot/
├── extension.toml          # Extension metadata
├── Cargo.toml              # Rust dependencies
├── src/
│   ├── lib.rs              # Main extension & tests
│   └── providers/          # AI provider implementations
│       ├── mod.rs
│       ├── openai.rs       # OpenAI provider
│       ├── anthropic.rs    # Anthropic provider
│       ├── trait_def.rs    # AiProvider trait
│       ├── factory.rs      # Provider factory
│       └── error.rs        # Error types
├── tests/
│   ├── common/mod.rs       # Test utilities
│   └── integration_tests.rs
├── docs/
│   ├── ROADMAP.md          # Feature roadmap
│   ├── DEVELOPMENT.md      # Architecture & design
│   ├── TESTING.md          # Testing guide
│   ├── SETUP.md            # Setup instructions
│   ├── QUICKSTART.md       # Quick start guide
│   └── PROVIDER_INTEGRATION.md
├── README.md               # This file
├── LICENSE                 # MIT License
└── Makefile                # Development commands
```

### Build & Development

```bash
# Format code
make fmt

# Lint code
make clippy

# Run all tests
make test

# Build for development
cargo build

# Build for release
cargo build --release

# View all available commands
make help
```

Or use cargo directly:

```bash
cargo build --release
cargo test
cargo fmt
cargo clippy
```

### Debug Mode

Run Zed in foreground mode to see extension logs:

```bash
zed --foreground
```

Look for `[Zed Copilot]` prefixed log messages.

## Supported AI Providers

### OpenAI ✅

**Models:** GPT-4, GPT-3.5-turbo

**Setup:**
1. Get API key from https://platform.openai.com/api-keys
2. Set environment variable: `export OPENAI_API_KEY="sk-..."`
3. Configure in Zed settings (see Configuration section above)

**Status:** Ready for use (Phase 2.1 complete)

### Anthropic Claude ✅

**Models:** Claude 3 Opus, Sonnet, Haiku

**Setup:**
1. Get API key from https://console.anthropic.com/keys
2. Set environment variable: `export ANTHROPIC_API_KEY="sk-ant-..."`
3. Configure in Zed settings with `"provider": "anthropic"`

**Status:** Ready for use (Phase 2.1 complete)

### GitHub Copilot LSP ⭐ Must-Have Integration

**Models:** Claude 3 (via GitHub Copilot API)

**Features:**
- 🔧 Inline code completions via Language Server Protocol
- 🔐 Seamless GitHub authentication and subscription integration
- ⚡ Native LSP protocol for reliability and performance
- 🎯 Smart, context-aware suggestions across 50+ languages
- 🔄 Copilot Chat integration for consistent AI responses

**Setup:**
1. Authenticate with GitHub: `gh auth login`
2. Enable in Zed settings with `"provider": "github_copilot"`
3. Configure LSP server settings (auto-detected by default)

**Status:** Planned (Phase 4, Q3 2025+) — **Critical must-have integration for inline completions**

**Documentation:** See [GH_COPILOT_LSP_INTEGRATION.md](docs/GH_COPILOT_LSP_INTEGRATION.md) for comprehensive implementation strategy and roadmap.

### Other Providers (Future)

Planned support for:
- Local models via Ollama
- Self-hosted LLM services
- Additional commercial providers

## Configuration

### Settings File Format

Configuration is done via Zed's `settings.json`:

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4",
      "api_base": "https://api.openai.com/v1"
    },
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-3-sonnet",
      "api_base": "https://api.anthropic.com/v1"
    },
    "chat": {
      "streaming_enabled": true,
      "max_history_messages": 50,
      "auto_scroll_to_latest": true
    }
  }
}
```

**Key Points:**
- API keys use environment variable interpolation (`${VARIABLE_NAME}`)
- No credentials are hardcoded
- Set environment variables before starting Zed
- Optional: customize API base URL (for proxies, self-hosted, etc.)

### Environment Variables

```bash
# OpenAI
export OPENAI_API_KEY="sk-..."

# Anthropic
export ANTHROPIC_API_KEY="sk-ant-..."
```

Load these automatically by adding to your shell profile (`~/.zshrc`, `~/.bashrc`, etc.):

```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
```

## Troubleshooting

### Extension Won't Load

1. Verify Rust is installed via rustup:
   ```bash
   rustup --version
   ```

2. Rebuild the extension:
   ```bash
   cargo clean
   cargo build --release
   ```

3. Check Zed logs:
   ```
   zed: open log
   ```

4. Make sure you installed via **Install Dev Extension**, not from the registry

### Build Fails

1. Update Rust:
   ```bash
   rustup update stable
   ```

2. Update dependencies:
   ```bash
   cargo update
   ```

3. Check the error message and refer to [SETUP.md](docs/SETUP.md)

### Chat Panel Doesn't Appear

- Chat UI is coming in Phase 3 (Q2 2025)
- For now, the extension loads but has no visible UI
- Configuration UI is planned for Phase 2.2

### API Errors

1. Verify your API key is correct and set as environment variable
2. Check that API key is included in Zed settings
3. Verify the model name is correct for your provider
4. Check your API account has available quota
5. Review Zed logs for detailed error messages

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Follow the coding rules in [zed-rules/AGENTS.md](https://github.com/zed-industries/zed-rules/blob/main/AGENTS.md)
4. Run tests and linting:
   ```bash
   make check-all
   ```
5. Commit with clear messages
6. Push and open a pull request

## Documentation

- [ROADMAP.md](docs/ROADMAP.md) — Feature roadmap and timeline
- [DEVELOPMENT.md](docs/DEVELOPMENT.md) — Architecture and design decisions
- [SETUP.md](docs/SETUP.md) — Detailed setup and troubleshooting
- [QUICKSTART.md](docs/QUICKSTART.md) — 5-minute quick start
- [TESTING.md](docs/TESTING.md) — Testing strategy and guidelines
- [PROVIDER_INTEGRATION.md](docs/PROVIDER_INTEGRATION.md) — AI provider details
- [DOCUMENTATION_REVIEW.md](docs/DOCUMENTATION_REVIEW.md) — Documentation alignment review

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Support

Need help?

- 📖 Read [SETUP.md](docs/SETUP.md) for setup issues
- 🚀 Check [QUICKSTART.md](docs/QUICKSTART.md) to get started
- 🏗️ Review [DEVELOPMENT.md](docs/DEVELOPMENT.md) for architecture
- 🐛 Open a GitHub issue for bugs
- 💬 Discussions available for questions

## Acknowledgments

- [Zed Industries](https://zed.dev) for the amazing editor and extension API
- [OpenAI](https://openai.com) and [Anthropic](https://www.anthropic.com) for AI APIs
- All contributors and community members

---

**Current Status:** Early Development (Phase 2.2)  
**Primary Feature:** AI Chat (coming Phase 3)  
**Latest Release:** v0.0.1 — Foundation complete  
**Next Milestone:** Configuration & Credentials (Phase 2.2)  

🚀 **Chat interface coming in Q2 2025!**