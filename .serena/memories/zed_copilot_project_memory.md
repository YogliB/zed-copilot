# Zed Copilot - Project Memory

**Last Updated:** 2025-01-09  
**Status:** Early Development (Phase 2.2/2.3)  
**Current Focus:** Configuration System & HTTP Integration

---

## 📋 Project Overview

**Zed Copilot** is an AI-powered chat assistant extension for Zed IDE, enabling interactive conversations with AI directly in the editor. Built in Rust as a WebAssembly extension with support for multiple AI providers (OpenAI, Anthropic Claude).

### Key Metadata
- **Repository:** `https://github.com/zed-industries/zed-copilot`
- **Extension ID:** `zed-copilot`
- **Version:** 0.0.1
- **Edition:** Rust 2021
- **License:** MIT
- **Crate Type:** `cdylib` (WebAssembly)

---

## 🏗️ Architecture

### High-Level Design
```
Zed IDE (Host)
  ↓
Zed Copilot Extension (WebAssembly)
  ├── Extension Root (src/lib.rs)
  ├── AI Provider Manager (Phase 2.1 ✅)
  ├── Configuration Manager (Phase 2.2 🔄)
  ├── HTTP Client (Phase 2.3 ⏳)
  ├── Chat Engine (Phase 3 ⏳)
  └── Completion Engine (Phase 4 ⏳)
  ↓
External AI Providers
  ├── OpenAI (GPT-4, GPT-3.5-turbo) ✅
  ├── Anthropic Claude ✅
  └── Future: Ollama, self-hosted
```

### Core Components

#### 1. **Extension Entry Point** (`src/lib.rs`)
- Main `ZedCopilot` struct implementing `zed::Extension` trait
- Initializes extension and logs status
- Contains 5 unit tests for initialization

#### 2. **AI Provider Abstraction** (`src/providers/`)
- **`trait_def.rs`** — `AiProvider` trait (4 methods)
  - `async fn complete(&self, prompt: &str) -> ProviderResult<String>`
  - `async fn is_available(&self) -> bool`
  - `fn name(&self) -> &str`
  - `fn model(&self) -> &str`
- **`openai.rs`** — OpenAI implementation
  - Supports GPT-4, GPT-3.5-turbo
  - Full test coverage
- **`anthropic.rs`** — Anthropic Claude implementation
  - Supports Claude 3 family (Opus, Sonnet, Haiku)
  - Full test coverage
- **`factory.rs`** — `ProviderFactory` for instantiation
- **`error.rs`** — Error types (`ProviderError`, `ProviderResult`)
  - `ApiError`, `ConfigError`, `NetworkError`, `ParseError`, `NotAvailable`

---

## 📊 Project Status

### Phase 2.1: AI Provider Abstraction ✅ COMPLETE
- ✅ Trait-based `AiProvider` interface
- ✅ OpenAI provider implementation
- ✅ Anthropic provider implementation
- ✅ Provider factory
- ✅ Error handling (5 error variants)
- ✅ 31 unit tests passing

### Phase 2.2: Configuration & Credentials 🔄 IN PROGRESS
- 🔄 Configuration loader for settings.json
- 🔄 Credential validation
- 🔄 Environment variable interpolation (`${VAR_NAME}`)
- 🔄 Per-provider configuration support
- 📁 Documented in `docs/CONFIGURATION.md`
- Expected: Q1 2025

### Phase 2.3: HTTP Integration & Streaming 🔄 NEXT
- 🔄 Actual API calls to providers
- 🔄 Streaming responses (SSE) for chat UX
- 🔄 Retry logic with exponential backoff
- 🔄 Rate limiting enforcement
- 🔄 Network failure handling
- Expected: Q1 2025

### Phase 3: Chat Interface ⏳ PRIMARY FEATURE
- ⏳ Interactive chat panel in Zed
- ⏳ Multi-turn conversation support
- ⏳ Message history storage/retrieval
- ⏳ Real-time streaming display
- ⏳ Code context integration
- 📁 Architecture documented in `docs/CHAT_ARCHITECTURE.md`
- Expected: Q2 2025

