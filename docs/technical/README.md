# Technical Documentation

Deep technical dives into Zed Copilot's implementation details and architecture.

> **Part of:** [Zed Copilot Documentation](../../README.md#-documentation)

---

## 📖 Documents in This Folder

### [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md)
**Architecture decisions and trade-offs** — Key technical decisions made during development.

Best for: Understanding why the project is structured as a native extension vs WASM.

**Contains:**
- Native cdylib vs WebAssembly decision
- HTTP dependency requirements
- WASM validation skip rationale
- Trade-offs and alternatives considered
- Future considerations

### [PROVIDER_INTEGRATION.md](PROVIDER_INTEGRATION.md)
**AI provider implementation** — How Zed Copilot interfaces with AI providers.

Best for: Understanding the provider abstraction and implementation details.

**Contains:**
- Core trait-based provider interface
- OpenAI provider implementation
- Anthropic Claude provider implementation
- Provider factory pattern
- Error handling strategies
- Request/response formats
- Adding new providers guide
- Streaming support (Phase 2.3)
- Chat implementation (Phase 3 planned)

### [GH_COPILOT_LSP_INTEGRATION.md](GH_COPILOT_LSP_INTEGRATION.md)
**GitHub Copilot LSP integration** — Strategy for inline code completions.

Best for: Understanding the GitHub Copilot LSP integration plan.

**Contains:**
- Strategic benefits of LSP integration
- Architecture and lifecycle management
- LSP methods to support
- Implementation plan (Phase 4)
- UX considerations
- Request routing strategy

### [HTTP_INTEGRATION.md](HTTP_INTEGRATION.md)
**HTTP client and streaming** — Real API calls to providers.

Best for: Understanding HTTP request/response handling and streaming.

**Contains:**
- HTTP client architecture
- Request formatting for each provider
- Streaming response handling (SSE)
- Retry logic with exponential backoff
- Network error handling
- Rate limiting enforcement
- Request/response examples

### [CHAT_ARCHITECTURE.md](CHAT_ARCHITECTURE.md)
**Chat system design** — Architecture for the chat interface.

Best for: Understanding the chat engine design (Phase 3).

**Contains:**
- Chat message structure
- Conversation management
- History storage and retrieval
- Streaming display logic
- Error recovery
- Context integration

### [RETRY_STRATEGY.md](RETRY_STRATEGY.md)
**Retry logic and resilience** — How Zed Copilot handles failures.

Best for: Understanding error handling and retry mechanisms.

**Contains:**
- Retry strategy rationale
- Exponential backoff implementation
- Max retry configuration
- Specific error handling
- Rate limit handling
- Timeout handling

---

## 🎯 When to Read What

### I want to understand why this is a native extension (not WASM)
→ Read [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md)

### I want to understand the provider system
→ Read [PROVIDER_INTEGRATION.md](PROVIDER_INTEGRATION.md)

### I want to add a new AI provider
→ Read [PROVIDER_INTEGRATION.md](PROVIDER_INTEGRATION.md#creating-a-new-provider)

### I want to understand HTTP integration
→ Read [HTTP_INTEGRATION.md](HTTP_INTEGRATION.md)

### I want to understand the chat system
→ Read [CHAT_ARCHITECTURE.md](CHAT_ARCHITECTURE.md)

### I want to understand error handling
→ Read [RETRY_STRATEGY.md](RETRY_STRATEGY.md)

### I want to understand GitHub Copilot LSP plans
→ Read [GH_COPILOT_LSP_INTEGRATION.md](GH_COPILOT_LSP_INTEGRATION.md)

---

## 📚 What to Read Next

- **Getting Started:** [../getting-started/QUICKSTART.md](../getting-started/QUICKSTART.md)
- **Development Guide:** [../development/DEVELOPMENT.md](../development/DEVELOPMENT.md)
- **Roadmap:** [../development/ROADMAP.md](../development/ROADMAP.md)
- **Documentation Index:** [../../README.md#-documentation](../../README.md#-documentation)

---

## 🔗 Document Relationships

```
ARCHITECTURE_DECISIONS.md ─→ DEVELOPMENT.md (native vs WASM)
                          
PROVIDER_INTEGRATION.md ──┐
                          ├─→ DEVELOPMENT.md (architecture)
                          │
HTTP_INTEGRATION.md ──────┤
                          │
GH_COPILOT_LSP_INTEGRATION.md
                          │
CHAT_ARCHITECTURE.md ─────┘
                          
RETRY_STRATEGY.md ────────→ HTTP_INTEGRATION.md
```

---

## 💡 Technical Deep Dives

These documents are for developers working on:
- Adding new AI providers
- Implementing streaming responses
- Building the chat interface
- Handling network resilience
- Integrating GitHub Copilot LSP
- Understanding system architecture

---

## 📊 System Overview

### Current Implementation (Phase 2.3 Complete ✅)

**Provider System:**
- Trait-based `AiProvider` abstraction
- OpenAI provider (GPT-4o, o1, o3-mini)
- Anthropic provider (Claude Opus, Sonnet, Haiku)
- Provider factory for instantiation

**HTTP Integration:**
- Real API calls to providers
- Streaming response support (SSE)
- Retry logic with exponential backoff
- Network error handling

**Configuration:**
- Settings loader from Zed
- Environment variable interpolation
- Per-provider configuration
- Validation before use

### Future Implementation (Phase 3+)

**Chat Interface:**
- Interactive chat panel
- Multi-turn conversation support
- Message history management
- Real-time streaming display
- Code context integration

**GitHub Copilot LSP:**
- LSP client adapter
- Code completion requests
- Inline suggestions
- Server lifecycle management

---

## ❓ FAQs

### How does Zed Copilot talk to AI providers?

See [PROVIDER_INTEGRATION.md](PROVIDER_INTEGRATION.md) for the architecture.

**Quick answer:** The `AiProvider` trait defines a standard interface. Implementations like `OpenAiProvider` and `AnthropicProvider` handle API-specific details.

### How are requests retried on failure?

See [RETRY_STRATEGY.md](RETRY_STRATEGY.md) for details.

**Quick answer:** Exponential backoff with configurable max retries. Specific handling for rate limits, timeouts, and transient errors.

### How does streaming work?

See [HTTP_INTEGRATION.md](HTTP_INTEGRATION.md) for details.

**Quick answer:** Server-Sent Events (SSE) streaming allows tokens to arrive in real-time instead of waiting for complete responses.

### Why is WASM validation skipped?

See [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md).

**Quick answer:** This is a native cdylib extension using HTTP dependencies (tokio, reqwest, async-openai, anthropic_rust) that are incompatible with WASM targets. WASM validation skip is intentional and correct.

### Can I add a custom AI provider?

Yes! See [PROVIDER_INTEGRATION.md](PROVIDER_INTEGRATION.md#creating-a-new-provider).

**Steps:** Create provider file → Implement `AiProvider` trait → Update factory → Add tests.

### What's the plan for GitHub Copilot?

See [GH_COPILOT_LSP_INTEGRATION.md](GH_COPILOT_LSP_INTEGRATION.md).

**Quick answer:** Phase 4 (Q3 2025+). Will integrate via LSP for inline code completions alongside the chat interface.

---

## 🚀 Related Reading

- **[ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md)** — Why native extension vs WASM
- **[../development/DEVELOPMENT.md](../development/DEVELOPMENT.md)** — System architecture overview
- **[../development/ROADMAP.md](../development/ROADMAP.md)** — Project phases and timeline
- **[PROVIDER_INTEGRATION.md](PROVIDER_INTEGRATION.md)** — Start here for implementation details

---

**Back to:** [Documentation Index](../../README.md#-documentation)