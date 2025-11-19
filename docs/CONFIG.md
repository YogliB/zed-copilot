# Configuration Guide

## Overview

Zed Copilot configuration enables you to:
- Select your AI provider (OpenAI or Anthropic Claude)
- Set API credentials securely via environment variables
- Customize chat behavior (streaming, history, context)
- Configure provider-specific settings (model, timeout, API endpoint)

## Quick Start

### 1. Set Environment Variables

```bash
# For OpenAI
export OPENAI_API_KEY="sk-..."

# For Anthropic
export ANTHROPIC_API_KEY="sk-ant-..."
```

### 2. Configure Zed Settings

Add to your Zed `settings.json`:

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o",
      "api_base": "https://api.openai.com/v1",
      "timeout_secs": 30
    },
    "chat": {
      "streaming_enabled": true,
      "max_history_messages": 50,
      "auto_scroll_to_latest": true,
      "context_window_size": 4096
    }
  }
}
```

### 3. Restart Zed

The extension will load your configuration on startup.

## Configuration Schema

### Root Configuration

```json
{
  "zed_copilot": {
    "enabled": boolean,           // Enable/disable the extension
    "provider": string,           // "openai" or "anthropic"
    "openai": OpenAiConfig,       // OpenAI provider settings
    "anthropic": AnthropicConfig, // Anthropic provider settings
    "chat": ChatConfig            // Chat-specific settings
  }
}
```

### OpenAI Configuration

```json
{
  "openai": {
    "api_key": string,           // Required: OpenAI API key (use env var)
    "model": string,             // Optional: default "gpt-4o"
    "api_base": string,          // Optional: default "https://api.openai.com/v1"
    "timeout_secs": number       // Optional: default 30
  }
}
```

**Supported Models:**
- `gpt-4o` — Multimodal, most capable (recommended)
- `o1` — Advanced reasoning for complex problems
- `o3-mini` — Lightweight reasoning model
- `gpt-4-turbo` — Previous generation (not recommended)

**Example:**

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o",
      "api_base": "https://api.openai.com/v1",
      "timeout_secs": 60
    }
  }
}
```

### Anthropic Configuration

```json
{
  "anthropic": {
    "api_key": string,           // Required: Anthropic API key (use env var)
    "model": string,             // Optional: default "claude-opus-4-1-20250805"
    "api_base": string,          // Optional: default "https://api.anthropic.com/v1"
    "timeout_secs": number       // Optional: default 30
  }
}
```

**Supported Models:**
- `claude-opus-4-1-20250805` — Most powerful, for complex reasoning (recommended)
- `claude-sonnet-4-20250514` — Balanced intelligence and speed
- `claude-haiku-4-5-20251001` — Fastest, most affordable

**Example:**

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "anthropic",
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-opus-4-1-20250805",
      "api_base": "https://api.anthropic.com/v1",
      "timeout_secs": 30
    }
  }
}
```

### Chat Configuration

```json
{
  "chat": {
    "streaming_enabled": boolean,        // Optional: default true
    "max_history_messages": number,      // Optional: default 50
    "auto_scroll_to_latest": boolean,    // Optional: default true
    "context_window_size": number        // Optional: default 4096
  }
}
```

**Fields:**
- `streaming_enabled` — Enable real-time token streaming for responses
- `max_history_messages` — Maximum messages to keep in conversation history
- `auto_scroll_to_latest` — Auto-scroll chat panel to newest messages
- `context_window_size` — Token limit for context consideration

**Example:**

```json
{
  "chat": {
    "streaming_enabled": true,
    "max_history_messages": 100,
    "auto_scroll_to_latest": true,
    "context_window_size": 8192
  }
}
```

## Environment Variable Interpolation

### Syntax

Use `${VARIABLE_NAME}` to reference environment variables:

```json
{
  "openai": {
    "api_key": "${OPENAI_API_KEY}"
  }
}
```

### Why Use Environment Variables?

- **Security:** Never hardcode API keys in settings files
- **Portability:** Same config across machines with different keys
- **Version Control:** Safe to commit configuration without exposing secrets

### Setting Environment Variables

**macOS/Linux:**

```bash
# Temporary (current shell only)
export OPENAI_API_KEY="sk-..."

