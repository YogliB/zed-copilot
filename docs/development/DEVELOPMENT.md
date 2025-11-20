# Development Guide

Complete guide for developers working on Zed Copilot.

> **Part of:** [Zed Copilot Documentation](../README.md)

---

## Architecture Overview

Zed Copilot is a WebAssembly-based AI extension for Zed IDE built in Rust. The primary feature is interactive chat with AI (Phase 3), with GitHub Copilot LSP integration for code completions planned for Phase 4.

### Core Components

```
┌─────────────────────────────────────────────────┐
│           Zed IDE (Host)                        │
├─────────────────────────────────────────────────┤
│  Zed Copilot Extension (WebAssembly)            │
│  ┌──────────────────────────────────────────┐   │
│  │ ZedCopilot (Extension)                   │   │
│  ├── AI Provider Manager (Phase 2.1) ✅     │   │
│  ├── Configuration Manager (Phase 2.2) ✅   │   │
│  ├── HTTP Client (Phase 2.3) ⏳              │   │
│  ├── Streaming Handler (Phase 2.3) ⏳        │   │
│  ├── Chat Engine (Phase 3) ⏳                 │   │
│  ├── Chat UI Panel (Phase 3) ⏳               │   │
│  ├── Message History (Phase 3) ⏳             │   │
│  ├── Context Manager (Phase 3) ⏳             │   │
│  └── Completion Engine (Phase 4) ⏳ optional │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
         ↓
┌─────────────────────────────────────────────────┐
│  External AI Providers                          │
│  ├── OpenAI API (GPT-4o, o1, o3-mini) ✅      │
│  ├── Anthropic Claude (Claude Opus 4.1, Sonnet 4, Haiku 4.5) ✅     │
│  └── Future: Ollama, self-hosted, others        │
└─────────────────────────────────────────────────┘
```

**Legend:** ✅ Complete | 🔄 In Progress | ⏳ Planned

## Project Structure

```
zed-copilot/
├── extension.toml              # Zed extension metadata
├── Cargo.toml                  # Rust dependencies and build config
├── src/
│   ├── lib.rs                  # Main extension + unit tests
│   ├── providers/              # AI provider abstraction (Phase 2.1 ✅)
│   │   ├── mod.rs
│   │   ├── trait_def.rs        # AiProvider trait
│   │   ├── openai.rs           # OpenAI implementation
│   │   ├── anthropic.rs        # Anthropic implementation
│   │   ├── factory.rs          # Provider factory
│   │   └── error.rs            # Error types
│   └── config/                 # Configuration system (Phase 2.2 ✅)
│       ├── mod.rs
│       ├── structs.rs          # Configuration data structures
│       ├── loader.rs           # JSON and env var loading
│       ├── validator.rs        # Configuration validation
│       ├── manager.rs          # ConfigManager facade
│       └── errors.rs           # Error types
├── tests/
│   ├── common/mod.rs           # Test utilities
│   └── integration_tests.rs    # Integration tests
├── docs/
│   ├── README.md               # User guide
│   ├── SETUP.md                # Installation instructions
│   ├── QUICKSTART.md           # 5-minute start
│   ├── DEVELOPMENT.md          # This file
│   ├── ROADMAP.md              # Feature timeline
│   ├── TESTING.md              # Test strategy
│   ├── PROVIDER_INTEGRATION.md # Provider API details
│   ├── CHANGELOG.md            # Version history
│   └── MAKEFILE.md             # Build commands
├── .github/
│   ├── workflows/              # CI/CD pipelines
│   └── CONTRIBUTING.md         # Contributing guidelines
├── Makefile                    # Development shortcuts
└── LICENSE                     # MIT License
```

## Current Status

**Current Phase:** Phase 2.3 ✅ Complete

**Completed Phases:**
- ✅ Phase 1 — Foundation
- ✅ Phase 2.1 — AI Provider Abstraction
- ✅ Phase 2.2 — Configuration System
- ✅ Phase 2.3 — HTTP Integration & Streaming

**Next Phase:** Phase 3 — Chat Interface (Q2 2025) 🎯

See [ROADMAP.md](ROADMAP.md) for complete timeline. 📅

