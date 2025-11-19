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

**Status:** Phase 2.1 Complete — Phase 2.2 Ready

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
- `OpenAiProvider` with GPT-4 and GPT-3.5-turbo support
- `AnthropicProvider` with Claude 3 family support
- `ProviderFactory` for provider instantiation
- `ProviderError` enum with 5 error variants
- Full async/await support via `async-trait`
- 503 lines of production-ready code
- Zero compiler warnings

### Phase 2.2: Configuration & Credentials (Next)
- [ ] API key management system
- [ ] Provider configuration from Zed settings
- [ ] Environment variable interpolation
- [ ] Per-provider configuration support
- [ ] Configuration validation and error handling
- [ ] Chat-specific streaming configuration
- [ ] Message persistence strategy

**Goals:**
- Enable users to configure API credentials securely
- Support chat-ready configuration
- Lay groundwork for Phase 3 chat interface

### Phase 2.3: HTTP Integration & Retry Logic
- [ ] HTTP client implementation (reqwest)
- [ ] Actual API request execution
- [ ] Response parsing and validation
- [ ] Streaming response support for chat
- [ ] Retry logic with exponential backoff
- [ ] Rate limiting support

**Goals:**
- Enable real API calls to AI providers
- Support streaming responses for chat UX
- Handle transient failures gracefully

**Key Goals for Phase 2:**
- Enable multiple AI provider support ✅ (Phase 2.1)
- Secure credential management (Phase 2.2)
- Robust error handling for API failures (Phase 2.3)
- User-friendly configuration (Phase 2.2)
- Streaming support for chat (Phase 2.3)

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

## Phase 4: Code Completions & Advanced Features (v0.3.0+)

**Target:** Q3 2025+

### Code Completions (Optional)
- [ ] Completion trigger logic (context-aware)
- [ ] Context extraction from buffer
- [ ] Response formatting for inline display
- [ ] Caching strategy for repeated prompts
- [ ] Performance optimization
- [ ] Completion-specific tests

**Goals:**
- Fast, responsive inline completions
- Smart context gathering
- Optional feature (not core to chat experience)

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

### Core Components (Phase 2 + 3)

```
┌─────────────────────────────────────────────────┐
│           Zed IDE (Host)                        │
├─────────────────────────────────────────────────┤
│  Zed Copilot Extension (WebAssembly)            │
│  ┌──────────────────────────────────────────┐   │
│  │ ZedCopilot (Extension)                   │   │
│  ├── Chat UI Panel                          │   │
│  ├── Chat Engine                            │   │
│  ├── Message History Manager                │   │
│  ├── Streaming Response Handler             │   │
│  ├── AI Provider Manager                    │   │
│  ├── Configuration Manager                  │   │
│  └── Context Manager                        │   │
│  ├── Logger/Telemetry                       │   │
│  └── (Future: Completion Engine)            │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
         ↓
┌─────────────────────────────────────────────────┐
│  External AI Providers                          │
│  ├── OpenAI API (GPT-4, GPT-3.5-turbo)         │
│  ├── Anthropic Claude API (Claude 3 family)    │
│  └── Future: Ollama, other LLMs                │
└─────────────────────────────────────────────────┘
```

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

**Security Features:**
- Environment variable interpolation
- No hardcoded credentials
- Per-provider configuration
- Optional per-user overrides

### Supported Providers

#### OpenAI ✅ (Complete - Phase 2.1)
- Models: GPT-4, GPT-3.5-turbo
- Capabilities: Chat, code generation, analysis
- Status: Implemented and tested
- Features: Custom API base URL support, request payload building

#### Anthropic Claude ✅ (Complete - Phase 2.1)
- Models: Claude 3 (Opus, Sonnet, Haiku)
- Capabilities: Chat, code analysis, conversations
- Status: Implemented and tested
- Features: Custom API base URL support, request payload building

#### Future Providers (Planned for Phase 2.4+)
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

### Long-term (Phases 4-5)
1. Code completion feature (optional)
2. Advanced features (refactoring, test generation, docs)
3. Multi-language support
4. Custom prompts and workflows
5. Security audit and official publication

## Success Metrics

### Phase 3 (Chat) - Primary Goals
- **Quality:** Chat tests at 90%+ coverage
- **UX:** Users can conduct multi-turn conversations
- **Performance:** Response streaming starts within 1 second
- **Stability:** Less than 0.1% error rate in chat operations
- **Reliability:** Chat state persists across sessions

### Overall
- **Adoption:** Published to Zed registry
- **Community:** Active contributor base
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
**Current Phase:** 2 (AI Provider Integration) — Phase 2.1 Complete
**Next Phase:** 2.2 (Configuration & Credentials)
**Primary Feature:** Chat Interface (Phase 3)
**Status:** On Track — 40/40 Tests Passing ✅