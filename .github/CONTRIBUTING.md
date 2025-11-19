# Contributing to Zed Copilot

Thank you for your interest in contributing to Zed Copilot! This document outlines the contribution process and expectations.

## Before You Start

1. Check [existing issues](https://github.com/zed-industries/zed-copilot/issues) to avoid duplication
2. For major changes, open an issue first to discuss your approach
3. Familiarize yourself with the [Development Guide](../docs/DEVELOPMENT.md)
4. Review the [Testing Guide](../docs/TESTING.md) for test requirements

## Development Workflow

### Setup

```bash
# Clone the repository
git clone https://github.com/zed-industries/zed-copilot.git
cd zed-copilot

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build --release

# Run tests locally
cargo test

# Or use the Makefile for convenience
make check-all  # Run fmt, clippy, and test in one command
</parameter>
```

### Making Changes

1. Create a feature branch: `git checkout -b feature/your-feature-name`
2. Make focused, atomic commits
3. Write tests for new functionality
4. Run local checks before pushing:

```bash
# Quick way - run all checks at once
make check-all

# Or run individually
make fmt       # Format code
make clippy    # Check for warnings
make test      # Run all tests

# View all available commands
make help
```

## Code Standards

### Naming Conventions

- **Functions**: Verbs in snake_case (`fetch_user_profile`)
- **Types**: Nouns in PascalCase (`RetryPolicy`)
- **Constants**: UPPER_SNAKE_CASE (`MAX_RETRIES`)
- **Clarity > Brevity**: Use `timeout_ms` not `t`

### Code Quality

- Keep functions small and focused (single responsibility)
- Use early returns for clarity
- Prefer explicit types over type inference where it aids readability
- No inline comments (code must explain itself)
- No TODO/FIXME comments (use issues instead)
- Write intention-revealing names

### Testing

- Add tests for all new functionality
- Use Arrange-Act-Assert pattern
- Test both happy paths and error cases
- Keep test names descriptive: `test_completion_returns_valid_response_when_api_succeeds`

See [TESTING.md](../docs/TESTING.md) for detailed testing guidelines.

## CI/CD Pipeline

All pull requests automatically run through our CI pipeline:

### Jobs

1. **Test Suite** (`test`)
   - Runs unit tests: `cargo test --lib`
   - Runs integration tests: `cargo test --test '*'`
   - All 14 tests must pass

2. **Formatting** (`fmt`)
   - Checks code follows Rust conventions
   - Command: `cargo fmt -- --check`
   - Fix locally with: `cargo fmt`

3. **Linting** (`clippy`)
   - Checks for code warnings and anti-patterns
   - Command: `cargo clippy -- -D warnings`
   - All warnings treated as errors

4. **Build** (`build`)
   - Builds WASM binary for `wasm32-unknown-unknown` target
   - Verifies binary exists
   - Checks binary size (must be < 1.5MB)
   - Creates artifact for download

5. **Summary** (`summary`)
   - Reports overall CI status
   - Fails if any job failed

### Running CI Locally

Before pushing, run all checks locally:

```bash
# Quick way using Makefile
make check-all

# Or install act to simulate GitHub Actions exactly:
brew install act  # macOS
choco install act  # Windows
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | bash  # Linux

# Run CI jobs locally
act -j test
act -j fmt
act -j clippy
act -j build
act  # runs all jobs
```

### CI Triggers

- Pushes to `main` or `develop` branches
- All pull requests to `main` or `develop`
- Manual trigger via GitHub Actions UI
- Changes to:
  - `src/**`
  - `tests/**`
  - `Cargo.toml`, `Cargo.lock`
  - `extension.toml`
  - `.github/workflows/ci.yml`

Documentation and license changes skip CI.

## Submission Checklist

Before opening a pull request, ensure:

- [ ] Code follows Rust conventions: `make fmt`
- [ ] All tests pass: `make test`
- [ ] No warnings: `make clippy`
- [ ] Full check passed: `make check-all`
- [ ] Tests added for new features
- [ ] Documentation updated in `docs/`
- [ ] `CHANGELOG.md` updated with your changes
- [ ] Commit messages are clear and descriptive
- [ ] No unrelated changes in the PR

## Pull Request Process

1. **Create PR** with descriptive title and description
2. **Link Issues**: Reference related issues with `Fixes #123`
3. **Await Review**: Automated checks must pass; maintainers will review
4. **Address Feedback**: Respond to comments and make changes as needed
5. **Approval & Merge**: Maintainers will merge once approved

### PR Title Format

```
[type]: Brief description

Types: feat, fix, docs, style, refactor, test, chore
Example: feat: Add OpenAI provider integration
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

## Screenshots/Output
If applicable, add screenshots or test output.
```

## Review Process

1. **Automated Checks**: CI pipeline must pass
2. **Code Review**: At least one maintainer review
3. **Manual Testing**: Changes may be tested locally
4. **Approval**: Maintainers will approve when ready
5. **Merge**: Squash-and-merge to keep history clean

## Reporting Issues

When reporting bugs or suggesting features:

1. **Search first**: Check existing issues
2. **Be specific**: Describe expected vs. actual behavior
3. **Provide context**: OS, Rust version, steps to reproduce
4. **Include logs**: Relevant error messages or stack traces
5. **Minimal example**: Isolate the issue if possible

## License

By contributing, you agree that your contributions will be licensed under the MIT License (see [LICENSE](../LICENSE)).

## Questions?

Open a discussion or issue if you have questions. We're here to help!

---

**Current Project Status**: Phase 1 - Foundation (v0.0.1)  
**Next Phase**: Phase 2 - AI Provider Integration (v0.1.0)

See [DEVELOPMENT.md](../docs/DEVELOPMENT.md) for the full roadmap.