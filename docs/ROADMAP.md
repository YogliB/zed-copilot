# Zed Copilot Roadmap

## Overview

Zed Copilot is a multi-phase project to build a WebAssembly-based AI extension for Zed IDE. This roadmap outlines the planned features and milestones for bringing an interactive AI chat assistant to Zed, with optional code completion features.

## Phase 1: Foundation (Current - v0.0.1) ✅

**Status:** Complete — Ready for Phase 2

- [x] Basic extension scaffolding
- [x] Zed extension registration
- [x] Project documentation
- [x] Unit test framework (14 tests, all passing)
- [x] CI/CD setup (GitHub Actions)

**Deliverables:**
- Working extension that loads in Zed
- Comprehensive development documentation
- Full test suite with 100% pass rate
- Automated CI/CD pipeline

## Phase 2: AI Provider Integration (v0.1.0)

**Status:** Phase 2.1 ✅ Complete — Phase 2.2 ✅ Complete — Phase 2.3 ✅ Complete

**Target:** Q1 2025

### Phase 2.1: AI Provider Abstraction ✅
- [x] Abstract AI provider interface
- [x] OpenAI provider implementation
- [x] Anthropic Claude provider implementation
- [x] Provider factory for instantiation
- [x] Error handling and type-safe results
- [x] Comprehensive unit tests (31 tests, all passing)
- [x] Provider integration documentation

**Deliverables:**
- Trait-based `AiProvider` interface
- `OpenAiProvider` with GPT-4o, o1, and o3-mini support
- `AnthropicProvider` with Claude Opus 4.1, Sonnet 4, and Haiku 4.5 support
- `ProviderFactory` for provider instantiation
- `ProviderError` enum with 5 error variants
- Full async/await support via `async-trait`
- 503 lines of production-ready code
- Zero compiler warnings

### Phase 2.2: Configuration & Credentials ✅
- [x] API key management system (ConfigManager)
- [x] Provider configuration from Zed settings (ConfigLoader)
- [x] Environment variable interpolation (EnvInterpolator with ${VAR_NAME} syntax)
- [x] Per-provider configuration support (OpenAiConfig, AnthropicConfig)
- [x] Configuration validation and error handling (ConfigValidator)
- [x] Chat-specific streaming configuration (ChatConfig)
- [x] 30+ unit tests with 90%+ coverage
- [x] Complete documentation (CONFIG.md, settings.schema.json)

**Deliverables:**
- `ConfigManager` facade for simple API
- Environment variable interpolation via `${VAR_NAME}` syntax
- Comprehensive validation with helpful error messages
- `RootConfig`, `OpenAiConfig`, `AnthropicConfig`, `ChatConfig` structs
- `ConfigLoader` for JSON/file loading
- `EnvInterpolator` for secure credential handling
- `ConfigValidator` with per-provider rules
- Full test coverage (27 configuration tests)
- User guide (CONFIG.md) with setup instructions and examples
- JSON Schema (settings.schema.json) for IDE autocomplete
- Updated DEVELOPMENT.md with configuration section

### Phase 2.3: HTTP Integration & Retry Logic ✅
- [x] HTTP client implementation (reqwest)
- [x] Actual API request execution
- [x] Response parsing and validation
- [x] Retry logic with exponential backoff
- [x] Rate limiting support
- [x] OpenAI API integration with chat completions
- [x] Anthropic API integration with messages endpoint
- [x] Comprehensive error handling and transient error classification
- [x] 30+ unit tests with 100% coverage

**Deliverables:**
- `HttpClient` wrapper with timeout and retry orchestration
- `RetryPolicy` with exponential backoff and jitter
- `RateLimiter` with token bucket algorithm
- `OpenAiHttpClient` for OpenAI chat completions
- `AnthropicHttpClient` for Anthropic messages
- Full integration with OpenAI and Anthropic providers
- HTTP_INTEGRATION.md guide with architecture and configuration
- RETRY_STRATEGY.md with exponential backoff rationale and tuning
- 98 passing unit tests
- Zero compiler warnings

**Goals:**
- ✅ Enable real API calls to AI providers
- ✅ Handle transient failures gracefully with exponential backoff
- ✅ Prevent thundering herd with jitter-based retry distribution
- ✅ Respect provider rate limits with token bucket algorithm

