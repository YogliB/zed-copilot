# Zed Copilot Development Guide

## Architecture Overview

Zed Copilot is built as a WebAssembly-based extension for Zed IDE using Rust. The architecture is designed to be modular and extensible, supporting multiple AI providers and feature types.

### Core Components

```
┌─────────────────────────────────────────────────┐
│           Zed IDE (Host)                        │
├─────────────────────────────────────────────────┤
│  Zed Copilot Extension (WebAssembly)            │
│  ┌──────────────────────────────────────────┐   │
│  │ ZedCopilot (Extension Struct)            │   │
│  │ ├── AI Provider Manager                  │   │
│  │ ├── Completion Engine                    │   │
│  │ ├── Context Manager                      │   │
│  │ └── Logger/Telemetry                     │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
         ↓
┌─────────────────────────────────────────────────┐
│  External AI Providers (Future)                 │
│  ├── OpenAI API                                 │
│  ├── Anthropic Claude API                       │
│  └── Other LLM Providers                        │
└─────────────────────────────────────────────────┘
```

### Current Implementation (v0.0.1)

Currently, the extension provides:
- **Extension Registration**: Basic ZedCopilot struct implementing the Extension trait
- **Initialization**: Logging on startup to verify proper loading
- **Test Framework**: Unit and integration tests with 14 passing tests
- **Foundation**: Ready for AI provider integration in Phase 2

### Extension Trait Implementation

The `ZedCopilot` struct implements Zed's `Extension` trait with:
- `new()`: Initializes the extension with logging
- `Default`: Provides default instantiation path
- Full test coverage with unit and integration tests

## Project Structure

```
zed-copilot/
├── extension.toml          # Zed extension metadata
├── Cargo.toml              # Rust dependencies and build config
├── src/
│   ├── lib.rs              # Main extension implementation (with unit tests)
│   ├── provider/            # (Planned) AI provider abstraction
│   │   ├── mod.rs
│   │   ├── openai.rs
│   │   └── anthropic.rs
│   ├── completion/          # (Planned) Completion logic
│   │   └── mod.rs
│   ├── context/             # (Planned) Context extraction
│   │   └── mod.rs
│   └── telemetry/           # (Planned) Logging and metrics
│       └── mod.rs
├── tests/
│   ├── common/
│   │   └── mod.rs          # Shared test utilities (with tests)
│   └── integration_tests.rs # Integration tests
├── docs/
│   ├── DEVELOPMENT.md       # This file
│   └── TESTING.md           # Comprehensive testing guide
├── README.md                # User guide
├── CHANGELOG.md             # Version history
├── LICENSE                  # MIT License
└── .gitignore               # Git ignore rules
```

## Development Workflow

### Setup Development Environment

1. Clone the repository and navigate to it
2. Ensure Rust is installed via rustup
3. Build the project: `cargo build --release`
4. Install as dev extension in Zed
5. Test with `zed --foreground` for debug output

### Making Changes

1. Edit source files in `src/`
2. Run `cargo build --release` to compile
3. Reload extension in Zed (may require restart)
4. Check `zed: open log` for any errors
5. Write tests in `tests/` directory (see TESTING.md for guidelines)

### Code Standards

- Follow Rust conventions (rustfmt, clippy)
- Add documentation comments for public items
- Use meaningful error messages
- Keep functions focused and testable
- Log important events for debugging
- Write tests for new functionality (see TESTING.md)

### Running Checks

```bash
# Format code
cargo fmt

# Check for warnings and issues
cargo clippy

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test '*'

# Check documentation
cargo doc --no-deps --open

# Full quality check
cargo fmt && cargo clippy && cargo test
```

## Testing

Zed Copilot includes a comprehensive test framework with 14 passing tests (5 unit + 9 integration).

### Quick Start

```bash
# Run all tests
cargo test

# Run tests with output (including println!)
cargo test -- --nocapture

# Run specific test
cargo test test_zed_copilot_new
```

### Test Structure

- **Unit Tests**: Located in `src/lib.rs`, test individual components
- **Integration Tests**: Located in `tests/integration_tests.rs`, test component interactions
- **Test Utilities**: Shared helpers in `tests/common/mod.rs`

### Current Test Coverage

- **Unit Tests (5)**: ZedCopilot instantiation, Default trait, Extension trait, panic handling
- **Integration Tests (9)**: Extension initialization, context creation, stability tests

### Adding New Tests

When adding features, write tests following these patterns:

```rust
// Unit test (in src/lib.rs or a module)
#[test]
fn test_my_feature() {
    // Arrange
    let component = MyComponent::new();
    
    // Act
    let result = component.do_something();
    
    // Assert
    assert_eq!(result, expected_value);
}
```

See `docs/TESTING.md` for comprehensive testing guide with examples and best practices.

## Planned Features (Roadmap)

### Phase 1: Foundation (Current - v0.0.1)
- [x] Basic extension scaffolding
- [x] Zed extension registration
- [x] Project documentation
- [x] Unit test framework (14 tests, all passing)
- [ ] CI/CD setup (GitHub Actions)

### Phase 2: AI Provider Integration (v0.1.0)
- [ ] Abstract AI provider interface
- [ ] OpenAI API integration
- [ ] Anthropic Claude integration
- [ ] API key management
- [ ] Provider configuration
- [ ] Error handling and retries

