# GitHub Copilot LSP Integration

Technical strategy for GitHub Copilot LSP integration with Zed Copilot.

> **Part of:** [Zed Copilot Documentation](../README.md)

## Overview

GitHub Copilot via LSP (Language Server Protocol) is a **must-have integration** for Zed Copilot. This document outlines the strategy for integrating GitHub Copilot's LSP server to provide inline code completion and intelligence features alongside Zed Copilot's chat interface.

## Why GitHub Copilot LSP?

### Strategic Benefits

1. **Industry Standard** — GitHub Copilot is the most widely adopted AI coding assistant
2. **LSP Protocol** — Language Server Protocol provides a standard, decoupled integration mechanism
3. **Complementary to Chat** — Code completions enhance the chat experience without duplicating effort
4. **Zed IDE Precedent** — Zed IDE itself integrates GitHub Copilot via LSP, proving the pattern works
5. **User Expectations** — Users expect both chat and inline completions from a modern AI assistant

### How It Complements Zed Copilot

| Feature | Zed Copilot Chat | GitHub Copilot LSP | Combined Experience |
|---------|------------------|-------------------|---------------------|
| **Chat** | ✅ Multi-turn conversations | ❌ N/A | Ask questions, get context |
| **Inline Completions** | ❌ N/A | ✅ Real-time suggestions | Autocomplete as you type |
| **Code Context** | ✅ Reference code in chat | ✅ Understand file context | Seamless code-aware workflow |
| **Code Refactoring** | ✅ Chat-based assistance | ✅ Quick fixes | Dialog + inline suggestions |
| **Documentation** | ✅ Chat generates docs | ✅ Suggests inline docs | Comprehensive documentation |

## Architecture

### Integration Pattern

GitHub Copilot LSP integration follows Zed IDE's proven approach:

```
┌─────────────────────────────────────────────────┐
│           Zed IDE (Host)                        │
├─────────────────────────────────────────────────┤
│  Zed Copilot Extension (WebAssembly)            │
│  ┌──────────────────────────────────────────┐   │
│  │ ZedCopilot (Extension)                   │   │
│  ├── Chat Engine (Phase 3) ⏳                 │   │
│  ├── Chat UI Panel (Phase 3) ⏳               │   │
│  ├── LSP Client Adapter (Phase 4) ⏳          │   │
│  │   └── GitHub Copilot LSP Bridge          │   │
│  └──────────────────────────────────────────┘   │
│                    ↕ LSP                        │
│  GitHub Copilot LSP Server (External Binary)    │
│  ├── Code Completion Engine                     │
│  ├── Index Management                           │
│  └── Token Billing Integration                  │
└─────────────────────────────────────────────────┘
```

### LSP Lifecycle Management

Zed Copilot will manage the GitHub Copilot LSP server:

1. **Server Installation** — Detect or download GitHub Copilot LSP binary
2. **Process Management** — Start/stop LSP server process as needed
3. **Request Routing** — Forward completion requests from Zed to GitHub Copilot LSP
4. **Response Handling** — Parse LSP responses and format for Zed UI
5. **Error Recovery** — Handle server crashes and reconnection gracefully

### LSP Methods to Support

**Phase 4.1 (Initial):**
- `textDocument/completion` — Inline code suggestions
- `textDocument/completionItem/resolve` — Completion item details
- `initialize` — Server handshake
- `shutdown` / `exit` — Graceful cleanup

**Phase 4.2 (Extended):**
- `textDocument/signatureHelp` — Function signature hints
- `textDocument/hover` — Hover documentation
- `codeAction/resolve` — Code actions and quick fixes
- `textDocument/inlineCompletion` — Inline completion (newer Copilot versions)

## Implementation Plan

### Phase 4: Code Completions via LSP (PLANNED)

**Timeline:** Q3 2025+

**Phase 4.1: Core LSP Integration**
- LSP client adapter for managing GitHub Copilot LSP server
- Server installation/discovery logic
- Basic completion request/response handling
- Configuration for Copilot authentication token
- Error handling and server recovery

**Phase 4.2: Advanced Features**
- Signature help and hover information
- Code actions and quick fixes
- Inline completion support
- Completion filtering and ranking
- Performance optimization

**Phase 4.3: UX Polish**
- Completion ranking and presentation
- Debouncing and request cancellation
- Caching of recent completions
- User preference configuration

### Dependencies

Before implementing LSP integration, Zed Copilot must complete:

