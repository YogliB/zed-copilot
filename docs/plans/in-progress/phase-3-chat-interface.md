# Phase 3 Masterplan: Chat Interface & Core Functionality (v0.2.0)

**Overview:** Build a complete interactive chat UI panel in Zed with message history, streaming responses, and context awareness. This is the primary deliverable for v0.2.0 and enables real-time conversation with AI providers.

**Approach:** Implement in 5 sequential PRs focusing on infrastructure → UI → streaming → state management → testing & polish. Each PR is independently testable with mocked providers.

**Est. Time:** 80–120h (dev + review) = ~3–4 weeks

**PRs:** 5 across 1 repo

**Risk:** **Medium** — UI complexity in new framework, streaming integration, state management. Mitigation: Research Zed API early (PR-1), prototype streaming (PR-3), comprehensive test coverage (PR-5).

**Repos:** `zed-copilot`

---

## Implementation Status

| PR  | Status | Title | ETA |
| --- | ------ | ----- | --- |
| 1   | ⏸️     | Chat Panel Scaffold & Context Infrastructure | 12–16h |
| 2   | ⏸️     | Message Handling, Display & History Storage | 16–20h |
| 3   | ⏸️     | Streaming Response Rendering | 12–16h |
| 4   | ⏸️     | Chat State Management & Configuration UI | 16–20h |
| 5   | ⏸️     | Comprehensive Testing & Documentation | 16–20h |

Status: 🟢 done · 🟡 in‑progress · 🟠 review · ⏸️ not‑started · 🔴 blocked · ⚫ canceled

---

## PR 1: Chat Panel Scaffold & Context Infrastructure — ⏸️

**Repo:** `zed-copilot` · **Link:** [-] · **ETA:** 12–16h dev + 2–3h review

**Files:** `src/ui/chat_panel.rs`, `src/chat/context.rs`, `src/lib.rs`

**Changes:**

1. **Research & Document Zed UI Panel API** — Create `docs/development/ZED_PANEL_API.md`
    - Study Zed extension API examples
    - Document panel lifecycle, rendering, event handling
    - Add code snippets for panel creation

2. **Implement Chat Panel Scaffold** — File: `src/ui/chat_panel.rs`
    - Create `ChatPanel` struct with lifecycle methods
    - Implement `open()`, `close()`, `is_visible()`
    - Add message list view component
    - Add input field and submit button
    - Wire panel to workspace context
    - Basic styling with Zed UI primitives

3. **Implement Context Provider Trait** — File: `src/chat/context.rs`
    - Define `ContextProvider` trait:
      ```rust
      pub trait ContextProvider: Send + Sync {
        async fn get_file_context(&self) -> Result<Option<FileContext>>;
        async fn get_selection_context(&self) -> Result<Option<SelectionContext>>;
        async fn get_cursor_context(&self) -> Result<Option<CursorContext>>;
      }
      ```
    - Implement `ZedContextProvider` for editor integration
    - Add unit tests for context extraction

4. **Integrate Panel into Extension** — File: `src/lib.rs`
    - Register chat panel with Zed workspace
    - Add command to open/close panel
    - Wire context provider to panel
    - Test panel opens without crashing

**Acceptance:**

- [ ] Chat panel opens and closes without errors
- [ ] Panel lifecycle (open → focus → blur → close) works
- [ ] Context provider extracts file, selection, cursor correctly
- [ ] All new functions have unit tests
- [ ] Documentation for Zed panel API is complete
- [ ] No panics or unwraps in new code
- [ ] Code follows project style guidelines
- [ ] Tests updated/added (integrated)
- [ ] No breaking changes

**Dependencies:** None · Blocks: PR-2, PR-4

---

## PR 2: Message Handling, Display & History Storage — ⏸️

**Repo:** `zed-copilot` · **Link:** [-] · **ETA:** 16–20h dev + 2–3h review

**Files:** `src/chat/message.rs`, `src/chat/history.rs`, `src/ui/message_display.rs`