# Permanent (add to ~/.bashrc, ~/.zshrc, or ~/.profile)
echo 'export OPENAI_API_KEY="sk-..."' >> ~/.zshrc
source ~/.zshrc
```

**Windows (PowerShell):**

```powershell
# Temporary (current session)
$env:OPENAI_API_KEY = "sk-..."

# Permanent (User Environment Variables)
[System.Environment]::SetEnvironmentVariable("OPENAI_API_KEY", "sk-...", "User")
```

### Verify Environment Variables

```bash
# Check if variable is set
echo $OPENAI_API_KEY

# Verify it's accessible to Zed (may need to restart Zed first)
```

## Complete Configuration Example

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o",
      "api_base": "https://api.openai.com/v1",
      "timeout_secs": 30
    },
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-opus-4-1-20250805",
      "api_base": "https://api.anthropic.com/v1",
      "timeout_secs": 30
    },
    "chat": {
      "streaming_enabled": true,
      "max_history_messages": 50,
      "auto_scroll_to_latest": true,
      "context_window_size": 4096
    }
  }
}
```

## Troubleshooting

### "Environment variable not found"

**Problem:** Configuration fails with `EnvVarNotFound: OPENAI_API_KEY`

**Solution:**
1. Verify the environment variable is set: `echo $OPENAI_API_KEY`
2. If empty, set it: `export OPENAI_API_KEY="your_key"`
3. Restart Zed for changes to take effect
4. Check if Zed inherits the variable (may need shell integration)

### "Missing required field: openai.api_key"

**Problem:** API key is empty or not set

**Solution:**
1. Ensure `api_key` field exists in your config
2. If using `${VAR_NAME}`, verify the environment variable is set
3. Check for typos in variable names
4. Reload Zed settings (Cmd/Ctrl+Shift+P → "Reload Zed")

### "Invalid provider 'openai'"

**Problem:** Provider name is not recognized

**Solution:**
1. Check spelling: must be exactly `"openai"` or `"anthropic"`
2. Verify `provider` field is set in root config
3. Make sure you have corresponding provider config section

### "Configuration validation failed"

**Problem:** General validation error in your configuration

**Solution:**
1. Check JSON syntax (use an editor with JSON validation)
2. Verify all required fields are present
3. Check field types match schema (strings, booleans, numbers)
4. Review error message for specific field causing issue

### "API request timeout"

**Problem:** Requests to AI API take too long

**Solution:**
1. Increase `timeout_secs`: `"timeout_secs": 60`
2. Check network connectivity
3. Verify API is accessible from your location
4. Try with different model (may be overloaded)

## Security Best Practices

1. **Never Commit API Keys**
   ```bash
   # ❌ Don't do this
   "api_key": "sk-actual-key-here"
   
   # ✅ Do this instead
   "api_key": "${OPENAI_API_KEY}"
   ```

2. **Use Environment Variables**
   - Store keys in shell profile, `.env` files, or secrets manager
   - Never hardcode in version control

3. **Rotate Keys Regularly**
   - Update API keys periodically
   - Revoke compromised keys immediately
   - Use provider dashboards to manage keys

4. **Restrict API Permissions**
   - Create API keys with minimal required permissions
   - Use separate keys for different environments
   - Monitor key usage in provider dashboards

## Advanced Configuration

### Custom API Endpoints

Use custom API endpoints for self-hosted or proxy setups:

```json
{
  "openai": {
    "api_key": "${OPENAI_API_KEY}",
    "api_base": "https://custom-openai-proxy.example.com/v1"
  }
}
```

### Multiple Profiles

Keep multiple configurations for different scenarios:

```bash
# Switch between profiles
cp ~/.config/zed/settings-work.json ~/.config/zed/settings.json
# Restart Zed
```

## Getting API Keys

### OpenAI

1. Visit [platform.openai.com](https://platform.openai.com)
2. Sign up or log in
3. Go to API Keys section
4. Create new secret key
5. Copy and save securely (only shown once!)

### Anthropic

1. Visit [console.anthropic.com](https://console.anthropic.com)
2. Sign up or log in
3. Go to API Keys section
4. Create new API key
5. Copy and save securely (only shown once!)

## Support

- **Documentation:** See [DEVELOPMENT.md](./DEVELOPMENT.md) for architecture details
- **Issues:** Report on [GitHub Issues](https://github.com/zed-industries/zed-copilot/issues)
- **Discussions:** Join community discussions on GitHub