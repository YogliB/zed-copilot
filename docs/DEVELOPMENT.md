# Zed Copilot Development Guide

## Architecture Overview

Zed Copilot is built as a WebAssembly-based extension for Zed IDE using Rust. The architecture is designed to be modular and extensible, with **chat as the primary feature** and code completion as an optional secondary feature.

### Core Components

```
┌─────────────────────────────────────────────────┐
│           Zed IDE (Host)                        │
├─────────────────────────────────────────────────┤
│  Zed Copilot Extension (WebAssembly)            │
│  ┌──────────────────────────────────────────┐   │
│  │ ZedCopilot (Extension)                   │   │
│  ├── Chat Engine (Phase 3)                  │   │
│  ├── Chat UI Panel (Phase 3)                │   │
│  ├── Message History Manager (Phase 3)      │   │
│  ├── Configuration Manager (Phase 2.2)      │   │
│  ├── AI Provider Manager (Phase 2.1)        │   │
│  ├── Streaming Response Handler (Phase 2.3) │   │
│  ├── Context Manager (Phase 3)              │   │
│  ├── Logger/Telemetry                       │   │
│  └── (Future: Completion Engine - Phase 4)  │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
         ↓
┌─────────────────────────────────────────────────┐
│  External AI Providers (Phase 2.1 Complete)     │
│  ├── OpenAI API (GPT-4, GPT-3.5-turbo)         │
│  ├── Anthropic Claude API (Claude 3 family)     │
│  └── Future: Ollama, self-hosted, others        │
└─────────────────────────────────────────────────┘
```

### Current Implementation (v0.0.1)

Currently, the extension provides:
- **Extension Registration**: Basic ZedCopilot struct implementing the Extension trait
- **Initialization**: Logging on startup to verify proper loading
- **Test Framework**: Unit and integration tests with 40+ passing tests
- **Provider Abstraction**: Complete AI provider interface (Phase 2.1) ✅
- **Foundation**: Ready for configuration system (Phase 2.2)

### Phase-by-Phase Component Rollout

| Component | Phase | Status | Purpose |
|-----------|-------|--------|---------|
| Extension Struct | 1 | ✅ Complete | Loads extension in Zed |
| Provider Trait | 2.1 | ✅ Complete | Abstract AI provider interface |
| OpenAI Provider | 2.1 | ✅ Complete | GPT-4, GPT-3.5-turbo support |
| Anthropic Provider | 2.1 | ✅ Complete | Claude 3 family support |
| Configuration Manager | 2.2 | 🔄 In Progress | Load settings, manage credentials |
| HTTP Client | 2.3 | ⏳ Planned | Make API calls, handle responses |
| Streaming Support | 2.3 | ⏳ Planned | Stream responses token-by-token |
| Chat Engine | 3 | ⏳ Planned | Handle conversations, context |
| Chat UI Panel | 3 | ⏳ Planned | Zed UI integration for chat |
| Message History | 3 | ⏳ Planned | Store and retrieve conversation |
| Completion Engine | 4 | ⏳ Optional | Inline code completion (optional) |

## Project Structure

```
zed-copilot/
├── extension.toml              # Zed extension metadata
├── Cargo.toml                  # Rust dependencies and build config
├── src/
│   ├── lib.rs                  # Main extension (with unit tests)
│   │
│   ├── providers/              # AI provider abstraction (Phase 2.1 ✅)
│   │   ├── mod.rs              # Module exports
│   │   ├── trait_def.rs        # AiProvider trait definition
│   │   ├── openai.rs           # OpenAI provider implementation
│   │   ├── anthropic.rs        # Anthropic Claude provider
│   │   ├── factory.rs          # Provider factory pattern
│   │   ├── error.rs            # Error types and handling
│   │   └── (with tests)         # 31 unit tests
│   │
│   ├── config/                 # Configuration system (Phase 2.2)
│   │   ├── mod.rs              # Module root
│   │   ├── settings.rs         # Zed settings parsing
│   │   ├── credentials.rs      # Credential management
│   │   └── validation.rs       # Config validation
│   │
│   ├── http/                   # HTTP client (Phase 2.3)
│   │   ├── mod.rs
│   │   ├── client.rs           # HTTP client wrapper
│   │   ├── streaming.rs        # Streaming response handling
│   │   └── retry.rs            # Retry logic with backoff
│   │
│   ├── chat/                   # Chat engine (Phase 3)
│   │   ├── mod.rs
│   │   ├── engine.rs           # Chat conversation logic
│   │   ├── message.rs          # Message types
│   │   ├── history.rs          # Conversation history
│   │   └── context.rs          # Code context integration
│   │
│   ├── ui/                     # UI integration (Phase 3)
│   │   ├── mod.rs
│   │   ├── panel.rs            # Chat panel component
│   │   ├── events.rs           # User input handling
│   │   └── rendering.rs        # Response rendering
│   │
│   └── completion/             # Completion engine (Phase 4 - Optional)
│       ├── mod.rs
│       └── engine.rs           # Inline completion logic
│
├── tests/
│   ├── common/
│   │   └── mod.rs              # Shared test utilities
│   ├── integration_tests.rs    # Integration tests
│   └── chat_tests.rs           # Chat-specific tests (Phase 3)
│
├── docs/
│   ├── DEVELOPMENT.md          # This file
│   ├── ROADMAP.md              # Project roadmap and milestones
│   ├── CHAT_ARCHITECTURE.md    # Chat design details (Phase 3)
│   ├── UI_INTEGRATION.md       # Zed UI APIs (Phase 3)
│   ├── CONFIGURATION.md        # Settings format (Phase 2.2)
│   ├── PROVIDER_INTEGRATION.md # Provider reference
│   ├── TESTING.md              # Testing guide
│   ├── DOCUMENTATION_REVIEW.md # Alignment review
│   └── CHANGELOG.md            # Version history
│
├── README.md                   # User guide
├── LICENSE                     # MIT License
├── Makefile                    # Development commands
└── .gitignore                  # Git ignore rules
```