### Phase 4: Code Completion ⏳ OPTIONAL
- ⏳ Inline code suggestions
- ⏳ Code refactoring assistance
- ⏳ Documentation generation
- Expected: Q3 2025+

---

## 📁 Project Structure

```
zed-copilot/
├── extension.toml              # Extension metadata (id, name, version, authors)
├── Cargo.toml                  # Dependencies & build config
│   └── deps: zed_extension_api, async-trait, serde, serde_json
├── src/
│   ├── lib.rs                  # Main extension (8 tests)
│   └── providers/
│       ├── mod.rs              # Module exports
│       ├── trait_def.rs        # AiProvider trait
│       ├── openai.rs           # OpenAI provider
│       ├── anthropic.rs        # Anthropic provider
│       ├── factory.rs          # ProviderFactory
│       └── error.rs            # Error types
├── tests/
│   ├── common/mod.rs           # Test utilities
│   └── integration_tests.rs    # Integration tests
├── docs/
│   ├── README.md               # User guide & features
│   ├── DEVELOPMENT.md          # Architecture & design
│   ├── SETUP.md                # Installation instructions
│   ├── QUICKSTART.md           # 5-minute quick start
│   ├── ROADMAP.md              # Feature timeline
│   ├── TESTING.md              # Test strategy
│   ├── PROVIDER_INTEGRATION.md # Provider API details
│   ├── CONFIGURATION.md        # Config documentation (Phase 2.2)
│   ├── CHAT_ARCHITECTURE.md    # Chat design (Phase 3 planned)
│   ├── CHANGELOG.md            # Version history
│   └── MAKEFILE.md             # Build commands
├── .github/
│   ├── workflows/              # CI/CD pipelines
│   └── CONTRIBUTING.md         # Contributing guidelines
├── Makefile                    # Development commands
├── CI_CD_IMPLEMENTATION.md     # CI/CD details
└── LICENSE                     # MIT License
```

---

## 🔧 Dependencies

### Core
- `zed_extension_api = "0.1"` — Zed extension API
- `async-trait = "0.1"` — Async trait support
- `serde = "1.0"` — Serialization (with derive feature)
- `serde_json = "1.0"` — JSON support

### Dev Only
- `tokio = "1.0"` — Async runtime (full features)

### Build Profile (Release)
- Optimization level: `z` (size optimization)
- LTO: Enabled
- Strip: Enabled

---

## 🧪 Testing

### Current Coverage
- **Total Tests:** 40+
- **Status:** 100% passing ✅
- **Unit Tests:** src/lib.rs (8 tests) + providers module (31 tests)
- **Integration Tests:** tests/integration_tests.rs

### Test Breakdown
| Component | Tests | Status |
|-----------|-------|--------|
| Extension (lib.rs) | 8 | ✅ Complete |
| Providers | 31 | ✅ Complete |
| **Total** | **40+** | **✅ Passing** |

### Key Tests
- Extension initialization
- Default implementation
- Extension trait implementation
- Multiple instance creation
- Provider instantiation and availability checks
- Error handling and propagation

---

## 🛠️ Development Workflow

### Build Commands
```bash
make fmt              # Format code
make clippy          # Lint code
make test            # Run tests
make check-all       # Format + Clippy + Tests
cargo build          # Dev build
cargo build --release # Release build
```

