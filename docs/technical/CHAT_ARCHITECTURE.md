# Chat Architecture

> **Status:** Phase 3 (Planned)  
> **Target Completion:** Q2 2025  
> **See Also:** [ROADMAP.md](ROADMAP.md#phase-3-chat-interface--core-functionality) | [DEVELOPMENT.md](DEVELOPMENT.md#phase-3-chat-interface-primary-feature)

## Overview

This document describes the architecture and design of Zed Copilot's chat interface, which is the primary feature of the extension.

Chat enables users to ask questions about their code, get explanations, debugging help, and more—all within Zed IDE through an interactive conversation interface.

## Chat Components

> **Implementation:** Phase 3 (coming Q2 2025)

### Chat Engine

[TODO: Add after Phase 3 design]

Responsibilities:
- Manage conversation state
- Route user messages to providers
- Handle streaming responses
- Maintain message history
- Extract code context
- Format prompts with context

### Message History

[TODO: Add after Phase 3 design]

Responsibilities:
- Store conversation messages with metadata
- Retrieve messages for multi-turn context
- Truncate history when limit exceeded
- Persist across editor sessions
- Support session-based or global history

### Streaming Response Handler

[TODO: Add after Phase 3 design]

Responsibilities:
- Receive streaming tokens from providers
- Buffer and emit updates to UI
- Handle incomplete/malformed chunks
- Display real-time responses

### Chat UI Panel

[TODO: Add after Phase 3 design]

Responsibilities:
- Display messages in conversation view
- Handle user input and submission
- Show typing/loading indicators
- Render formatted responses
- Support syntax highlighting for code

### Context Manager

[TODO: Add after Phase 3 design]

Responsibilities:
- Extract current file content
- Capture cursor position
- Get selected text
- Format context for inclusion in prompts
- Handle large files/selections

## Message Flow

> **Implementation:** Phase 3 (coming Q2 2025)

Typical chat interaction flow:

```
User Input
  ↓
Chat Engine (validate)
  ↓
Context Extraction (file, cursor, selection)
  ↓
Prompt Formatting (include context)
  ↓
AI Provider (stream response)
  ↓
Streaming Handler (buffer tokens)
  ↓
Chat UI (display in real-time)
  ↓
Message History (persist)
```

## Message Format

> **Implementation:** Phase 3 (coming Q2 2025)

### Message Struct

[TODO: Add after Phase 3 design]

```rust
pub struct Message {
    pub role: MessageRole,      // User or Assistant
    pub content: String,        // Message text
    pub timestamp: u64,         // When sent
    pub metadata: MessageMetadata,
}

pub enum MessageRole {
    User,
    Assistant,
    System,  // For system prompts
}

pub struct MessageMetadata {
    pub id: String,
    pub model: String,          // Which model generated this
    pub token_count: usize,
    pub error: Option<String>,  // If generation failed
}
```

### System Prompt

[TODO: Add after Phase 3 design]

System prompt structure for guiding AI behavior:
- Start with role/purpose
- Include code context guidelines
- Set tone and response format
- Specify what information to include

## Context Integration

> **Implementation:** Phase 3 (coming Q2 2025)

### Code Context

[TODO: Add after Phase 3 design]

Types of context that can be included:
- Current file name and language
- Current file content (or relevant portions)
- Selected text (primary context)
- Cursor line/column
- Project structure (if available)

### Context Limits

[TODO: Add after Phase 3 design]

Handling large contexts:
- Maximum context size per message
- Truncation strategy (recent lines preferred)
- Snippet extraction for large files
- Selection priority over full file

### Context Formatting

[TODO: Add after Phase 3 design]

How context is formatted in prompts:
- File header with language/name
- Code block with syntax highlighting markers
- Position indicators (line numbers)
- Selection highlighting

## History Management

> **Implementation:** Phase 3 (coming Q2 2025)

### Storage Strategy

[TODO: Add after Phase 3 design]

Options being considered:
- In-memory (fastest, lost on restart)
- JSON file (persistent, simple)
- SQLite (persistent, queryable)
- Zed's native storage (if available)

### Persistence

[TODO: Add after Phase 3 design]

When and how to save:
- Auto-save after each message
- Configurable history size limit
- Cleanup old conversations
- Session management

### Session vs Global

[TODO: Add after Phase 3 design]

History scope options:
- Per-session (separate for each editor session)
- Global (shared across sessions)
- Per-file (context-specific conversations)
- Manual organization (user-managed folders)

## Streaming Strategy

> **Implementation:** Phase 3 (coming Q2 2025)

### Response Streaming

[TODO: Add after Phase 3 design]

Streaming implementation details:
- Token buffering (aggregate small tokens)
- UI update frequency (e.g., every 100ms)
- Cursor/typing indicator during streaming
- Stop token detection

### Token Display

[TODO: Add after Phase 3 design]

How tokens are rendered:
- Progressive display (token by token)
- Markdown rendering support
- Code block syntax highlighting
- Link/reference formatting

### Real-time Updates

[TODO: Add after Phase 3 design]

UI responsiveness during streaming:
- Non-blocking updates (don't freeze UI)
- Scroll to latest message
- Show token count/rate
- Allow user to stop generation

## Error Handling

> **Implementation:** Phase 3 (coming Q2 2025)

### Recoverable Errors

[TODO: Add after Phase 3 design]

Errors the system should retry:
- Network timeouts
- Rate limiting (429 responses)
- Temporary provider unavailability
- Token limit exceeded (truncate context)

### Permanent Errors

[TODO: Add after Phase 3 design]

Errors requiring user intervention:
- Invalid API key
- Provider not configured
- Unsupported model
- Account/billing issues

### Error Display

[TODO: Add after Phase 3 design]

How errors are shown to users:
- Clear error message
- Suggested fixes
- Retry button for retryable errors
- Log location for debugging

## Configuration

> **Implementation:** Phase 2.2, used in Phase 3

Chat-specific configuration in settings.json:

[TODO: Add after Phase 2.2 implementation]

```json
{
  "zed_copilot": {
    "chat": {
      "streaming_enabled": true,
      "max_history_messages": 50,
      "auto_scroll_to_latest": true,
      "include_file_context": true,
      "context_limit_tokens": 2000
    }
  }
}
```

## Testing Strategy

> **Implementation:** Phase 3 (coming Q2 2025)

See [TESTING.md](TESTING.md#phase-3-chat-interface-primary-feature) for detailed test plan.

Key test areas:
- Chat engine flow (single-turn and multi-turn)
- Message history (FIFO, truncation, persistence)
- Streaming response parsing and display
- Context extraction and formatting
- Error handling and recovery
- UI interactions

## Performance Considerations

> **Implementation:** Phase 3 (coming Q2 2025)

### Streaming Performance

[TODO: Add after Phase 3 design]

- Token buffering to avoid excessive UI updates
- Efficient history storage (don't load entire history every time)
- Lazy-load older messages if needed

### Memory Usage

[TODO: Add after Phase 3 design]

- Limit conversation history in memory
- Compress old messages if stored locally
- Clear temporary buffers after streaming completes

### UI Responsiveness

[TODO: Add after Phase 3 design]

- Don't block on provider calls (use async/await)
- Update UI progressively during streaming
- Smooth scrolling/rendering

## Security Considerations

> **Implementation:** Phase 3 (coming Q2 2025)

### Data Privacy

[TODO: Add after Phase 3 design]

- Don't log sensitive data (API keys, user code details)
- Consider user privacy in stored conversations
- Allow chat history deletion

### Input Validation

[TODO: Add after Phase 3 design]

- Validate user input before sending to provider
- Sanitize displayed responses
- Prevent prompt injection attacks

## Related Documentation

- [DEVELOPMENT.md](DEVELOPMENT.md#phase-3-chat-interface-primary-feature) — Architecture overview
- [ROADMAP.md](ROADMAP.md#phase-3-chat-interface--core-functionality) — Phase 3 timeline
- [TESTING.md](TESTING.md#phase-3-chat-interface-primary-feature) — Chat test strategy
- [PROVIDER_INTEGRATION.md](PROVIDER_INTEGRATION.md#chat-implementation-phase-3---planned) — Provider API for chat
- [CONFIGURATION.md](CONFIGURATION.md) — Chat configuration options

---

**Last Updated:** November 2024  
**Phase:** 3 (Chat Interface - Primary Feature)  
**Status:** Stub (awaiting design and implementation)