### Phase 3: Code Completion (v0.2.0)
- [ ] Completion trigger logic
- [ ] Context extraction from buffer
- [ ] Response formatting
- [ ] Caching strategy
- [ ] Performance optimization

### Phase 4: Advanced Features (v0.3.0+)
- [ ] Multi-language support
- [ ] Custom prompts
- [ ] Code refactoring suggestions
- [ ] Documentation generation
- [ ] Test generation
- [ ] Debugging assistance

### Phase 5: Polish & Publishing (v1.0.0)
- [ ] Comprehensive test coverage
- [ ] Performance optimization
- [ ] Security audit
- [ ] Official documentation
- [ ] Publish to Zed extension registry

## API Integration Strategy

### AI Provider Interface (Planned)

```rust
pub trait AiProvider {
    async fn complete(&self, prompt: &str) -> Result<String>;
    async fn is_available(&self) -> bool;
    fn name(&self) -> &str;
}

pub struct OpenAiProvider {
    api_key: String,
    model: String,
}

pub struct AnthropicProvider {
    api_key: String,
    model: String,
}
```

### Configuration (Planned)

Users will configure providers in Zed settings:

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

## Performance Considerations

- **WASM Size**: Keep compiled binary under 1MB
- **Startup Time**: Minimize initialization overhead
- **Latency**: Cache common completions
- **Memory**: Stream responses to avoid large buffers
- **Network**: Implement request batching

## Security Considerations

- **API Keys**: Never hardcode keys; use environment variables
- **Data Privacy**: Don't send user code without consent
- **Network**: Use HTTPS for all API calls
- **Validation**: Validate all external responses
- **Dependencies**: Keep dependencies minimal and updated

## Dependencies

### Current
- `zed_extension_api = "0.1"` - Zed extension API

### Planned
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `serde`/`serde_json` - Serialization
- `env_logger` - Logging
- `thiserror` - Error handling

## Known Issues & Limitations

### v0.0.1
- No AI integration yet (planned for Phase 2)
- No configuration options (planned for Phase 2)
- WASM limitations on std library features (documented in TESTING.md)

## Contributing Guidelines

### Before You Start
1. Check existing issues to avoid duplication
2. Discuss major changes in an issue first
3. Follow the code standards listed above
4. Add tests for new functionality (see TESTING.md)

### Submission Checklist
- [ ] Code follows Rust conventions
- [ ] All tests pass: `cargo test`
- [ ] No warnings: `cargo clippy`
- [ ] Formatted: `cargo fmt`
- [ ] Tests added for new features
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Commit messages are clear and descriptive

### Review Process
1. Automated checks must pass
2. Code review by maintainers
3. Manual testing
4. Approval and merge

## Building for Release

```bash
# Build optimized release binary
cargo build --release

# This generates: target/release/zed_copilot.wasm

# Verify size
ls -lh target/release/*.wasm

# Run full test suite
cargo test --release

# Verify all quality checks
cargo fmt && cargo clippy && cargo test
```

## Publishing to Registry

When ready for publication:

1. Update version in `extension.toml` and `Cargo.toml`
2. Update `CHANGELOG.md`
3. Tag release: `git tag v0.1.0`
4. Fork `zed-industries/extensions` repo
5. Add as submodule in `extensions/` directory
6. Update root `extensions.toml`
7. Submit PR to official repository

See [Zed Publishing Guide](https://zed.dev/docs/extensions/developing-extensions.html#publishing-your-extension) for details.

## Debugging

### Enable Verbose Logging
```bash
zed --foreground
```

### View Extension Logs
Use `zed: open log` command in Zed to inspect:
- Extension initialization
- API errors
- Performance metrics

### Debug Tests

```bash
# Run tests with output
cargo test -- --nocapture

# Run single test with output
cargo test test_name -- --nocapture

# Run tests in single-threaded mode
cargo test -- --test-threads=1
```

See TESTING.md for more debugging tips.

### Common Issues & Solutions

**Extension doesn't load:**
- Check for Rust compilation errors: `cargo build --release`
- Verify `zed_extension_api` version compatibility
- Check Zed version compatibility

**Crashes on startup:**
- Review Zed.log for panic messages
- Simplify recent changes to isolate issue
- Test with minimal code
- Run `cargo test` to verify tests still pass

**Performance degradation:**
- Profile with `cargo flamegraph`
- Check for blocking operations
- Review memory usage
- Run benchmarks if available

**Tests failing:**
- Run `cargo test -- --nocapture` to see output
- Check TESTING.md for debugging strategies
- Verify recent code changes don't break invariants

## Useful Resources

- [TESTING.md](./TESTING.md) — Comprehensive testing guide
- [Zed Extension API Docs](https://zed.dev/docs/extensions)
- [Zed Extension Capabilities](https://zed.dev/docs/extensions/capabilities.html)
- [Rust WebAssembly Book](https://rustwasm.github.io/book/)
- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Async Tutorial](https://tokio.rs/tokio/tutorial)
- [OpenAI API Docs](https://platform.openai.com/docs)
- [Anthropic Claude Docs](https://docs.anthropic.com/)

## License

This project is MIT licensed. See LICENSE file for details.

---

**Last Updated:** 2024
**Current Version:** 0.0.1
**Status:** Phase 1 - Foundation (Unit Test Framework Complete ✅)
**Maintainers:** Zed Copilot Contributors