✅ **Phase 2.1** — AI Provider Abstraction (Complete)
✅ **Phase 2.2** — Configuration & Credentials (Complete)
⏳ **Phase 2.3** — HTTP Integration & Streaming (Required)
⏳ **Phase 3** — Chat Interface (Recommended, not blocking)

### Key Requirements

1. **Authentication**
   - GitHub Copilot requires valid GitHub credentials
   - Support GitHub CLI authentication (`gh auth login`)
   - Support device flow authentication
   - Handle token refresh

2. **Server Discovery**
   - Detect GitHub Copilot LSP installation
   - Support multiple Copilot versions
   - Provide installation instructions if missing
   - Fallback to bundled version if available

3. **Configuration**
   - LSP-specific settings in Zed config
   - Completion filtering options
   - Server timeout and retry settings
   - Telemetry opt-out configuration

4. **Performance**
   - Non-blocking completion requests
   - Request debouncing (avoid excessive requests)
   - Caching of redundant completions
   - Memory management for large completions

5. **Error Handling**
   - Graceful degradation if server unavailable
   - Automatic reconnection on server crash
   - User-friendly error messages
   - Telemetry for diagnostics

## Configuration

### Zed Settings

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "lsp": {
      "github_copilot": {
        "enabled": true,
        "auto_start": true,
        "server_path": null,
        "timeout_ms": 5000,
        "completion": {
          "enabled": true,
          "trigger_on_whitespace": false,
          "max_suggestions": 5,
          "debounce_ms": 100
        },
        "auth": {
          "source": "github_cli",
          "token_path": null
        }
      }
    }
  }
}
```

### Configuration Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | bool | `true` | Enable GitHub Copilot LSP integration |
| `auto_start` | bool | `true` | Automatically start LSP server on extension load |
| `server_path` | string | `null` | Path to GitHub Copilot LSP binary (auto-detect if null) |
| `timeout_ms` | number | `5000` | Request timeout in milliseconds |
| `completion.enabled` | bool | `true` | Enable code completions |
| `completion.trigger_on_whitespace` | bool | `false` | Show completions after whitespace |
| `completion.max_suggestions` | number | `5` | Maximum completions to show |
| `completion.debounce_ms` | number | `100` | Debounce completion requests |
| `auth.source` | string | `"github_cli"` | Authentication source ("github_cli" or "token") |
| `auth.token_path` | string | `null` | Path to GitHub token file (if not using CLI) |

## Authentication

### GitHub CLI Integration

The recommended authentication method:

```bash
# User runs once
gh auth login