**Changes:**

1. **Define Message & Conversation Data Structures** — File: `src/chat/message.rs`
    - Create `Message` struct with fields: id, role, content, timestamp, metadata
    - Implement `Serialize`/`Deserialize` for persistence
    - Add builder pattern for easy construction
    - Add unit tests for serialization

2. **Implement Conversation History Storage** — File: `src/chat/history.rs`
    - Create `ConversationStore` trait with methods: save_message, load_messages, delete_conversation, list_conversations
    - Implement `JsonFileStore` (writes to `~/.cache/zed-copilot/chat/`)
    - Add migration logic for future schema changes
    - Implement async file I/O with proper error handling

3. **Implement Message Display Component** — File: `src/ui/message_display.rs`
    - Create `MessageView` component with user/assistant styling
    - Add `MessageListView` for scrollable history
    - Implement auto-scroll to latest message
    - Add smooth animations for new messages

4. **Wire Message Flow in Chat Panel** — File: `src/ui/chat_panel.rs`
    - Add `on_message_input()` handler
    - Implement `on_submit()` to capture user message
    - Wire message to `ChatManager` (from PR-4)
    - Display message in list immediately (optimistic)
    - Handle submission validation (non-empty, no spam)

5. **Integrate History Loading on Startup** — File: `src/lib.rs`
    - Load last conversation or start fresh
    - Display history on panel open
    - Persist conversation ID
    - Test history is preserved across restarts

**Acceptance:**

- [ ] Messages can be created, serialized, and deserialized
- [ ] Message history is persisted to disk
- [ ] Messages display correctly with user/assistant styling
- [ ] Message list scrolls and auto-scrolls
- [ ] User input is captured and validated
- [ ] History loads on extension startup
- [ ] >85% test coverage for message/history code
- [ ] No data loss on crash/restart
- [ ] Tests updated/added (integrated)
- [ ] No breaking changes

**Dependencies:** Blocked by PR-1 · Blocks: PR-3, PR-4

---

## PR 3: Streaming Response Rendering — ⏸️

**Repo:** `zed-copilot` · **Link:** [-] · **ETA:** 12–16h dev + 2–3h review

**Files:** `src/chat/streaming.rs`, `src/ui/streaming_display.rs`, `src/http/streaming.rs` (extend Phase 2)

**Changes:**

1. **Extend HTTP Streaming from Phase 2** — File: `src/http/streaming.rs`
    - Enhance `StreamingResponse` to expose token-by-token iterator
    - Add `on_chunk` callback for real-time processing
    - Implement `AsyncStreamReader` for buffering and partial tokens
    - Add timeout handling for stalled streams

2. **Implement Chat Streaming Orchestrator** — File: `src/chat/streaming.rs`
    - Create `StreamingChat` wrapper with request, provider, on_token callback, cancellation_token
    - Buffer tokens for debouncing UI updates
    - Implement cancellation support (via button click)
    - Collect final message for history storage
    - Add error handling for network failures

3. **Implement Streaming UI Component** — File: `src/ui/streaming_display.rs`
    - Create `StreamingMessageView` with token rendering
    - Show "typing" indicator while streaming
    - Display token count and estimated response time
    - Add cancel button
    - Implement debounced rendering (batch updates every 50ms)

4. **Integrate Streaming into Chat Panel** — File: `src/ui/chat_panel.rs`
    - On message submit, call `StreamingChat::execute()`
    - Pass `on_token` callback to update UI
    - Display streaming message in list
    - Disable input during streaming
    - Add cancel button to stop mid-response
    - After streaming completes, save to history

5. **Test with Large Responses** — 
    - Mock provider returning 10K+ token response
    - Verify UI remains responsive
    - Profile memory usage during streaming
    - Test cancellation at various stages

**Acceptance:**

