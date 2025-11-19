# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Phase 2.2 (In Progress)
- Configuration system for Zed settings
- Credential management and validation
- Environment variable interpolation
- Per-provider configuration support
- Configuration error handling

### Phase 2.3 (Planned - Q1 2025)
- HTTP client integration with reqwest
- Streaming response support (SSE)
- Retry logic with exponential backoff
- Rate limiting enforcement
- Network error recovery

### Phase 3 (Planned - Q2 2025)
- Chat interface (primary feature)
- Multi-turn conversation support
- Message history storage and retrieval
- Streaming response display
- Code context integration

### Phase 4+ (Optional)
- Code completion engine
- Advanced features (refactoring, test generation, etc.)
- Custom system prompts
- Performance optimizations

For detailed timeline, see [ROADMAP.md](../docs/ROADMAP.md).

## [0.1.0-alpha] - 2024-11-20

### Added
- Abstract `AiProvider` trait for multi-provider support
- `OpenAiProvider` implementation with GPT-4 and GPT-3.5-turbo support
- `AnthropicProvider` implementation with Claude 3 family support
- `ProviderFactory` for convenient provider instantiation
- Comprehensive error handling with `ProviderError` enum
- Provider request payload building (OpenAI and Anthropic formats)
- Full async/await support with `async-trait` crate
- 31 unit tests covering all provider implementations
- `PROVIDER_INTEGRATION.md` documentation with architecture and usage examples
- Support for custom API base URLs per provider
- Provider availability checking via `is_available()` method

### Notes
- Phase 2.1 (AI Provider Integration) complete
- Trait-based design ready for additional providers (Ollama, Cohere, etc.)
- HTTP client integration deferred to Phase 2.3
- Retry logic and caching deferred to Phase 3
- All tests passing with 100% success rate

## [0.0.1] - 2024-11-19

### Added
- Initial project scaffolding
- Extension metadata in `extension.toml`
- Cargo.toml with WebAssembly configuration
- Basic `ZedCopilot` struct implementing Extension trait
- Extension registration and initialization
- Comprehensive README with installation and troubleshooting guide
- DEVELOPMENT.md with architecture and roadmap
- This CHANGELOG
- MIT LICENSE
- .gitignore for Rust and Zed projects
- Logging on extension initialization
- WebAssembly build configuration with optimizations (size, LTO, strip)

### Notes
- Foundation release for AI-powered code assistance in Zed
- Ready for dev extension installation and testing
- No AI provider integration yet (coming in v0.1.0)
- All code follows Rust conventions and best practices

---

## Version History

### How to Contribute to This Changelog

When submitting a PR, please update this file by:

1. Adding your changes under the `[Unreleased]` section
2. Using appropriate categories: `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`
3. Keeping entries concise and user-focused
4. When releasing, move `[Unreleased]` section to a new version header

### Release Process

When releasing a new version:

1. Update version in `extension.toml` and `Cargo.toml`
2. Move `[Unreleased]` section to new version header with date
3. Create git tag: `git tag vX.Y.Z`
4. Push tag and trigger release workflow
5. Announce release in appropriate channels

---

**Last Updated:** November 19, 2024
**Current Version:** 0.0.1