## Development Workflow

### Setup Development Environment

1. Clone the repository and navigate to it:
   ```bash
   git clone https://github.com/zed-industries/zed-copilot.git
   cd zed-copilot
   ```

2. Ensure Rust is installed via rustup (not Homebrew):
   ```bash
   rustup --version
   rustup update stable
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. Install as dev extension in Zed:
   - Open Zed IDE
   - Go to Extensions panel (`zed: open extensions`)
   - Click "Install Dev Extension"
   - Select the `zed-copilot` directory

5. Verify it loads:
   ```bash
   zed --foreground
   ```
   Look for `[Zed Copilot] Extension initialized` in logs.

### Making Changes

1. Edit source files in `src/`
2. Run tests to verify changes: `cargo test`
3. Format code: `cargo fmt`
4. Check for warnings: `cargo clippy`
5. Rebuild: `cargo build --release`
6. Reload extension in Zed (may require restart)
7. Check `zed: open log` for any errors

### Code Standards

Follow the [zed-rules](../zed-rules/AGENTS.md) coding guidelines:

- **Clear names** — Functions as verbs (`fetchUserProfile`), types as nouns (`RetryPolicy`)
- **Small functions** — One responsibility per function
- **No comments** — Code should explain itself through naming and structure
- **Type safety** — Use Rust's type system to prevent bugs
- **Early returns** — Simple control flow
- **Pure functions** — Minimize side effects
- **Tests** — Write tests for new functionality (see TESTING.md)

### Running Checks

Use the Makefile for convenient shortcuts:

```bash
# Run all quality checks (fmt, clippy, test)
make check-all

# Individual checks
make fmt          # Format code with rustfmt
make clippy       # Lint code with clippy
make test         # Run all tests
make build        # Build debug binary
make release      # Build optimized release binary

# View all available commands
make help
```

Or use cargo directly:

```bash
cargo fmt
cargo clippy
cargo test
cargo build --release
```

## Testing

Zed Copilot includes comprehensive tests at all levels: unit, integration, and component-specific.

### Quick Start

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_zed_copilot_new

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'
```

### Test Structure

- **Unit Tests**: Located in each module (`#[cfg(test)]` blocks), test individual components
- **Integration Tests**: Located in `tests/`, test component interactions
- **Provider Tests**: Located in `src/providers/`, comprehensive provider testing
- **Chat Tests**: Located in `tests/chat_tests.rs`, chat-specific flows (Phase 3)

### Current Test Coverage

- **Provider Tests (31)**: Provider instantiation, trait methods, factory, error handling ✅
- **Extension Tests (9)**: ZedCopilot initialization and integration ✅
- **Total: 40+ tests, all passing** ✅

### Adding Tests

When adding features, follow the Arrange-Act-Assert pattern:

```rust
#[test]
fn test_chat_sends_message() {
    // Arrange
    let chat = ChatEngine::new();
    let message = Message::new("Hello, AI!");
    
    // Act
    let result = chat.send_message(message);
    
    // Assert
    assert!(result.is_ok());
}
```

See `docs/TESTING.md` for comprehensive testing guide.

## CI/CD Pipeline

Zed Copilot uses GitHub Actions for automated quality checks on every commit and pull request.

### Automated Checks

1. **Tests** — All 40+ tests must pass
2. **Formatting** — Code must be formatted with `cargo fmt`
3. **Linting** — No warnings from `cargo clippy`
4. **Build** — WASM binary must compile successfully