- [ ] Streaming responses render token-by-token
- [ ] UI updates are debounced and responsive (<16ms frame time)
- [ ] Streaming can be cancelled mid-response
- [ ] Partial responses are handled gracefully
- [ ] Network errors show user-friendly messages
- [ ] Final response is correctly saved to history
- [ ] >85% test coverage for streaming code
- [ ] Performance: <100ms latency per token batch
- [ ] Tests updated/added (integrated)
- [ ] No breaking changes

**Dependencies:** Blocked by PR-2 · Blocks: PR-5

---

## PR 4: Chat State Management & Configuration UI — ⏸️

**Repo:** `zed-copilot` · **Link:** [-] · **ETA:** 16–20h dev + 2–3h review

**Files:** `src/chat/manager.rs`, `src/config/chat_config.rs`, `src/ui/config_panel.rs`

**Changes:**

1. **Implement Chat State Machine** — File: `src/chat/manager.rs`
    - Create `ChatManager` with explicit state enum: Idle, Typing, Streaming, Error
    - Implement state transitions with validation
    - Add lifecycle hooks: `on_open()`, `on_close()`, `on_context_change()`
    - Implement request debouncing (prevent spam)
    - Add logging for state changes

2. **Extend Configuration System from Phase 2** — File: `src/config/chat_config.rs`
    - Add `ChatConfig` section with streaming_enabled, max_history_messages, auto_scroll_to_latest, include_context_by_default
    - Validate config on load
    - Support hot-reload of settings
    - Add type-safe config accessors

3. **Implement Configuration UI Panel** — File: `src/ui/config_panel.rs`
    - Create settings UI with provider selection, API key input, streaming toggle, history settings
    - Add validation and error messages
    - Persist changes immediately
    - Add "Test Connection" button

4. **Integrate Configuration into Chat Panel** — File: `src/ui/chat_panel.rs`
    - Load provider/config on panel open
    - Handle provider switching mid-conversation
    - Show provider info in panel header
    - Apply context preferences
    - Respect streaming settings

5. **Add Error Handling & Recovery** — File: `src/chat/manager.rs`
    - Define `ChatError` enum with variants: ProviderUnavailable, InvalidConfiguration, NetworkError, Timeout, InvalidResponse
    - Implement retry logic with exponential backoff
    - Add user-friendly error messages
    - Log errors with full context for debugging

**Acceptance:**

- [ ] Chat state machine transitions correctly
- [ ] Configuration can be set and persisted
- [ ] Provider switching works without data loss
- [ ] Configuration UI is intuitive and validated
- [ ] All errors show helpful messages
- [ ] Retry logic works with backoff
- [ ] >85% test coverage for state/config code
- [ ] No data races or mutex deadlocks
- [ ] Tests updated/added (integrated)
- [ ] No breaking changes

**Dependencies:** Blocked by PR-1 (context), PR-2 (history) · Can overlap with PR-3

---

## PR 5: Comprehensive Testing & Documentation — ⏸️

**Repo:** `zed-copilot` · **Link:** [-] · **ETA:** 16–20h dev + 2–3h review

**Files:** `tests/chat_e2e.rs`, `tests/streaming_mocks.rs`, `docs/development/CHAT_ARCHITECTURE.md`, etc.

**Changes:**

1. **Add Unit Tests** (if not already in PRs 1-4)
    - Test `Message` serialization/deserialization
    - Test `ConversationStore` with mock filesystem
    - Test `ChatManager` state transitions
    - Test `ContextProvider` with mock editor
    - Target: >90% coverage

2. **Add Integration Tests** — File: `tests/chat_e2e.rs`
    - Mock AI provider with deterministic responses
    - Test full message flow: user input → provider → response → storage
    - Test streaming with chunked responses
    - Test history retrieval and sorting
    - Test configuration changes during chat
    - Test error handling (provider down, network error, timeout)

3. **Add Streaming Mock Provider** — File: `tests/streaming_mocks.rs`
    - Implement `MockStreamingProvider` with configurable token delay
    - Inject errors at specific token counts
    - Simulate various response sizes
    - Use for performance testing and edge cases

