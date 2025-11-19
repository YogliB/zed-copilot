# Zed Copilot Development Guide

## Architecture Overview

Zed Copilot is a WebAssembly-based AI extension for Zed IDE built in Rust. **Chat is the primary feature** (Phase 3), with optional code completion in Phase 4.

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
│  ├── OpenAI API (GPT-4, GPT-3.5-turbo) ✅      │
│  ├── Anthropic Claude (Claude 3 family) ✅     │
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

## Current Phase: 2.2 (Configuration & Credentials)

### Phase 2.1: AI Provider Abstraction ✅ COMPLETE

**Delivered:**
- Trait-based `AiProvider` interface supporting multiple providers
- `OpenAiProvider` with GPT-4 and GPT-3.5-turbo support
- `AnthropicProvider` with Claude 3 family support
- `ProviderFactory` for convenient instantiation
- Error handling with `ProviderError` enum
- 31 unit tests with 100% pass rate

**See:** `PROVIDER_INTEGRATION.md` for implementation details

### Phase 2.2: Configuration & Credentials ✅ COMPLETE

**Delivered:**
- Configuration loader for settings.json (from Zed settings or JSON strings)
- Environment variable interpolation via `${VAR_NAME}` syntax
- Per-provider configuration structs (OpenAI, Anthropic)
- Chat-specific configuration options (streaming, history, context)
- Comprehensive validation with clear error messages
- `ConfigManager` facade for simple API
- 30+ unit tests with 90%+ coverage
- Complete documentation (CONFIG.md, JSON schema)

**See:** `docs/CONFIG.md` for user guide and schema documentation

### Phase 2.3: HTTP Integration & Streaming (NEXT)

**Goals:**
- Make actual API calls to providers
- Support streaming responses for chat UX
- Implement retry logic with exponential backoff
- Handle network failures gracefully

### Phase 3: Chat Interface (PRIMARY FEATURE)

**Goals:**
- Interactive chat panel in Zed
- Multi-turn conversation support
- Message history storage and retrieval
- Real-time streaming response display
- Code context integration

**See:** `ROADMAP.md` for detailed Phase 3 timeline and deliverables

## Development Workflow

### Setup Development Environment

```bash
# 1. Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Verify installation
rustc --version
cargo --version

# 3. Clone and enter project
git clone https://github.com/zed-industries/zed-copilot.git
cd zed-copilot

# 4. Build the extension
cargo build --release

# 5. Install in Zed as dev extension
# Open Zed → Extensions → Install Dev Extension → Select zed-copilot directory
```

### Making Changes

1. **Edit source files** in `src/`
2. **Build:** `cargo build --release`
3. **Test:** `cargo test`
4. **Lint:** `cargo clippy`
5. **Format:** `cargo fmt`
6. **Verify:** `make check-all` (runs format + clippy + tests)

### Code Standards