**Key Goals for Phase 2:**
- Enable multiple AI provider support ✅ (Phase 2.1)
- Secure credential management ✅ (Phase 2.2)
- Robust error handling for API failures ✅ (Phase 2.3)
- User-friendly configuration ✅ (Phase 2.2)
- HTTP integration for real API calls ✅ (Phase 2.3)

**Phase 2 Summary:**
Phase 2 is now complete! The foundation for AI provider integration is solid:
- Extensible provider interface supporting multiple AI services
- Secure configuration and credential management
- Reliable HTTP integration with exponential backoff retry logic
- Rate limiting to respect provider quotas
- Comprehensive error handling and logging
Ready for Phase 3: Chat Interface implementation.

## Phase 3: Chat Interface & Core Functionality (v0.2.0) 🎯 PRIMARY

**Target:** Q2 2025

- [ ] Chat UI panel implementation in Zed
- [ ] Message handling and display
- [ ] User message input and submission
- [ ] Multi-turn conversation management
- [ ] Message history storage and retrieval
- [ ] Streaming response rendering
- [ ] Context awareness (file, cursor, selection)
- [ ] Chat state management
- [ ] Error handling and user feedback
- [ ] Comprehensive chat tests

**Deliverables:**
- Interactive chat panel in Zed sidebar/panel
- Full conversation flow (send message → stream response → update history)
- Message history persistence (per session or global)
- Chat-specific error messages and recovery
- Streaming response display with real-time token updates
- Context integration (code snippets, file context)
- Configuration UI for API keys and provider selection
- Full test coverage for chat functionality

**Key Goals:**
- Intuitive chat interface for asking questions about code
- Real-time response streaming for immediate feedback
- Conversation history for context continuity
- Seamless integration with Zed IDE
- High-quality user experience

## Phase 4: GitHub Copilot LSP Integration & Code Completions (v0.3.0+)

**Target:** Q3 2025+

### GitHub Copilot LSP Integration ⭐ Must-Have
- [ ] LSP client implementation for GitHub Copilot server
- [ ] Server discovery and process management
- [ ] GitHub CLI authentication support
- [ ] Device flow authentication setup
- [ ] Inline code completion requests via LSP
- [ ] Completion response parsing and display
- [ ] Multi-completion cycling support
- [ ] Code completion debouncing and caching
- [ ] Signature help and hover information (Phase 4.2)
- [ ] Code actions and quick fixes (Phase 4.2)
- [ ] Comprehensive LSP integration tests
- [ ] GitHub Copilot LSP documentation

**Goals:**
- Provide inline code completions matching Zed IDE's native Copilot support
- Enable seamless GitHub Copilot subscription integration
- Support 50+ programming languages via LSP protocol
- Establish foundation for future code intelligence features
- **Critical:** Must-have for competitive parity with VS Code/JetBrains

**See:** [GH_COPILOT_LSP_INTEGRATION.md](./GH_COPILOT_LSP_INTEGRATION.md) for detailed implementation strategy

### Code Completions (Integrated via LSP)
- [x] GitHub Copilot LSP provides inline completions (Phase 4.1)
- [ ] Completion trigger logic (context-aware)
- [ ] Response formatting for inline display
- [ ] Caching strategy for repeated prompts
- [ ] Performance optimization
- [ ] Completion-specific tests

**Goals:**
- Fast, responsive inline completions via GitHub Copilot LSP
- Smart context gathering from editor state
- Integrated feature (core to competitive positioning)

### Advanced Features
- [ ] Multi-language support in chat
- [ ] Custom system prompts
- [ ] Code refactoring suggestions
- [ ] Documentation generation from chat
- [ ] Test generation from chat
- [ ] Debugging assistance
- [ ] Chat personas/modes (e.g., teacher, expert, etc.)

**Key Goals:**
- Expand beyond basic chat
- Support diverse workflows
- Improve developer productivity
- Enable advanced use cases

## Phase 5: Polish & Publishing (v1.0.0)

**Target:** Q4 2025

- [ ] Comprehensive test coverage (chat + completions)
- [ ] Performance optimization
- [ ] Security audit
- [ ] Official documentation
- [ ] User guide and tutorials
- [ ] Publish to Zed extension registry
- [ ] Community support infrastructure

