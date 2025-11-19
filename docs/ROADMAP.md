# Zed Copilot Roadmap

## Overview

Zed Copilot is a multi-phase project to build a WebAssembly-based AI extension for Zed IDE. This roadmap outlines the planned features and milestones for bringing intelligent code completion and AI-assisted features to Zed.

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

### Phase 2.3: HTTP Integration & Retry Logic
- [ ] HTTP client implementation (reqwest)
- [ ] Actual API request execution
- [ ] Response parsing and validation
- [ ] Retry logic with exponential backoff
- [ ] Rate limiting support

**Key Goals:**
- Enable multiple AI provider support ✅ (Phase 2.1)
- Secure credential management (Phase 2.2)
- Robust error handling for API failures (Phase 2.3)
- User-friendly configuration (Phase 2.2)

## Phase 3: Code Completion (v0.2.0)

**Target:** Q2 2025

- [ ] Completion trigger logic
- [ ] Context extraction from buffer
- [ ] Response formatting
- [ ] Caching strategy
- [ ] Performance optimization

**Key Goals:**
- Fast, responsive completions
- Smart context gathering
- High-quality suggestions
- Memory-efficient operation

## Phase 4: Advanced Features (v0.3.0+)

**Target:** Q3 2025+

- [ ] Multi-language support
- [ ] Custom prompts
- [ ] Code refactoring suggestions
- [ ] Documentation generation
- [ ] Test generation
- [ ] Debugging assistance

**Key Goals:**
- Expand beyond simple completions
- Support diverse workflows
- Improve developer productivity
- Enable advanced use cases

## Phase 5: Polish & Publishing (v1.0.0)

**Target:** Q4 2025

- [ ] Comprehensive test coverage
- [ ] Performance optimization
- [ ] Security audit
- [ ] Official documentation
- [ ] Publish to Zed extension registry

**Key Goals:**
- Production-ready quality
- Broad compatibility
- User-friendly experience
- Official support

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

**Implementation Status:**
- ✅ Trait fully defined with 4 methods
- ✅ OpenAI provider with GPT-4 and GPT-3.5-turbo support
- ✅ Anthropic provider with Claude 3 family support
- ✅ Custom API base URL support for both providers
- ✅ Request payload building (ready for HTTP implementation)

### Configuration (Planned)

Users will configure providers in Zed settings via JSON:

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4"
    },
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-3-sonnet"
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
- Capabilities: Text completion, code generation
- Status: Implemented and tested
- Features: Custom API base URL support, request payload building

#### Anthropic Claude ✅ (Complete - Phase 2.1)
- Models: Claude 3 (Opus, Sonnet, Haiku)
- Capabilities: Text completion, code analysis
- Status: Implemented and tested
- Features: Custom API base URL support, request payload building

#### Future Providers (Planned)
- Local models via Ollama (Phase 2.4+)
- Self-hosted LLM services (Phase 2.4+)
- Additional commercial providers (Cohere, Mistral, etc.)
- Streaming response support (Phase 4+)

## Implementation Priorities

### Current (Phase 2 - In Progress)
1. ✅ Stable extension foundation (Phase 1 - Complete)
2. ✅ AI provider interface abstraction (Phase 2.1 - Complete)
3. ✅ OpenAI and Anthropic implementations (Phase 2.1 - Complete)
4. **Phase 2.2 (Next):** Configuration and credential management
5. **Phase 2.3 (After 2.2):** HTTP integration and retry logic

### Near-term (Phase 2.2-2.3)
1. API key management and secure storage
2. Provider configuration system
3. Environment variable interpolation
4. HTTP client implementation (reqwest)
5. Retry logic with exponential backoff

### Medium-term (Phase 3)
1. Completion trigger logic and context extraction
2. Response formatting and caching
3. Performance optimization and benchmarking
4. Memory efficiency improvements

### Long-term (Phases 4-5)
1. Advanced features (refactoring, test generation, docs)
2. Multi-language support
3. Custom prompts and workflows
4. Security audit and official publication

## Success Metrics

- **Quality:** All tests passing, zero warnings
- **Performance:** Completions under 2 seconds
- **Stability:** Less than 0.1% error rate
- **Adoption:** Published to Zed registry
- **Community:** Active contributor base

## Breaking Changes

This roadmap may be adjusted based on:
- Zed IDE API changes
- Community feedback
- Technical discoveries
- Resource constraints

Updates will be communicated via:
- GitHub issues
- CHANGELOG.md updates
- Milestone tracking
- Status badges

## Related Documentation

- [DEVELOPMENT.md](./DEVELOPMENT.md) — Development workflow and architecture
- [TESTING.md](./TESTING.md) — Testing strategies and guidelines
- [CHANGELOG.md](../CHANGELOG.md) — Version history and changes

---

**Last Updated:** November 20, 2024
**Current Phase:** 2 (AI Provider Integration) — Phase 2.1 Complete
**Next Phase:** 2.2 (Configuration & Credentials)
**Status:** On Track — 40/40 Tests Passing ✅