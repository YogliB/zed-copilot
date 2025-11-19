# Configuration Guide

> **Status:** Phase 2.2 (In Progress)  
> **Target Completion:** Q1 2025  
> **See Also:** [ROADMAP.md](ROADMAP.md#phase-22-configuration--credentials) | [DEVELOPMENT.md](DEVELOPMENT.md#phase-22-configuration--credentials)

## Overview

This guide documents how to configure Zed Copilot for chat functionality and AI provider selection.

Configuration is stored in Zed's `settings.json` file, which supports environment variable interpolation for secure credential management.

## Configuration File Location

- **macOS:** `~/.config/zed/settings.json`
- **Linux:** `~/.config/zed/settings.json`
- **Windows:** `%APPDATA%\Zed\settings.json`

## Chat Configuration

> **Implementation:** Phase 2.2 (coming soon)

### Enabling/Disabling Chat

[TODO: Add after Phase 2.2 implementation]

### Chat History Settings

[TODO: Add after Phase 2.2 implementation]

### Streaming Configuration

[TODO: Add after Phase 2.2 implementation]

## Provider Configuration

> **Implementation:** Phase 2.2 (coming soon)

### OpenAI Configuration

[TODO: Add after Phase 2.2 implementation]

Example (when available):
```json
{
  "zed_copilot": {
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o"
    }
  }
}
```

### Anthropic Configuration

[TODO: Add after Phase 2.2 implementation]

Example (when available):
```json
{
  "zed_copilot": {
    "provider": "anthropic",
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-opus-4-1-20250805"
    }
  }
}
```

## Environment Variables

> **Implementation:** Phase 2.2 (coming soon)

### Setting Up API Keys

[TODO: Add after Phase 2.2 implementation]

### Credential Management

[TODO: Add after Phase 2.2 implementation]

## Validation

> **Implementation:** Phase 2.2 (coming soon)

Configuration is validated when Zed loads. Common validation errors:

[TODO: Add error messages and solutions after Phase 2.2 implementation]

## Troubleshooting

### Configuration Won't Load

[TODO: Add troubleshooting steps after Phase 2.2 implementation]

### API Key Not Recognized

[TODO: Add troubleshooting steps after Phase 2.2 implementation]

### Wrong Provider Selected

[TODO: Add troubleshooting steps after Phase 2.2 implementation]

## See Also

- [DEVELOPMENT.md](DEVELOPMENT.md#phase-22-configuration--credentials) — Implementation details
- [ROADMAP.md](ROADMAP.md#phase-22-configuration--credentials) — Phase 2.2 timeline
- [PROVIDER_INTEGRATION.md](PROVIDER_INTEGRATION.md) — AI provider details
- [README.md](README.md#configuration) — User configuration guide

---

**Last Updated:** November 2024  
**Phase:** 2.2 (Configuration & Credentials)  
**Status:** Stub (awaiting implementation)