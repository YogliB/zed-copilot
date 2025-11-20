# Contributing to Zed Copilot

Welcome! We're excited you want to contribute to Zed Copilot. This guide will help you get started.

> **Part of:** [Zed Copilot Documentation](../README.md)

---

## First Time Contributing?

No worries! Here's what you need to know:

1. **Check existing issues** — Someone might already be working on your idea
2. **Start small** — Documentation fixes, tests, and small features are great first contributions
3. **Ask questions** — Open a discussion or comment on an issue if you're unsure
4. **Be patient** — Reviews may take a few days

---

## Quick Start

### 1. Set Up Your Environment

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/zed-copilot.git
cd zed-copilot

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install git hooks (highly recommended)
bash scripts/setup-hooks.sh

# Build the project
cargo build --release

# Run tests to verify everything works
cargo test
```

### 2. Make Your Changes

```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Make your changes
# Edit files in src/

# Run checks before committing
make check-all
```

### 3. Submit Your Work

```bash
# Commit with a clear message
git add .
git commit -m "feat: Add your feature description"

# Push to your fork
git push origin feature/your-feature-name

# Open a pull request on GitHub
```

---

## What Can I Contribute?

### 🐛 Bug Fixes

Found a bug? We'd love a fix!

1. Check if there's already an issue for it
2. If not, open one describing the bug
3. Submit a PR with your fix
4. Include tests if possible

### ✨ New Features

Have an idea? Great!

1. **Open an issue first** to discuss the feature
2. Wait for maintainer feedback before coding
3. Follow the implementation plan from the discussion
4. Submit PR with tests and documentation

### 📚 Documentation

Documentation is always welcome:

- Fix typos or unclear explanations
- Add examples or use cases
- Improve setup instructions
- Write tutorials or guides

### 🧪 Tests

Help improve test coverage:

- Add tests for untested features
- Improve existing test clarity
- Add edge case tests
- Fix flaky tests

### 💡 Ideas and Discussions

Not ready to code? Share ideas!

- Open a discussion for new features
- Suggest improvements
- Report usability issues
- Ask questions

---

## Development Workflow

### Before You Start

Read these docs to understand the project:

- **[DEVELOPMENT.md](DEVELOPMENT.md)** — Architecture and design
- **[TESTING.md](TESTING.md)** — Testing guidelines
- **[ROADMAP.md](ROADMAP.md)** — Project timeline and priorities
- **[../getting-started/SETUP.md](../getting-started/SETUP.md)** — Installation and setup
- **[../getting-started/QUICKSTART.md](../getting-started/QUICKSTART.md)** — Quick start guide
- **[../../docs/GIT_HOOKS.md](../../docs/GIT_HOOKS.md)** — Git hooks and pre-push validation

### Git Hooks Setup

After cloning and before making changes, install git hooks:

```bash
bash scripts/setup-hooks.sh
```

This installs **pre-commit** and **pre-push** hooks that:
- Validate code formatting and linting on commit (fast, ~5-10 sec)
- Run smart tests on push (only for changed modules, ~20-45 sec)
- Prevent test failures from reaching CI

Hooks are optional but **highly recommended** to catch issues early. See [GIT_HOOKS.md](../../docs/GIT_HOOKS.md) for details.

### Making Changes

**1. Write Tests First** (TDD recommended)

```rust
#[test]
fn test_my_new_feature() {
    // Arrange
    let input = "test input";
    
    // Act
    let result = my_function(input);
    
    // Assert
    assert_eq!(result, "expected output");
}
```

**2. Implement Your Feature**

Follow our code standards:
- Self-explanatory code (no comments)
- Clear, intention-revealing names
- Small, focused functions
- Simple control flow with early returns

**3. Run All Checks**

```bash
make check-all
```

This runs:
- `cargo fmt` — Code formatting
- `cargo clippy` — Linting and warnings
- `cargo test` — All tests

**Note:** If you have git hooks installed, `cargo fmt` and `cargo clippy` checks are enforced automatically on commit. You can also run `scripts/smart-test.sh` to preview what tests will run on push.

**Fix any issues before committing.**

### Commit Messages

Use clear, descriptive commit messages:

```
type: Brief description

Longer explanation if needed.
```

**Types:**
- `feat:` — New feature
- `fix:` — Bug fix
- `docs:` — Documentation only
- `test:` — Adding or updating tests
- `refactor:` — Code refactoring
- `chore:` — Maintenance tasks

**Examples:**
```
feat: Add streaming support for chat responses
fix: Handle missing API key gracefully
docs: Improve setup instructions for Windows
test: Add integration tests for configuration
```

---

## Code Standards

### Follow the Project Rules

See [zed-rules/AGENTS.md](https://github.com/zed-industries/zed-rules/blob/main/AGENTS.md) for complete coding rules.

**Key principles:**

1. **No comments** — Code must be self-explanatory
2. **Clear names** — Use intention-revealing names
3. **Small functions** — One purpose per function
4. **Simple flow** — Prefer early returns
5. **Types over comments** — Express intent via types

### Naming Conventions

```rust
// Functions: verbs in snake_case
fn fetch_user_profile() { }