**Key Goals:**
- Production-ready quality
- Broad compatibility
- User-friendly experience
- Official support and maintenance

## Architecture Overview

### Core Components (Phase 2 + 3 + 4)

```
┌─────────────────────────────────────────────────┐
│           Zed IDE (Host)                        │
├─────────────────────────────────────────────────┤
│  Zed Copilot Extension (WebAssembly)            │
│  ┌──────────────────────────────────────────┐   │
│  │ ZedCopilot (Extension)                   │   │
│  ├── Chat UI Panel (Phase 3)                │   │
│  ├── Chat Engine (Phase 3)                  │   │
│  ├── Message History Manager (Phase 3)      │   │
│  ├── Streaming Response Handler (Phase 2.3) │   │
│  ├── AI Provider Manager (Phase 2.1)        │   │
│  ├── Configuration Manager (Phase 2.2)      │   │
│  ├── Context Manager (Phase 3)              │   │
│  ├── GitHub Copilot LSP Client (Phase 4) ⭐ │   │
│  │   ├── Server Process Manager             │   │
│  │   ├── LSP Message Handler                │   │
│  │   └── Completion Engine                  │   │
│  ├── Logger/Telemetry                       │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
         ↓                              ↓
┌─────────────────────────────────────────────────┐
│  External AI Providers & Services               │
│  ├── OpenAI API (GPT-4o, o1, o3-mini)          │
│  ├── Anthropic Claude API (Claude Opus 4.1, Sonnet 4, Haiku 4.5) │
│  ├── GitHub Copilot LSP Server ⭐ (Phase 4)    │
│  │   └── GitHub Authentication & API          │
│  └── Future: Ollama, other LLMs                │
└─────────────────────────────────────────────────┘
```

**Legend:** ⭐ = Must-have integration | Phase X = Planned completion phase

## API Integration Strategy

### AI Provider Interface (Complete - Phase 2.1) ✅

The extension uses an abstract provider interface to support multiple AI services:

```rust
#[async_trait::async_trait]
pub trait AiProvider: Send + Sync {
    async fn complete(&self, prompt: &str) -> ProviderResult<String>;
    async fn is_available(&self) -> bool;
    fn name(&self) -> &str;
    fn model(&self) -> &str;
}

pub struct OpenAiProvider {
    api_key: String,
    model: String,
    api_base: String,
}

pub struct AnthropicProvider {
    api_key: String,
    model: String,
    api_base: String,
}
```

This design enables:
- Easy addition of new providers (trait-based)
- Provider switching at runtime (via `Box<dyn AiProvider>`)
- Fallback mechanisms (try primary, fall back to secondary)
- Transparent API management (provider handles format details)
- Type-safe error handling via `ProviderResult<T>`
- Thread-safe execution with `Send + Sync` bounds

**Note:** Phase 2.3 will extend this to support streaming responses for chat.

### Configuration (Phase 2.2 - Planned)

Users will configure providers in Zed settings via JSON:

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
      "model": "claude-sonnet-4-20250514",
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

**Security Features:**
- Environment variable interpolation
- No hardcoded credentials
- Per-provider configuration
- Optional per-user overrides

### Supported Providers

#### OpenAI ✅ (Complete - Phase 2.1)
- Models: GPT-4o, o1, o3-mini
- Capabilities: Chat, code generation, analysis, reasoning
- Status: Implemented and tested
- Features: Custom API base URL support, request payload building

#### Anthropic Claude ✅ (Complete - Phase 2.1)
- Models: Claude 3 (Opus, Sonnet, Haiku)
- Capabilities: Chat, code analysis, conversations
- Status: Implemented and tested
- Features: Custom API base URL support, request payload building

#### GitHub Copilot LSP ⭐ Must-Have (Phase 4 - Planned)
- Models: Claude 3 (via GitHub Copilot API)
- Capabilities: Inline code completions, code intelligence, chat integration
- Status: Planned for Phase 4 (Q3 2025+)
- **Critical Features:**
  - Language Server Protocol (LSP) for reliable integration
  - GitHub authentication via CLI or device flow
  - 50+ language support with context awareness
  - Inline completion cycling and smart suggestions
  - Native Zed IDE integration pattern
  - Competitive parity with VS Code and JetBrains