4. **Add Performance & Profiling Tests**
    - Measure input latency (<100ms)
    - Measure first-token latency (<500ms)
    - Measure memory during streaming (1000+ token response)
    - Test with large conversation history (1000+ messages)

5. **Write Comprehensive Documentation** 
    - **`docs/development/CHAT_ARCHITECTURE.md`**: System design, component overview, data flow diagrams
    - **`docs/development/CHAT_STATE_MACHINE.md`**: State transitions, error handling
    - **`docs/user/CHAT_USAGE.md`**: User guide (how to use, tips, troubleshooting)
    - **`docs/development/TESTING_GUIDE.md`**: How to test chat code, mock patterns
    - Update **`docs/development/ARCHITECTURE.md`**: Add chat subsystem

6. **Update CHANGELOG & Release Notes**
    - Summarize Phase 3 features
    - List breaking changes (none expected)
    - Document known limitations
    - Add upgrade instructions

7. **Code Review & Refactoring**
    - Review all PRs for code clarity
    - Remove any commented-out code
    - Ensure consistent error handling
    - Optimize hot paths (if identified)
    - Add clarifying comments for complex logic (per AGENTS.md guidelines)

8. **Final Integration Testing**
    - Run all tests together
    - Test interaction between all components
    - Verify no memory leaks
    - Stress test with rapid chat submissions
    - Test with all supported providers

**Acceptance:**

- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] >90% code coverage for chat module
- [ ] Performance benchmarks meet targets
- [ ] Architecture documentation is complete and clear
- [ ] User guide is helpful and accurate
- [ ] No test flakiness (run 10x)
- [ ] All code follows AGENTS.md guidelines
- [ ] No security issues (API keys, injections, etc.)
- [ ] Tests updated/added (integrated)
- [ ] No breaking changes

**Dependencies:** Blocked by PR-1, PR-2, PR-3, PR-4

---

## Risk Mitigation

**UI Framework Complexity:**
- **Concern**: Zed UI panel API may be poorly documented or have learning curve.
- **Analysis**: PR-1 includes research phase; worst case we prototype with simple UI.
- **Mitigation**: Start with minimal panel, add complexity incrementally. Study Zed examples early.
- **Recovery**: Fall back to CLI-based chat if UI proves infeasible.

**Streaming Integration & Performance:**
- **Concern**: Token-by-token rendering could cause UI lag or memory issues with large responses.
- **Analysis**: Need to profile and test with realistic data.
- **Mitigation**: Implement debouncing (batch updates), test with 10K+ token responses in PR-3, profile memory.
- **Recovery**: Revert to non-streaming (wait for full response) if performance unacceptable. Users still get chat, just less responsive.

**State Management Complexity:**
- **Concern**: Chat state with pending requests, history, provider switching could cause subtle bugs.
- **Analysis**: Explicit state machine (PR-4) reduces risk.
- **Mitigation**: Comprehensive logging, state transition tests, avoid mutable shared state.
- **Recovery**: Simplify to stateless request/response if state machine becomes too complex. Implement multi-turn as future feature.

**Context Injection Bugs:**
- **Concern**: Stale or incorrect context (wrong file, outdated selection) breaks response quality.
- **Analysis**: Need thorough testing of context extraction.
- **Mitigation**: Unit tests for each context type, validation before injection, user toggle to disable context.
- **Recovery**: Disable context by default, release as opt-in feature.

**Provider Switching Mid-Conversation:**
- **Concern**: Switching providers could break state or lose conversation.
- **Analysis**: Requires careful state management.
- **Mitigation**: Validate provider availability before operations, clear state gracefully on switch, warn user.
- **Recovery**: Lock provider selection during active requests, prevent switching until conversation completes.