### Workflow Triggers

CI runs automatically on:
- Push to `main` or `develop` branches
- All pull requests to `main` or `develop`
- Manual trigger via GitHub Actions UI

### Running CI Locally

```bash
# Run all quality checks
cargo fmt && cargo clippy && cargo test && cargo build --release

# Or use make
make check-all
```

### Simulation with `act`

Install [act](https://github.com/nektos/act) to run workflows locally:

```bash
brew install act
act              # Run all jobs
act -j test      # Run only test job
act -v           # Verbose output
```

## Phase-by-Phase Development Guide

### Phase 2.1: AI Provider Abstraction ✅ COMPLETE

**Status:** Provider trait and implementations complete with 31 passing tests.

**Components:**
- `AiProvider` trait defining interface
- `OpenAiProvider` implementation
- `AnthropicProvider` implementation
- `ProviderFactory` for instantiation
- `ProviderError` for error handling

**Key Files:**
- `src/providers/trait_def.rs` — Provider trait
- `src/providers/openai.rs` — OpenAI implementation
- `src/providers/anthropic.rs` — Anthropic implementation
- `src/providers/factory.rs` — Factory pattern

**What's Needed for Phase 2.2:**
- Configuration system to instantiate providers from settings
- HTTP client to make actual API calls (Phase 2.3)

### Phase 2.2: Configuration & Credentials (CURRENT)

**Focus:** Enable users to configure API keys and select providers.

**Components to Build:**
1. **Configuration Parser** (`src/config/settings.rs`)
   - Parse Zed settings.json
   - Validate required fields
   - Support environment variable interpolation

2. **Credential Manager** (`src/config/credentials.rs`)
   - Securely store/retrieve API keys
   - Handle credential validation
   - Support per-provider configuration

3. **Configuration Validator** (`src/config/validation.rs`)
   - Validate API keys format
   - Check provider configuration completeness
   - Error reporting with helpful messages

**Key Decisions:**
- Where to store credentials (Zed API capabilities)
- How to interpolate environment variables
- Configuration schema for chat-specific settings

**Tests to Add:**
- Configuration parsing tests
- Credential validation tests
- Environment variable interpolation tests
- Error handling for invalid configs

**Deliverables:**
- Configuration loading from Zed settings
- Credential validation and error messages
- Support for multiple providers simultaneously
- Chat-ready configuration schema

### Phase 2.3: HTTP Integration & Streaming Support (NEXT)

**Focus:** Make real API calls and support streaming responses for chat.

**Components to Build:**
1. **HTTP Client** (`src/http/client.rs`)
   - Wrapper around `reqwest` crate
   - Request building for OpenAI and Anthropic APIs
   - Response parsing and validation

2. **Streaming Handler** (`src/http/streaming.rs`)
   - Parse streaming responses (SSE format)
   - Buffer and emit tokens as they arrive
   - Handle incomplete/malformed chunks

3. **Retry Logic** (`src/http/retry.rs`)
   - Exponential backoff for transient failures
   - Configurable retry count and delays
   - Error classification (retryable vs. permanent)

**Key Decisions:**
- Streaming protocol handling (SSE vs. chunked HTTP)
- Token buffering strategy
- Retry configuration (max attempts, backoff multiplier)
- Timeout handling

**Tests to Add:**
- HTTP request/response tests (with mocks)
- Streaming response parsing tests
- Retry logic tests
- Timeout handling tests

**Deliverables:**
- Functional HTTP client for API calls
- Real-time streaming response support
- Robust error recovery with retry logic
- Full test coverage

### Phase 3: Chat Interface (PRIMARY FEATURE) 📋 PLANNED

**Focus:** Build the interactive chat interface — the main user-facing feature.

**Components to Build:**
1. **Chat Engine** (`src/chat/engine.rs`)
   - Manage conversation state and flow
   - Handle multi-turn interactions
   - Integrate context from buffer
   - Route messages to providers

2. **Message History** (`src/chat/history.rs`)
   - Store conversation messages
   - Retrieve history for context
   - Truncate long conversations
   - Persist across sessions

3. **Chat UI** (`src/ui/panel.rs`)
   - Zed panel for chat interface
   - Message display with streaming updates
   - User input handling
   - Error and status messages

4. **Context Integration** (`src/chat/context.rs`)
   - Extract file content
   - Capture cursor position
   - Get selected text
   - Format for inclusion in prompts

**Key Decisions:**
- How to structure conversation state
- Message history storage strategy (in-memory vs. persistent)
- UI framework/approach for Zed
- Context injection into prompts

**Tests to Add:**
- Multi-turn conversation flow tests
- Message history tests
- Context extraction tests
- Chat engine error handling tests
- UI interaction tests

**Deliverables:**
- Functional chat panel in Zed
- Real-time response streaming
- Multi-turn conversation support
- Conversation history and persistence
- Code context integration
- Full test coverage

### Phase 4: Code Completions & Advanced Features 📋 OPTIONAL

**Focus:** Add optional code completion feature and advanced chat features.

**Components to Build:**
1. **Completion Engine** (`src/completion/engine.rs`)
   - Trigger logic (keyboard shortcuts)
   - Context extraction for inline suggestions
   - Response formatting for display
   - Performance optimization

2. **Advanced Chat Features**
   - Custom system prompts
   - Code refactoring suggestions
   - Documentation generation
   - Test generation
   - Debugging assistance

**Key Decisions:**
- Completion trigger mechanism
- Inline display formatting
- Caching strategy for performance
- Feature toggles for advanced options

**Deliverables:**
- Optional code completion feature
- Advanced chat capabilities
- Performance optimization
- Full test coverage

## Error Handling Strategy

### Provider Errors

Providers return `ProviderError` with variants for different failure modes:

```rust
pub enum ProviderError {
    ApiError(String),      // API request/response failures
    ConfigError(String),   // Configuration issues
    NetworkError(String),  // Network connectivity
    ParseError(String),    // Response parsing failures
    NotAvailable(String),  // Provider unavailable
}
```

**Handling:**
- Log with context for debugging
- Return user-friendly error messages
- Implement fallback to secondary provider (future)

### Chat-Specific Error Handling

In Phase 3, chat engine should:
- Catch provider errors gracefully
- Display error messages to user
- Offer retry option
- Maintain conversation state on error
- Log detailed error context

### Configuration Errors

In Phase 2.2:
- Validate configuration at load time
- Provide specific error messages
- Suggest fixes for common issues
- Fail fast if critical configuration is missing

## Performance Considerations

### Streaming Optimization
- Buffer tokens efficiently to avoid UI thrashing
- Update UI at reasonable intervals (e.g., every 100ms)
- Don't block on API calls (use async/await)

### Memory Efficiency
- Limit conversation history size (configurable)
- Truncate old messages when limit exceeded
- Use references and borrowing effectively
- Profile with `cargo flamegraph` if issues arise

### API Efficiency
- Cache identical prompts (future)
- Batch requests if applicable
- Respect rate limiting
- Implement request debouncing

## Logging & Observability

### Log Format

All logs should include context:
```rust
println!("[Zed Copilot] Component: Message here");
println!("[Chat] Sent message: {} tokens", token_count);
println!("[Provider:openai] API response received: {} chars", len);
```

### Key Logging Points

- Extension initialization
- Configuration loading
- Provider instantiation
- API requests/responses (non-sensitive)
- Error conditions with context
- Chat events (user message, assistant response)

### Debugging

Enable detailed logging by running Zed in foreground:
```bash
zed --foreground
```

View logs with:
```
zed: open log
```

## Security Considerations

### Credential Handling
- Never hardcode API keys
- Use environment variable interpolation
- Use Zed's secure storage APIs when available
- Validate credentials before use
- Clear credentials from memory appropriately

### Data Privacy
- Don't log sensitive data (API keys, user code)
- Implement user code sanitization if logging
- Respect user privacy in error messages
- Consider data retention for conversation history

### Input Validation
- Validate all user input
- Sanitize prompts before sending to API
- Handle injection attacks in prompts
- Validate API responses before processing

## Useful Resources

- [ROADMAP.md](./ROADMAP.md) — Project roadmap and timelines
- [TESTING.md](./TESTING.md) — Comprehensive testing guide
- [PROVIDER_INTEGRATION.md](./PROVIDER_INTEGRATION.md) — Provider details
- [DOCUMENTATION_REVIEW.md](./DOCUMENTATION_REVIEW.md) — Alignment review
- [zed-rules/AGENTS.md](../zed-rules/AGENTS.md) — Coding guidelines
- [Zed Extension API Docs](https://zed.dev/docs/extensions)
- [Zed Extension Capabilities](https://zed.dev/docs/extensions/capabilities.html)
- [Rust WebAssembly Book](https://rustwasm.github.io/book/)
- [Tokio Async Tutorial](https://tokio.rs/tokio/tutorial)
- [OpenAI API Docs](https://platform.openai.com/docs)
- [Anthropic Claude Docs](https://docs.anthropic.com/)

## License

This project is MIT licensed. See LICENSE file for details.

---

**Last Updated:** November 2024
**Current Version:** 0.0.1 (Phase 1 Complete ✅)
**Current Phase:** 2.2 (Configuration & Credentials)
**Primary Feature:** Chat Interface (Phase 3 - Coming Q2 2025)
**Status:** Provider foundation complete, configuration system in progress
**Maintainers:** Zed Copilot Contributors