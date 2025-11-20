# Development

Everything for contributing to Zed Copilot, understanding its architecture, and following the development process.

> **Part of:** [Zed Copilot Documentation](../../README.md#-documentation)

---

## 📖 Documents in This Folder

### [CONTRIBUTING.md](CONTRIBUTING.md)
**Contribution guide** — How to contribute to Zed Copilot.

Best for: Anyone interested in contributing code or documentation.

**Contains:**
- First-time contributor guide
- Development setup
- Code standards and naming conventions
- Testing requirements
- Pull request process
- CI/CD pipeline overview
- Code of Conduct

### [DEVELOPMENT.md](DEVELOPMENT.md)
**Architecture and development workflow** — Technical guide for developers.

Best for: Understanding how Zed Copilot works and developing features.

**Contains:**
- Architecture overview
- Component descriptions
- Development workflow
- Code standards
- Testing strategy
- Common development tasks
- Key components (Configuration, Providers)
- Performance and security considerations

### [TESTING.md](TESTING.md)
**Testing strategy and guidelines** — How to write and run tests.

Best for: Writing tests and understanding testing practices.

**Contains:**
- Test organization (unit vs integration)
- Running tests (all, specific, with output)
- Writing new tests
- Test structure and assertions
- Current test coverage
- Debugging tests
- Performance testing

### [ROADMAP.md](ROADMAP.md)
**Feature timeline and project phases** — What's planned and when.

Best for: Understanding project priorities and upcoming features.

**Contains:**
- Phase-by-phase breakdown
- Current status and progress
- Implementation timeline (Q1, Q2, Q3 2025+)
- Architecture overview by phase
- Success metrics
- API integration strategy

---

## 🚀 Quick Start for Contributors

**Want to contribute?** Follow this path:

1. Read **[CONTRIBUTING.md](CONTRIBUTING.md)** (understand the process)
2. Review **[DEVELOPMENT.md](DEVELOPMENT.md)** (understand architecture)
3. Follow **[TESTING.md](TESTING.md)** (write tests)
4. Check **[ROADMAP.md](ROADMAP.md)** (see what to work on)

**Want to understand the code?**

1. Start with **[DEVELOPMENT.md](DEVELOPMENT.md)** (architecture overview)
2. Review **[../technical/PROVIDER_INTEGRATION.md](../technical/PROVIDER_INTEGRATION.md)** (provider system)
3. Check **[ROADMAP.md](ROADMAP.md)** (phases and timeline)

---

## 📚 What to Read Next

- **Getting Started:** [../getting-started/QUICKSTART.md](../getting-started/QUICKSTART.md)
- **Configuration:** [../getting-started/EXAMPLES.md](../getting-started/EXAMPLES.md)
- **Technical Details:** [../technical/PROVIDER_INTEGRATION.md](../technical/PROVIDER_INTEGRATION.md)
- **Documentation Index:** [../../README.md#-documentation](../../README.md#-documentation)

---

## ❓ Frequently Asked Questions

### I want to start contributing. Where do I begin?

1. Read [CONTRIBUTING.md](CONTRIBUTING.md) - "First Time Contributing?" section
2. Follow the "Quick Start" in [CONTRIBUTING.md](CONTRIBUTING.md#quick-start)
3. Review [DEVELOPMENT.md](DEVELOPMENT.md) for the architecture
4. Check [ROADMAP.md](ROADMAP.md) for what to work on

### How do I understand the codebase?

1. Read [DEVELOPMENT.md](DEVELOPMENT.md) - "Architecture Overview" section
2. Review the project structure in [DEVELOPMENT.md](DEVELOPMENT.md#project-structure)
3. Study the key components in [DEVELOPMENT.md](DEVELOPMENT.md#key-components)
4. Deep dive into [../technical/PROVIDER_INTEGRATION.md](../technical/PROVIDER_INTEGRATION.md)

### What's the current project status?

Check [ROADMAP.md](ROADMAP.md) - "Current Status" section shows:
- What phase we're in
- What's complete
- What's coming next
- Timeline for features

### How do I write tests?

1. Read [TESTING.md](TESTING.md) - "Writing New Tests" section
2. Review examples in [TESTING.md](TESTING.md#test-structure)
3. Check current test coverage in [TESTING.md](TESTING.md#test-coverage-by-component)
4. Follow guidelines in [CONTRIBUTING.md](CONTRIBUTING.md#testing)

### What coding standards do I need to follow?

Check [CONTRIBUTING.md](CONTRIBUTING.md#code-standards) for:
- Naming conventions
- Code quality guidelines
- Testing requirements
- Example good/bad code

And [DEVELOPMENT.md](DEVELOPMENT.md#code-standards) for more details.

### What's the PR process?

1. Read [CONTRIBUTING.md](CONTRIBUTING.md#pull-request-process)
2. Follow the checklist in [CONTRIBUTING.md](CONTRIBUTING.md#before-submitting)
3. Use the PR template in [CONTRIBUTING.md](CONTRIBUTING.md#pr-description-template)
4. Wait for CI checks and reviews

---

## 🔗 Document Relationships

```
CONTRIBUTING.md ──┐
                  ├─→ DEVELOPMENT.md (architecture)
                  └─→ TESTING.md (testing)
                        │
ROADMAP.md ────────────┘
                   
DEVELOPMENT.md ───→ ../technical/PROVIDER_INTEGRATION.md
```

---

## 💡 Tips

### For First-Time Contributors
- Start with [CONTRIBUTING.md](CONTRIBUTING.md) - it's written for you!
- Don't be afraid to ask questions in the PR or discussions
- Start with small contributions (docs, tests, small features)

### For Regular Contributors  
- Check [ROADMAP.md](ROADMAP.md) for priorities
- Review [DEVELOPMENT.md](DEVELOPMENT.md) before major changes
- Follow [TESTING.md](TESTING.md) for test coverage requirements

### For Maintainers
- Refer to [CONTRIBUTING.md](CONTRIBUTING.md#review-process) for review workflow
- Use [ROADMAP.md](ROADMAP.md) to manage expectations
- Reference [DEVELOPMENT.md](DEVELOPMENT.md#code-standards) when reviewing code

---

## 🎯 Current Focus

**Phase:** 2.3 Complete ✅  
**Next:** Phase 3 — Chat Interface (Q2 2025) 🎯

See [ROADMAP.md](ROADMAP.md#current-status) for details.

---

**Back to:** [Documentation Index](../../README.md#-documentation)