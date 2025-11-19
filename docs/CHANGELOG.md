# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- AI provider abstraction layer
- OpenAI integration
- Anthropic Claude integration
- Code completion engine
- Context extraction system
- Configuration management
- API key handling
- Performance optimizations

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