// Types: nouns in PascalCase
struct RetryPolicy { }

// Constants: UPPER_SNAKE_CASE
const MAX_RETRIES: u32 = 3;

// Include units and context
let timeout_ms = 5000;      // ✅ Good
let t = 5000;               // ❌ Bad
```

### Example: Good Code

```rust
fn validate_api_key(key: &str) -> Result<(), ConfigError> {
    if key.is_empty() {
        return Err(ConfigError::MissingApiKey);
    }
    
    if !key.starts_with("sk-") {
        return Err(ConfigError::InvalidApiKeyFormat);
    }
    
    Ok(())
}
```

### Example: Bad Code

```rust
// Don't write code like this!
fn check(k: &str) -> Result<(), ConfigError> {
    // Check if key is valid
    if k.is_empty() {
        return Err(ConfigError::MissingApiKey);
    }
    
    // TODO: Add more validation
    Ok(())
}
```

---

## Testing

### Write Tests for All Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Test Structure

```rust
#[test]
fn test_feature_succeeds_when_input_valid() {
    // Arrange — Set up test data
    let config = create_test_config();
    
    // Act — Execute the feature
    let result = process_config(config);
    
    // Assert — Verify the result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_value);
}
```

### Testing Guidelines

- Test both success and failure cases
- Use descriptive test names
- Keep tests independent
- Mock external dependencies
- See [TESTING.md](TESTING.md) for details

---

## Pull Request Process

### Before Submitting

- [ ] Code follows style guidelines
- [ ] Tests added for new features
- [ ] All tests pass: `cargo test`
- [ ] No warnings: `cargo clippy`
- [ ] Code formatted: `cargo fmt`
- [ ] Documentation updated
- [ ] Commit messages are clear

### PR Title Format

```
[type]: Brief description

Examples:
feat: Add configuration validation
fix: Prevent crash when API key missing
docs: Update quickstart guide
```

### PR Description Template

```markdown
## Description
Brief explanation of what this PR does.

## Related Issues
Fixes #123
Related to #456

## Changes
- Change 1
- Change 2
- Change 3

## Testing
How was this tested?

## Screenshots
If applicable, add screenshots or logs.
```

### Review Process

1. **Automated checks run** — CI must pass
2. **Maintainer review** — Usually within 2-3 days
3. **Address feedback** — Make requested changes
4. **Approval** — Maintainer approves PR
5. **Merge** — Squash and merge to keep history clean

---

## Continuous Integration

Our CI runs automatically on all PRs:

### CI Checks

1. **Tests** — All tests must pass
   ```bash
   cargo test --lib
   cargo test --test '*'
   ```

2. **Formatting** — Code must be formatted
   ```bash
   cargo fmt -- --check
   ```

3. **Linting** — No warnings allowed
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Build** — Must compile successfully
   ```bash
   cargo build --release
   ```

### Run CI Locally

Before pushing, run all checks:

```bash
make check-all
```

Or manually:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release
```

---

## Need Help?

## Documentation

- **[../getting-started/QUICKSTART.md](../getting-started/QUICKSTART.md)** — 5-minute setup
- **[../getting-started/SETUP.md](../getting-started/SETUP.md)** — Detailed installation
- **[DEVELOPMENT.md](DEVELOPMENT.md)** — Architecture guide
- **[TESTING.md](TESTING.md)** — Testing guide
- **[../README.md](../README.md)** — Documentation index

### Community

- **GitHub Issues** — Bug reports and features
- **GitHub Discussions** — Questions and ideas
- **Pull Requests** — Code contributions

### Stuck?

Don't hesitate to ask for help:

1. Comment on the issue you're working on
2. Open a discussion with your question
3. Tag maintainers if you need guidance

---

## Code of Conduct

### Be Respectful

- Be kind and welcoming
- Respect different viewpoints
- Accept constructive criticism
- Focus on what's best for the project

### Be Constructive

- Provide helpful feedback
- Explain your reasoning
- Suggest alternatives
- Help others learn

---

## Recognition

All contributors are recognized in:

- GitHub Contributors page
- Release notes
- Community acknowledgments

Thank you for making Zed Copilot better! 🎉

---

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](../../LICENSE).

---

**Questions?** Open a [discussion](https://github.com/zed-industries/zed-copilot/discussions) or comment on an issue.

**Ready to contribute?** Pick an issue or open a new one. We're here to help! 🚀

---

**Back to:** [Development](../README.md#quick-navigation)