**Data Persistence & Corruption:**
- **Concern**: File-based message storage could corrupt on crash or concurrent writes.
- **Analysis**: Zed is single-process per workspace, but async code needs care.
- **Mitigation**: Use atomic writes, implement file locking, add integrity checks on load.
- **Recovery**: Clear corrupted history file and start fresh. Implement automated backup.

---

## Deployment Strategy

**CRITICAL:** Phase 3 is v0.2.0 release. Roll out all 5 PRs sequentially (dependencies shown above), then release as single version bump.

### Stage 1: Infrastructure & Core (PRs 1-2)

1. **Merge PR-1** (Chat Panel Scaffold & Context Infrastructure)
    - Verify: Panel opens without panic, context extraction works
    - Rollback: Revert commit, remove chat panel from extension registration

2. **Merge PR-2** (Message Handling & History Storage)
    - Verify: Messages persist to disk, can be loaded on restart
    - Rollback: Revert commit, remove history code (in-memory only)

### Stage 2: Streaming & State (PRs 3-4)

3. **Merge PR-3** (Streaming Response Rendering)
    - Verify: Streaming responses appear token-by-token, no UI lag
    - Rollback: Revert to non-streaming (blocking requests)

4. **Merge PR-4** (State Management & Configuration)
    - Verify: State machine transitions work, config can be changed
    - Rollback: Revert to simple request/response

### Stage 3: Testing & Release (PR-5)

5. **Merge PR-5** (Testing & Documentation)
    - Verify: All tests pass, coverage >90%, docs are accurate
    - Rollback: Not applicable (no behavioral changes)

6. **Create Release v0.2.0**
    - Tag commit with all 5 PRs merged
    - Write release notes
    - Publish to Zed marketplace

### Cross-Repo Version Map

| Stage | PR | zed-copilot | Notes |
| ----: | -- | ----------- | ----- |
| 1 | 1 | v0.2.0-alpha.1 | Chat panel scaffold, context provider |
| 1 | 2 | v0.2.0-alpha.2 | Message & history storage |
| 2 | 3 | v0.2.0-beta.1 | Streaming rendering |
| 2 | 4 | v0.2.0-beta.2 | State mgmt & config |
| 3 | 5 | v0.2.0 | Testing & docs |

---

## Monitoring & Observability

**Metrics to track:**
- `chat.messages_sent_total` — counter, per provider
- `chat.response_time_ms` — histogram, per provider
- `chat.streaming_first_token_latency_ms` — histogram
- `chat.streaming_tokens_per_second` — gauge
- `chat.errors_total` — counter, by error type
- `chat.history_size_messages` — gauge, per conversation
- `chat.context_inclusion_rate` — gauge (% of messages with context)

**Logs to capture:**
- **Success**: `msg="chat_message_sent" provider=openai model=gpt4 tokens_used=150 context_included=true`
- **Success**: `msg="streaming_complete" duration_ms=2500 tokens=350 first_token_ms=450`
- **Error**: `msg="provider_error" provider=openai code=401 error="Invalid API key"`
- **Error**: `msg="streaming_error" error="connection_reset" tokens_received=50 partial_content=true`

**Alarms to set:**
- Error rate > 5% in 5min window → page on-call
- Response time p95 > 10s → investigate provider
- Streaming first-token > 2s → investigate network/provider

---

## Rollback

### Quick Rollback (Feature Flags)
If issues found after release:
1. Add `[zed_copilot.chat] enabled = false` config option
2. Disable chat panel in UI
3. Users can opt-out without reinstalling
4. Issue fix in patch release (v0.2.1)

### Full Rollback Strategy

**If PR-1 breaks panel registration:**
- Revert PR-1 commit
- Remove chat panel from `src/lib.rs`
- Re-release as v0.1.1 hotfix

**If PR-2 causes data corruption:**
- Revert PR-2 commit
- Restore from backup (manual file restore from `~/.cache/zed-copilot/`)
- Disable history persistence in v0.1.1 hotfix

**If PR-3 streaming causes crashes:**
- Revert PR-3 commit
- Revert to blocking requests (wait for full response)
- Fix streaming in v0.2.1 and re-release