### Key Principles
Per [zed-rules/AGENTS.md](https://github.com/zed-industries/zed-rules/blob/main/AGENTS.md):
- **Code explains itself** — No comments or docstrings
- **Clear names** — Intention-revealing identifiers
- **Small functions** — Single responsibility
- **Simple control flow** — Early returns preferred
- **Type-driven design** — Use types, not comments
- **Consistent style** — Match existing code

---

## 🔐 Configuration

### Settings Format (settings.json)
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

### Environment Variables
- `OPENAI_API_KEY` — OpenAI API key
- `ANTHROPIC_API_KEY` — Anthropic API key

---

## 🤖 Supported Providers

### OpenAI ✅
- **Models:** GPT-4, GPT-3.5-turbo
- **Status:** Implemented & tested
- **Configuration key:** `openai`

### Anthropic Claude ✅
- **Models:** Claude 3 (Opus, Sonnet, Haiku)
- **Status:** Implemented & tested
- **Configuration key:** `anthropic`

### Future Providers (Planned)
- Ollama (local models)
- Self-hosted LLM services
- Additional commercial providers

---

## 🚀 Getting Started

### Prerequisites
- Zed IDE (latest)
- Rust (via rustup)
- API key from OpenAI or Anthropic

### Installation
1. Clone: `git clone https://github.com/zed-industries/zed-copilot.git`
2. Build: `cargo build --release`
3. Install in Zed: Extensions → Install Dev Extension → Select zed-copilot
4. Configure: Add settings.json with API key and provider
5. Restart Zed

---

## 📚 Key Files Summary

| File | Purpose | Key Content |
|------|---------|------------|
| `src/lib.rs` | Extension entry point | `ZedCopilot` struct, 8 tests |
| `src/providers/trait_def.rs` | Provider interface | `AiProvider` trait (4 methods) |
| `src/providers/openai.rs` | OpenAI implementation | `OpenAiProvider` struct |
| `src/providers/anthropic.rs` | Anthropic implementation | `AnthropicProvider` struct |
| `src/providers/factory.rs` | Provider creation | `ProviderFactory` |
| `src/providers/error.rs` | Error types | `ProviderError` enum (5 variants) |
| `Cargo.toml` | Dependencies | Core deps + dev deps |
| `extension.toml` | Extension metadata | ID, name, version, authors |
| `docs/DEVELOPMENT.md` | Architecture guide | Detailed design & phases |
| `docs/ROADMAP.md` | Timeline | Phase breakdown & dates |

---

## 🎯 Next Steps (Phase 2.2)

1. ✅ Provider abstraction complete
2. 🔄 Implement configuration loader
3. 🔄 Add credential validation
4. 🔄 Support environment variable interpolation
5. 🔄 Add configuration tests
6. ⏳ Move to Phase 2.3: HTTP integration

---

## 💡 Quick Reference

### Error Handling
- Use `ProviderResult<T>` = `Result<T, ProviderError>`
- Error variants: `ApiError`, `ConfigError`, `NetworkError`, `ParseError`, `NotAvailable`

### Async Code
- All provider ops use `async_trait`
- Non-blocking calls prevent UI freezing
- Streaming support planned for Phase 2.3

### Code Style
- No inline comments or docstrings
- Clear naming: `fetchUserProfile` (verb for functions)
- Single responsibility per unit
- Early returns for control flow

---

## 📖 Documentation References
- [README.md](README.md) — User guide
- [DEVELOPMENT.md](DEVELOPMENT.md) — Architecture
- [ROADMAP.md](ROADMAP.md) — Timeline
- [SETUP.md](SETUP.md) — Installation
- [TESTING.md](TESTING.md) — Test strategy
- [PROVIDER_INTEGRATION.md](PROVIDER_INTEGRATION.md) — Provider details

---

## 📝 Recent Changes (2025-01-09)

- ✅ Documentation expanded with CHAT_ARCHITECTURE.md
- 📋 CHANGELOG.md established with version history and release process
- 🔄 Phase 2.2 configuration system still in progress
- 🔄 Phase 2.3 HTTP integration next (reqwest, SSE, rate limiting)
- 📚 Full documentation structure complete with 10+ guides

---

**Status:** Phase 2.2 (Configuration) in progress → Phase 2.3 (HTTP) next  
**Last Sync:** 2025-01-09  
**Next Milestone:** Phase 2.3 HTTP Integration & Streaming (Q1 2025)