## Development Workflow

### Prerequisites

Before starting development:

1. Complete setup from [../getting-started/SETUP.md](../getting-started/SETUP.md)
2. Have Rust and Zed installed
3. Extension built and loaded in Zed

### Quick Reference

```bash
# Clone and build (if not done)
git clone https://github.com/zed-industries/zed-copilot.git
cd zed-copilot
cargo build --release
```

### Development Cycle

**1. Edit Code**
```bash
# Edit files in src/
```

**2. Run Checks**
```bash
make fmt        # Format code
make clippy     # Check warnings
make test       # Run tests
make check-all  # Run all checks
```

**3. Build**
```bash
cargo build --release
```

**4. Reload in Zed**
- Restart Zed or reload extension
- Check logs: `zed: open log`

**5. Debug**
```bash
zed --foreground  # See real-time logs
```

### Code Standards

Follow the principles in [zed-rules/AGENTS.md](https://github.com/zed-industries/zed-rules/blob/main/AGENTS.md):

- **Code must explain itself** — No comments or docstrings
- **Clear names** — Use intention-revealing names (`fetchUserProfile` not `fu`)
- **Small functions** — One purpose per unit
- **Simple control flow** — Early returns preferred
- **Express intent via types** — Use types and tests, not comments
- **Consistent style** — Match existing code

### Make Commands

```bash
make help       # Show all commands
make fmt        # Format with rustfmt
make clippy     # Lint with clippy
make test       # Run all tests
make check-all  # Format + clippy + test
make build      # Build release binary
make clean      # Clean build artifacts
```

## Testing

### Running Tests

```bash
cargo test                  # All tests
cargo test --lib           # Unit tests only
cargo test --test '*'      # Integration tests only
cargo test test_name       # Specific test
cargo test -- --nocapture  # Show println! output
```

**Current Coverage:** 63 tests, all passing ✅

See [TESTING.md](TESTING.md) for detailed testing guide.

### Test Coverage by Component

| Component | Tests | Coverage |
|-----------|-------|----------|
| Extension | 5 | ✅ Complete |
| Providers | 31 | ✅ Complete |
| Configuration | 27 | ✅ Complete |
| **Total** | **63** | **All Passing** |

## Key Components

### Configuration System

Manages settings and credentials:

```
ConfigManager (facade)
  ├── ConfigLoader (JSON parsing)
  ├── EnvInterpolator (${VAR_NAME} substitution)
  ├── ConfigValidator (validation rules)
  └── Structs (RootConfig, OpenAiConfig, AnthropicConfig, ChatConfig)
```

**Features:**
- Load from Zed settings.json
- Environment variable interpolation: `${OPENAI_API_KEY}`
- Validation with clear error messages
- Per-provider configuration

**See:** [../getting-started/EXAMPLES.md](../getting-started/EXAMPLES.md) for configuration examples

### Provider System

AI provider abstraction via traits:

```rust
#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn complete(&self, prompt: &str) -> ProviderResult<String>;
    async fn is_available(&self) -> bool;
    fn name(&self) -> &str;
    fn model(&self) -> &str;
}
```

**Implementations:**
- `OpenAiProvider` — GPT-4o, o1, o3-mini
- `AnthropicProvider` — Claude Opus, Sonnet, Haiku
- `ProviderFactory` — Convenient provider creation

**See:** [../technical/PROVIDER_INTEGRATION.md](../technical/PROVIDER_INTEGRATION.md) for details

## Common Tasks

### Adding a New Feature

1. **Design** — Document in `docs/` if significant
2. **Write Tests** — TDD approach recommended
3. **Implement** — Follow code standards
4. **Verify** — `make check-all`
5. **Document** — Update relevant docs

### Adding a New Provider

1. Create `src/providers/myprovider.rs`
2. Implement `AiProvider` trait
3. Add to `ProviderFactory`
4. Write unit tests
5. Update `PROVIDER_INTEGRATION.md`

Example:
```rust
pub struct MyProvider {
    api_key: String,
    model: String,
}

#[async_trait]
impl AiProvider for MyProvider {
    async fn complete(&self, prompt: &str) -> ProviderResult<String> {
        // Implementation
    }
    // ... other methods
}
```

### Adding Tests

```rust
#[test]
fn test_my_feature() {
    // Arrange
    let input = setup_test_data();
    
    // Act
    let result = my_function(input);
    
    // Assert
    assert_eq!(result, expected_value);
}
```

Run: `cargo test test_my_feature`

### Debugging

```bash
# Run Zed with logs
zed --foreground

# Look for: [Zed Copilot] messages
```

### Updating Dependencies

```bash
cargo update
make check-all
```

## Architecture Principles

### Trait-Based Abstraction (Phase 2.1 ✅)

Providers implement `AiProvider` trait, enabling:
- Easy provider switching at runtime
- Simple provider fallback mechanisms
- Type-safe error handling via `ProviderResult<T>`
- Support for new providers without core changes

### Configuration (Phase 2.2 🔄)

- Load settings from Zed's `settings.json`
- Support environment variable interpolation
- Validate before use (fail fast on config errors)
- No hardcoded credentials

### Error Handling

Use `Result<T, ProviderError>` for operations that can fail:

```rust
pub enum ProviderError {
    ApiError(String),       // API call failures
    ConfigError(String),    // Configuration issues
    NetworkError(String),   // Network problems
    ParseError(String),     // Response parsing failures
    NotAvailable(String),   // Provider unavailable
}
```

### Async/Await

All provider operations are async using `async_trait`:

```rust
#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn complete(&self, prompt: &str) -> ProviderResult<String>;
    async fn is_available(&self) -> bool;
    fn name(&self) -> &str;
    fn model(&self) -> &str;
}
```

## Performance Considerations

- **Async I/O:** Non-blocking provider calls prevent UI freezing
- **Streaming:** Phase 2.3 will stream responses for real-time UX
- **Lazy Loading:** Providers instantiated only when needed
- **Caching:** Future phases will add request caching

## Security Considerations

- **No Hardcoded Credentials:** API keys come from environment or settings
- **Environment Variable Interpolation:** Settings use `${VARIABLE_NAME}` pattern
- **Per-Provider Configuration:** Each provider manages its own credentials
- **Input Validation:** All user input validated before use

## Code Standards

Follow principles from [zed-rules/AGENTS.md](https://github.com/zed-industries/zed-rules/blob/main/AGENTS.md):

- **Self-explanatory code** — No comments or docstrings
- **Clear names** — `fetchUserProfile` not `fu`
- **Small functions** — One purpose per unit
- **Simple control flow** — Early returns preferred
- **Types over comments** — Express intent via types
- **Consistent style** — Match existing code

### Naming

- **Functions:** Verbs in snake_case (`fetch_user_profile`)
- **Types:** Nouns in PascalCase (`RetryPolicy`)
- **Constants:** UPPER_SNAKE_CASE (`MAX_RETRIES`)
- **Clarity over brevity:** `timeout_ms` not `t`

### Testing

- Test both happy paths and errors
- Use Arrange-Act-Assert pattern
- Keep tests focused and independent
- Descriptive names: `test_provider_returns_error_when_api_key_invalid`

## Resources

### Documentation
- [../README.md](../../README.md) — User guide
- [ROADMAP.md](ROADMAP.md) — Feature timeline
- [TESTING.md](TESTING.md) — Testing strategy
- [../technical/PROVIDER_INTEGRATION.md](../technical/PROVIDER_INTEGRATION.md) — Provider API details
- [../getting-started/SETUP.md](../getting-started/SETUP.md) — Installation guide
- [../README.md](../README.md) — Documentation index

### External
- [Zed Extension API](https://zed.dev/docs/extensions)
- [Rust Book](https://doc.rust-lang.org/book/)
- [async-trait crate](https://docs.rs/async-trait/)
- [serde_json crate](https://docs.rs/serde_json/)

---

**Current Phase:** Phase 2.3 Complete ✅  
**Next Milestone:** Phase 3 — Chat Interface (Q2 2025) 🎯  
**Contributing:** See [CONTRIBUTING.md](CONTRIBUTING.md)

---

**Back to:** [Development](../README.md#quick-navigation)