# Zed Copilot automatically discovers and uses the token
```

Zed Copilot will:
1. Detect GitHub CLI installation
2. Read authentication token from `~/.config/gh/hosts.yml`
3. Pass token to LSP server

### Environment Variable Fallback

If GitHub CLI is unavailable:

```bash
export GITHUB_TOKEN="ghp_..."
```

### Device Flow Authentication

For headless systems or advanced setup:
1. LSP server initiates GitHub device flow
2. User visits github.com/login/device
3. Zed Copilot provides device code and instructions
4. Authentication completes automatically

## Security Considerations

### Token Management

- **No Token Storage** — Never cache tokens in config files
- **CLI Integration** — Use GitHub CLI for secure token handling
- **Environment Variables** — Support `GITHUB_TOKEN` for CI/CD systems
- **Token Refresh** — Handle automatic token refresh from GitHub CLI

### Privacy

- **Telemetry Opt-out** — Respect user privacy preferences
- **Code Privacy** — Code is sent to GitHub Copilot (user responsibility)
- **No Extra Tracking** — Don't add additional tracking beyond LSP server

### Permissions

- **Minimal Scope** — Only request necessary GitHub scopes
- **Clear Prompts** — Explain what authentication is needed
- **Easy Revocation** — Make it easy to disable/revoke access

## Testing Strategy

### Unit Tests

- LSP client instantiation and configuration
- Request/response serialization
- Error handling and recovery
- Token discovery from GitHub CLI

### Integration Tests

- End-to-end completion request/response
- Server startup and shutdown
- Authentication flow
- Network failure recovery

### Manual Testing

1. **Setup**
   ```bash
   gh auth login
   cargo build --release
   # Install as dev extension in Zed
   ```

2. **Test Completion**
   - Open a file in Zed
   - Type code and verify completions appear
   - Check that suggestions are relevant
   - Verify performance (no UI freezing)

3. **Test Error Cases**
   - Disable GitHub CLI authentication → Should show error
   - Kill LSP server process → Should reconnect
   - Timeout on slow network → Should gracefully fail
   - Invalid token → Should prompt re-authentication

## Rollout Strategy

### Phase 4.1 Release

- **Target Users** — Developers who want inline completions
- **Feature Flag** — Default disabled, opt-in via config
- **Documentation** — Setup guide and troubleshooting
- **Feedback Channel** — Encourage issue reports

### Phase 4.2 Release

- **Stability Focus** — Fix Phase 4.1 issues
- **Performance Tuning** — Optimize completion latency
- **UX Improvements** — Better ranking and presentation
- **Advanced Features** — Signature help, code actions

### Graduation to Default

After Phase 4.2 stabilization:
- Enable by default for new users
- Provide easy disable option
- Continue supporting GitHub CLI as primary auth

## Related Zed IDE Implementation

Zed IDE integrates GitHub Copilot LSP with:

1. **LSP Client Manager** — Manages LSP server process lifecycle
2. **Request Router** — Routes completion requests to GitHub Copilot LSP
3. **UI Integration** — Renders completions in editor
4. **Authentication** — GitHub CLI or device flow
5. **Configuration** — Per-project LSP settings

Zed Copilot will follow this pattern, adapted for Zed's extension API constraints.

## Implementation Checklist

- [ ] **Phase 4.1: Core LSP**
  - [ ] LSP client wrapper around GitHub Copilot LSP binary
  - [ ] Server discovery and launch logic
  - [ ] GitHub CLI token detection
  - [ ] Completion request/response handling
  - [ ] Basic error handling and logging
  - [ ] Configuration schema and validation
  - [ ] Unit and integration tests (80%+ coverage)
  - [ ] Documentation and troubleshooting guide

- [ ] **Phase 4.2: Advanced Features**
  - [ ] Signature help support
  - [ ] Hover information support
  - [ ] Code actions support
  - [ ] Inline completion (newer Copilot versions)
  - [ ] Completion ranking and filtering
  - [ ] Performance optimizations
  - [ ] Extended test coverage

- [ ] **Phase 4.3: UX Polish**
  - [ ] Completion debouncing
  - [ ] Request cancellation on model changes
  - [ ] Caching of recent completions
  - [ ] User preference configuration
  - [ ] Telemetry integration

## FAQ

**Q: Will this replace OpenAI/Anthropic providers for chat?**
A: No. GitHub Copilot LSP provides inline completions, while OpenAI/Anthropic power the chat interface. They work together.

**Q: Do I need a GitHub Copilot subscription?**
A: Yes, GitHub Copilot requires an active subscription or free trial. This is a GitHub limitation, not specific to Zed Copilot.

**Q: Why not build our own completion engine?**
A: GitHub Copilot has years of training data and optimization. Using it via LSP leverages proven technology while we focus on chat UX.

**Q: Can I use Copilot without GitHub CLI?**
A: Yes, we'll support environment variable auth (`GITHUB_TOKEN`) and device flow, but GitHub CLI is recommended.

**Q: What if GitHub Copilot LSP server crashes?**
A: Zed Copilot will automatically restart it and notify the user. Completions will resume without manual intervention.

**Q: How does this affect Zed IDE's own Copilot integration?**
A: No conflict. Zed Copilot and Zed's native Copilot support can coexist. Users choose which they prefer.

## Related Documentation

- [DEVELOPMENT.md](./DEVELOPMENT.md) — Architecture overview
- [ROADMAP.md](./ROADMAP.md) — Full project timeline
- [PROVIDER_INTEGRATION.md](./PROVIDER_INTEGRATION.md) — AI provider details
- [CONFIG.md](./CONFIG.md) — Configuration documentation

## Future Considerations

- **Other Code Intelligence** — Linters, formatters, type information
- **Multi-LSP Support** — Support other LSP servers (Pylance, Rust Analyzer, etc.)
- **Custom Models** — Fine-tuned models via GitHub Copilot for Business
- **Performance Metrics** — Collect aggregate performance data
- **IDE Integration** — Deeper Zed IDE integration in future versions

---

**Phase:** 4 (Planned, Q3 2025+)  
**Status:** Design & Planning  
**Priority:** Must-Have Feature  
**Complexity:** High (external process management, LSP protocol)  