**If PR-4 state issues cause lost data:**
- Revert PR-4 commit
- Simplify state machine in subsequent version
- Release as v0.1.2 hotfix

**Rollback order** (reverse of rollout):
1. PR-5 (testing) — not applicable
2. PR-4 (state/config) — disable config changes, use defaults
3. PR-3 (streaming) — disable streaming, use blocking requests
4. PR-2 (history) — lose history, start fresh
5. PR-1 (panel) — disable chat panel entirely

**Artifacts safe to keep** after rollback:
- Extension configuration (doesn't change with chat PRs)
- AI provider implementations (unchanged)
- HTTP client (unchanged)
- User's API keys in settings (don't delete on rollback)

---

## Success Criteria

**Functional:**
- [ ] Chat panel opens, closes, and maintains state
- [ ] User can send message, receive response, and see history
- [ ] Streaming responses render token-by-token with <100ms latency
- [ ] Context (file, selection) is correctly extracted and included
- [ ] Configuration UI allows setting API keys and provider
- [ ] Message history persists across sessions

**Quality:**
- [ ] >90% test coverage for chat module
- [ ] All 5 PRs merged and passing CI
- [ ] Zero test flakiness (run each test 10x)
- [ ] No panics or unwraps in new code

**Performance:**
- [ ] Input latency < 100ms
- [ ] First token latency < 500ms
- [ ] Streaming throughput: 10+ tokens/sec
- [ ] Memory usage stable during 10K+ token response
- [ ] No memory leaks detected

**User Experience:**
- [ ] Chat is intuitive to use without documentation
- [ ] Error messages are helpful and actionable
- [ ] UI remains responsive during streaming
- [ ] Configuration is easy to change

**Documentation:**
- [ ] Architecture guide is complete and clear
- [ ] User guide covers common workflows
- [ ] Testing guide helps future contributors
- [ ] API documentation is accurate

**Production Readiness:**
- [ ] 0 known critical bugs
- [ ] Monitoring metrics are captured
- [ ] Rollback plan is documented and tested
- [ ] Release notes are clear and accurate

---

## References

- [ROADMAP.md](../ROADMAP.md) — Phase 3 section (v0.2.0)
- [AGENTS.md](../../zed-rules/AGENTS.md) — Code style and quality guidelines
- Zed Extension API — https://zed.dev/docs/extensions (research in PR-1)
- Phase 2 Implementation — `src/providers/`, `src/http/` (base for streaming)

---

## Notes & Assumptions

**Implementation Decisions:**
- ✅ Message persistence: JSON file storage (simple, no DB dependency)
- ✅ Streaming debouncing: 50ms batches (balance responsiveness vs. performance)
- ✅ State machine: Explicit enum (prevents invalid state combinations)
- ✅ Error recovery: Retry with backoff (user-friendly, no manual intervention)
- ❓ Context limit: 4K tokens or unlimited? (Decide in PR-1)

**Cross-Repo Coordination:**
- ✅ Single repo only (`zed-copilot`)
- ✅ No breaking changes to Phase 2 API

**Data Model:**
- ✅ `Message`: UUID + role + content + metadata
- ✅ `Conversation`: list of messages + metadata (provider, model, created_at)
- ✅ Persistence: `~/.cache/zed-copilot/chat/{conversation_id}.json`

**Testing:**
- ✅ All tests integrated with PRs (no standalone test PR)
- ✅ Mock providers for deterministic testing
- ✅ Performance tests in PR-5 (after all code merged)

**Assumptions to Verify:**
- ✅ Zed extension API is stable and documented
- ✅ File I/O in async context is safe (no blocking on main thread)
- ✅ Phase 2 HTTP streaming is production-ready
- ❌ Multi-turn conversations will work with all providers (verify in testing)
- ❌ Context injection won't exceed token limits (needs validation)

---

**Status:** Ready for review. Next step: Begin PR-1 after approval.