Follow the principles in [zed-rules/AGENTS.md](https://github.com/zed-industries/zed-rules/blob/main/AGENTS.md):

- **Code must explain itself** — No comments or docstrings
- **Clear names** — Use intention-revealing names (`fetchUserProfile` not `fu`)
- **Small functions** — One purpose per unit
- **Simple control flow** — Early returns preferred
- **Express intent via types** — Use types and tests, not comments
- **Consistent style** — Match existing code

### Running Checks

```bash
# Format code
make fmt

# Check for warnings
make clippy

# Run tests
make test

# Run all checks (do this before committing)
make check-all
```

## Testing

### Test Structure

- **Unit tests:** In `src/lib.rs` and within modules
- **Integration tests:** In `tests/` directory
- **Current coverage:** 40+ tests, 100% pass rate

### Run Tests

```bash
cargo test                    # All tests
cargo test --lib             # Unit tests only
cargo test --test '*'        # Integration tests only
cargo test test_name          # Specific test
cargo test -- --nocapture    # Show println! output
```

## Current Test Coverage

| Component | Tests | Status |
|-----------|-------|--------|
| Extension | 5 | ✅ Complete |
| Providers | 31 | ✅ Complete |
| Configuration | 27 | ✅ Complete |
| **Total** | **63** | **✅ Passing** |

### Test Expectations by Phase

- **Phase 2.2:** Configuration tests (27 tests, 90%+ coverage) ✅
- **Phase 2.3:** Add HTTP client tests (with mocks)
- **Phase 3:** Add chat engine and UI tests (85%+ coverage target)

## Configuration System (Phase 2.2) ✅

### Architecture

The configuration system provides secure, validated setup for AI providers:

```
ConfigManager (facade)
  ├── ConfigLoader (JSON parsing + file loading)
  ├── EnvInterpolator (${VAR_NAME} substitution)
  ├── ConfigValidator (validation rules)
  └── Structs (RootConfig, OpenAiConfig, AnthropicConfig, ChatConfig)
```

### Key Components

**ConfigManager** — Main entry point for configuration
```rust
pub struct ConfigManager;

impl ConfigManager {
    pub fn initialize() -> ConfigResult<Self>
    pub fn initialize_from_json(json: &str) -> ConfigResult<Self>
    pub fn get_active_provider(&self) -> ConfigResult<ProviderConfig>
    pub fn get_chat_config(&self) -> ChatConfig
    pub fn is_enabled(&self) -> bool
}
```

**EnvInterpolator** — Handles environment variable substitution
```rust
pub struct EnvInterpolator;

impl EnvInterpolator {
    pub fn interpolate(value: &str) -> ConfigResult<String>
}
```

Supports `${VARIABLE_NAME}` syntax:
```json
{
  "openai": {
    "api_key": "${OPENAI_API_KEY}"
  }
}
```

**ConfigValidator** — Validates configuration before use
```rust
pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate(config: &RootConfig) -> ConfigResult<()>
}
```

Enforces:
- Provider selection (must be "openai" or "anthropic")
- Required fields (api_key, model)
- Valid values (timeout > 0, history > 0)

### Testing Configuration

#### Unit Tests

Run configuration tests:
```bash
cargo test config::
cargo test config::manager::
cargo test config::loader::
cargo test config::validator::
```

#### Test with JSON

```rust
#[test]
fn test_my_config() {
    let json = r#"
    {
        "enabled": true,
        "provider": "openai",
        "openai": {
            "api_key": "test_key",
            "model": "gpt-4"
        }
    }
    "#;
    
    let manager = ConfigManager::initialize_from_json(json).unwrap();
    assert!(manager.is_enabled());
}
```

#### Test with Environment Variables

```rust
#[test]
fn test_env_interpolation() {
    std::env::set_var("TEST_API_KEY", "secret_key");
    
    let json = r#"
    {
        "enabled": true,
        "provider": "openai",
        "openai": {
            "api_key": "${TEST_API_KEY}"
        }
    }
    "#;
    
    let manager = ConfigManager::initialize_from_json(json).unwrap();
    let provider = manager.get_active_provider().unwrap();
    assert_eq!(provider.api_key(), "secret_key");
}
```

### Configuration Schema

See `docs/CONFIG.md` for complete documentation including:
- Configuration schema and fields
- Environment variable setup
- Provider-specific examples
- Troubleshooting guide
- Best practices and security

JSON Schema available in `docs/settings.schema.json` for IDE autocomplete.

See `TESTING.md` for detailed test strategy.

## Common Development Tasks

### Build for Release

```bash
cargo build --release
```

Produces optimized WebAssembly binary in `target/wasm32-unknown-unknown/release/`.

### Debug with Zed Logs

```bash
# Run Zed in foreground to see extension logs
zed --foreground

# Look for [Zed Copilot] prefixed log messages
```

### Add a New Test

```rust
#[test]
fn test_my_feature() {
    // Arrange
    let setup = prepare_test();
    
    // Act
    let result = my_function();
    
    // Assert
    assert_eq!(result, expected_value);
}
```

### Update Dependencies

```bash
cargo update
```

Then verify builds and tests pass:

```bash
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

## Useful Resources

- **Zed Extension API:** https://zed.dev/docs/extensions
- **Rust Book:** https://doc.rust-lang.org/book/
- **async-trait crate:** https://docs.rs/async-trait/
- **serde_json crate:** https://docs.rs/serde_json/

## Related Documentation

- [README.md](README.md) — User guide and features
- [ROADMAP.md](ROADMAP.md) — Full feature timeline and phases
- [TESTING.md](TESTING.md) — Test strategy and guidelines
- [PROVIDER_INTEGRATION.md](PROVIDER_INTEGRATION.md) — Provider API details
- [SETUP.md](SETUP.md) — Installation and environment setup

---

**Current Status:** Phase 2.2 Complete ✅  
**Next Milestone:** Phase 2.3 HTTP Integration & Streaming  
**Primary Feature:** Chat Interface (Phase 3, Q2 2025)