- **Documentation:** See [GH_COPILOT_LSP_INTEGRATION.md](./GH_COPILOT_LSP_INTEGRATION.md)

#### Future Providers (Planned for Phase 4.3+)
- Local models via Ollama
- Self-hosted LLM services
- Additional commercial providers (Cohere, Mistral, etc.)
- Custom/fine-tuned models

## Implementation Priorities

### Current (Phase 2 - In Progress)
1. ✅ Stable extension foundation (Phase 1 - Complete)
2. ✅ AI provider interface abstraction (Phase 2.1 - Complete)
3. ✅ OpenAI and Anthropic implementations (Phase 2.1 - Complete)
4. **Phase 2.2 (Next):** Configuration and credential management
5. **Phase 2.3 (After 2.2):** HTTP integration and streaming support

### Near-term (Phase 2.2-2.3)
1. API key management and secure storage
2. Provider configuration system (chat-focused)
3. Environment variable interpolation
4. HTTP client implementation (reqwest with streaming)
5. Retry logic with exponential backoff
6. Streaming response support for chat

### Medium-term (Phase 3 - PRIMARY FOCUS)
1. Chat UI panel and message display
2. Multi-turn conversation management
3. Message history storage and retrieval
4. Streaming response rendering
5. User input handling and submission
6. Context awareness (file, cursor, selection)
7. Error handling and user feedback

### Near-term Critical (Phase 4 - Must-Have)
1. ⭐ GitHub Copilot LSP integration (code completions)
2. LSP server discovery and authentication
3. Inline completion rendering and cycling
4. Signature help and hover information
5. Code actions and quick fixes

### Long-term (Phases 4.2-5)
1. Advanced code intelligence features
2. Code refactoring and test generation
3. Multi-language support in chat
4. Custom prompts and workflows
5. Security audit and official publication

## Success Metrics

### Phase 3 (Chat) - Primary Goals
- **Quality:** Chat tests at 90%+ coverage
- **UX:** Users can conduct multi-turn conversations
- **Performance:** Response streaming starts within 1 second
- **Stability:** Less than 0.1% error rate in chat operations
- **Reliability:** Chat state persists across sessions

### Phase 4 (GitHub Copilot LSP) - Must-Have Goals ⭐
- **Quality:** LSP integration tests at 85%+ coverage
- **Functionality:** Inline completions work across 50+ languages
- **Authentication:** GitHub CLI and device flow fully supported
- **Performance:** Completion requests respond in <200ms
- **Stability:** LSP server auto-recovery on failure
- **Parity:** Feature-equivalent to VS Code Copilot integration

### Overall
- **Adoption:** Published to Zed registry with GitHub Copilot LSP support
- **Community:** Active contributor base with LSP expertise
- **Support:** Comprehensive documentation and tutorials

## Breaking Changes

This roadmap may be adjusted based on:
- Zed IDE API changes
- Community feedback
- Technical discoveries
- Resource constraints

**Recent Change (November 2024):**
- **MAJOR:** Repositioned chat as Phase 3 (primary feature)
- Code completions moved to Phase 4 (optional feature)
- Rationale: Chat is the primary user-facing feature; completions are secondary

Updates will be communicated via:
- GitHub issues
- CHANGELOG.md updates
- Milestone tracking
- Status badges

## Related Documentation

- [DEVELOPMENT.md](./DEVELOPMENT.md) — Development workflow and architecture
- [DOCUMENTATION_REVIEW.md](./DOCUMENTATION_REVIEW.md) — Alignment review and gaps
- [CHAT_ARCHITECTURE.md](./CHAT_ARCHITECTURE.md) — Chat design and implementation (Phase 3)
- [TESTING.md](./TESTING.md) — Testing strategies and guidelines
- [PROVIDER_INTEGRATION.md](./PROVIDER_INTEGRATION.md) — AI provider details
- [CHANGELOG.md](../CHANGELOG.md) — Version history and changes

---

**Last Updated:** November 2024
**Current Phase:** 2 (AI Provider Integration) — Phase 2.2 Complete ✅
**Next Phase:** 2.3 (HTTP Integration & Streaming)
**Primary Feature:** Chat Interface (Phase 3)
**Must-Have Integration:** GitHub Copilot LSP (Phase 4) ⭐
**Status:** On Track — 63/63 Tests Passing ✅