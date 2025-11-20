# Zed Copilot Documentation

> Complete guide to installing, configuring, and developing Zed Copilot

Welcome to the Zed Copilot documentation! This is your central hub for all documentation—whether you're installing for the first time, configuring for your use case, or contributing to the project.

**Back to:** [Project README](../README.md)

---

## 📂 Documentation Structure

```
docs/
├── getting-started/          # New users start here
│   ├── QUICKSTART.md        # 5-minute setup
│   ├── SETUP.md             # Detailed installation
│   └── EXAMPLES.md          # Configuration examples
│
├── reference/               # Configuration reference
│   └── CONFIG.md            # Schema and options
│
├── development/             # For contributors
│   ├── CONTRIBUTING.md      # How to contribute
│   ├── DEVELOPMENT.md       # Architecture guide
│   ├── TESTING.md           # Testing strategy
│   └── ROADMAP.md           # Feature timeline
│
├── technical/               # Deep technical details
│   ├── PROVIDER_INTEGRATION.md
│   ├── GH_COPILOT_LSP_INTEGRATION.md
│   ├── HTTP_INTEGRATION.md
│   ├── CHAT_ARCHITECTURE.md
│   └── RETRY_STRATEGY.md
│
└── CHANGELOG.md             # Version history
```

---

## 🚀 Getting Started

**New to Zed Copilot?** Start here:

### For End Users

#### [getting-started/QUICKSTART.md](getting-started/QUICKSTART.md)
**5-minute setup guide** — Get Zed Copilot running quickly.

- Step-by-step installation
- Basic configuration
- Verification steps
- Quick fixes for common issues

#### [getting-started/SETUP.md](getting-started/SETUP.md)
**Detailed installation guide** — Complete setup with troubleshooting.

- System requirements
- Platform-specific instructions (macOS, Linux, Windows)
- 11+ troubleshooting scenarios
- Advanced setup options
- Uninstallation guide

#### [getting-started/EXAMPLES.md](getting-started/EXAMPLES.md)
**Configuration cookbook** — 13+ ready-to-use configuration examples.

- OpenAI configurations (GPT-4o, o1, o3-mini)
- Anthropic configurations (Claude Opus, Sonnet, Haiku)
- Environment setup by OS
- Model comparison table
- Troubleshooting examples

**Recommended path:** QUICKSTART → SETUP (if issues) → EXAMPLES (for config)

---

## ⚙️ Configuration

Learn how to configure Zed Copilot for your needs:

### [getting-started/EXAMPLES.md](getting-started/EXAMPLES.md)
**Practical configuration examples** — Copy-paste ready configs.

Best for: Getting started quickly, trying different models.

### [reference/CONFIG.md](reference/CONFIG.md)
**Complete schema reference** — Detailed documentation of all options.

Best for: Understanding all available options, advanced customization.

**Quick comparison:**
- **EXAMPLES.md** = Cookbook (recipes you can use immediately)
- **CONFIG.md** = Manual (comprehensive reference)

---

## 👨‍💻 Development & Contributing

For contributors and developers:

### [development/CONTRIBUTING.md](development/CONTRIBUTING.md)
**How to contribute** — Process, guidelines, and best practices.

- Code of conduct
- Development workflow
- Pull request process
- Coding standards
- Testing requirements

### [development/DEVELOPMENT.md](development/DEVELOPMENT.md)
**Architecture guide** — Understanding the codebase.

- Project architecture
- Module organization
- Design patterns
- Extension lifecycle
- Development workflow
- Build and test commands

### [development/TESTING.md](development/TESTING.md)
**Testing strategy** — How we test Zed Copilot.

- Testing philosophy
- Test organization
- Unit tests
- Integration tests
- Mocking strategies
- Writing new tests

### [development/GIT_HOOKS.md](development/GIT_HOOKS.md)
**Git hooks & pre-push validation** — Prevent test failures locally.

- Pre-commit validation (format + lint)
- Pre-push smart testing (changed modules only)
- Installation and setup
- Troubleshooting guide
- Bypass procedures

### [development/ROADMAP.md](development/ROADMAP.md)
**Feature timeline** — What's built, what's next.

- Project phases
- Current status
- Timeline (Q2-Q3 2025)
- Feature priorities
- Future plans

**Recommended path:** CONTRIBUTING → DEVELOPMENT → TESTING → GIT_HOOKS → ROADMAP

---

## 🔧 Technical Deep Dives

Detailed technical documentation:

### [technical/PROVIDER_INTEGRATION.md](technical/PROVIDER_INTEGRATION.md)
**AI provider implementation** — How providers work internally.

- Provider abstraction design
- OpenAI integration details
- Anthropic integration details
- Error handling and retries
- Adding new providers

### [technical/GH_COPILOT_LSP_INTEGRATION.md](technical/GH_COPILOT_LSP_INTEGRATION.md)
**GitHub Copilot LSP integration** — Future inline completions.

- LSP protocol overview
- GitHub Copilot API
- Integration architecture
- Implementation plan (Phase 4, Q3 2025+)

### [technical/HTTP_INTEGRATION.md](technical/HTTP_INTEGRATION.md)
**HTTP client and streaming** — Network layer implementation.

- HTTP client design
- Streaming responses
- Connection management
- Error handling
- Performance considerations

### [technical/CHAT_ARCHITECTURE.md](technical/CHAT_ARCHITECTURE.md)
**Chat system design** — Future chat interface architecture.

- Chat UI design
- Message flow
- Context management
- State handling
- Implementation plan (Phase 3, Q2 2025)

### [technical/RETRY_STRATEGY.md](technical/RETRY_STRATEGY.md)
**Retry logic details** — How we handle failures.

- Retry policies
- Exponential backoff
- Rate limiting
- Circuit breakers
- Timeout handling

---

## 🎯 Documentation by Goal

### I want to install and use Zed Copilot

1. [getting-started/QUICKSTART.md](getting-started/QUICKSTART.md) — Quick setup
2. [getting-started/SETUP.md](getting-started/SETUP.md) — If you run into issues
3. [getting-started/EXAMPLES.md](getting-started/EXAMPLES.md) — Configuration

### I want to configure for my use case

- [getting-started/EXAMPLES.md](getting-started/EXAMPLES.md) — Ready-to-use configs
- [reference/CONFIG.md](reference/CONFIG.md) — Schema reference
- Model comparison in [getting-started/EXAMPLES.md](getting-started/EXAMPLES.md#comparison-when-to-use-what)

### I want to contribute to the project

1. [development/CONTRIBUTING.md](development/CONTRIBUTING.md) — Process
2. [development/DEVELOPMENT.md](development/DEVELOPMENT.md) — Architecture
3. [development/TESTING.md](development/TESTING.md) — Testing guidelines
4. [development/GIT_HOOKS.md](development/GIT_HOOKS.md) — Local validation setup
5. [development/ROADMAP.md](development/ROADMAP.md) — What's planned

### I want to understand the architecture

- [development/DEVELOPMENT.md](development/DEVELOPMENT.md) — Start here
- [technical/PROVIDER_INTEGRATION.md](technical/PROVIDER_INTEGRATION.md) — Provider details
- [development/ROADMAP.md](development/ROADMAP.md) — Project phases

### I want to troubleshoot issues

- [getting-started/SETUP.md](getting-started/SETUP.md#troubleshooting) — Common issues
- [getting-started/EXAMPLES.md](getting-started/EXAMPLES.md#troubleshooting-examples) — Configuration errors
- [reference/CONFIG.md](reference/CONFIG.md#troubleshooting) — Validation errors

### I want to understand specific technical areas

- **Providers:** [technical/PROVIDER_INTEGRATION.md](technical/PROVIDER_INTEGRATION.md)
- **HTTP/Streaming:** [technical/HTTP_INTEGRATION.md](technical/HTTP_INTEGRATION.md)
- **Retry Logic:** [technical/RETRY_STRATEGY.md](technical/RETRY_STRATEGY.md)
- **Future Chat:** [technical/CHAT_ARCHITECTURE.md](technical/CHAT_ARCHITECTURE.md)
- **Future Completions:** [technical/GH_COPILOT_LSP_INTEGRATION.md](technical/GH_COPILOT_LSP_INTEGRATION.md)

---

## 📋 Additional Resources

### [CHANGELOG.md](CHANGELOG.md)
Version history and release notes.

### [settings.schema.json](settings.schema.json)
JSON schema for Zed settings validation.

---

## 🗺️ Quick Reference

### Current Status

- ✅ **Foundation** — Complete
- ✅ **OpenAI integration** — Complete
- ✅ **Anthropic integration** — Complete
- ✅ **Configuration system** — Complete
- ✅ **HTTP & streaming** — Complete
- 📅 **Chat interface** — Q2 2025 (Primary milestone)
- 📅 **Code completions** — Q3 2025+

### Supported AI Models

**OpenAI:**
- GPT-4o (recommended)
- o1 (advanced reasoning)
- o3-mini (lightweight)

**Anthropic Claude:**
- Claude Opus 4.1 (most powerful)
- Claude Sonnet 4 (recommended)
- Claude Haiku 4.5 (fastest)

### Common Commands

```bash
make fmt          # Format code
make clippy       # Check warnings
make test         # Run tests
make check-all    # Run all checks
```

---

## ❓ Need Help?

- **Can't find what you need?** Use the [Documentation by Goal](#-documentation-by-goal) section
- **Setup issues?** → [getting-started/SETUP.md](getting-started/SETUP.md#troubleshooting)
- **Configuration help?** → [getting-started/EXAMPLES.md](getting-started/EXAMPLES.md)
- **Want to contribute?** → [development/CONTRIBUTING.md](development/CONTRIBUTING.md)
- **Questions about the project?** → [GitHub Discussions](https://github.com/zed-industries/zed-copilot/discussions)

---

**Back to:** [Project README](../README.md)

**Happy coding! 🎉**