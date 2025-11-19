# Configuration Examples

Practical setup examples for common use cases.

## Example 1: OpenAI with GPT-4o (Recommended)

Most capable multimodal model, best for complex code analysis and understanding images.

### Setup Steps

1. **Get your API key:**
   - Visit https://platform.openai.com/api/keys
   - Create a new secret key
   - Copy and save it securely

2. **Set environment variable:**

   ```bash
   export OPENAI_API_KEY="sk-proj-..."
   ```

3. **Add to Zed settings.json:**

   ```json
   {
     "zed_copilot": {
       "enabled": true,
       "provider": "openai",
       "openai": {
         "api_key": "${OPENAI_API_KEY}",
         "model": "gpt-4o"
       }
     }
   }
   ```

4. **Restart Zed**

## Example 2: OpenAI with o1 (Advanced Reasoning)

Advanced reasoning model for complex problem-solving and deeply analytical tasks.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "o1"
    }
  }
}
```

Set environment variable:
```bash
export OPENAI_API_KEY="sk-proj-..."
```

## Example 3: OpenAI with o3-mini (Lightweight Reasoning)

Lightweight reasoning model that balances capability with speed and cost efficiency.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "o3-mini"
    }
  }
}
```

## Example 4: Anthropic Claude Opus 4.1 (Most Powerful)

Most capable Claude model. Best for complex reasoning and detailed analysis.

### Setup Steps

1. **Get your API key:**
   - Visit https://console.anthropic.com/api/keys
   - Create a new API key
   - Copy and save it securely

2. **Set environment variable:**

   ```bash
   export ANTHROPIC_API_KEY="sk-ant-..."
   ```

3. **Add to Zed settings.json:**

   ```json
   {
     "zed_copilot": {
       "enabled": true,
       "provider": "anthropic",
       "anthropic": {
         "api_key": "${ANTHROPIC_API_KEY}",
         "model": "claude-opus-4-1-20250805"
       }
     }
   }
   ```

4. **Restart Zed**

## Example 5: Anthropic Claude Sonnet 4 (Balanced)

Good balance of quality, speed, and cost. Recommended Claude option.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "anthropic",
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-sonnet-4-20250514"
    }
  }
}
```

Set environment variable:
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

## Example 6: Anthropic Claude Haiku 4.5 (Fast & Affordable)

Fastest Claude model. Best for quick responses with simple tasks and cost-sensitive applications.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "anthropic",
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-haiku-4-5-20251001"
    }
  }
}
```

## Example 7: Custom Timeout (Slow Network)

Increase timeout for slow or unstable internet connections.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o",
      "timeout_secs": 60
    }
  }
}
```

## Example 8: Custom API Endpoint (Proxy/Self-Hosted)

Route requests through a custom endpoint.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o",
      "api_base": "https://custom-proxy.example.com/v1"
    }
  }
}
```

## Example 9: Multiple Providers (Easy Switching)

Configure both providers, switch between them by changing `provider` field.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o"
    },
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-opus-4-1-20250805"
    }
  }
}
```

To switch to Anthropic, change:
```json
"provider": "anthropic"
```

Then restart Zed.

## Example 10: Optimized Chat Settings (Large History)

Keep more messages in history, increase context window.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o"
    },
    "chat": {
      "streaming_enabled": true,
      "max_history_messages": 100,
      "auto_scroll_to_latest": true,
      "context_window_size": 8192
    }
  }
}
```

## Example 11: No Streaming (Stable Connection Issues)

Disable streaming if you experience connection issues.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o"
    },
    "chat": {
      "streaming_enabled": false,
      "max_history_messages": 50
    }
  }
}
```

Responses will load as complete blocks instead of streaming tokens.

## Example 12: Minimal Configuration (All Defaults)

Use all defaults except API key.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}"
    }
  }
}
```

This uses:
- Model: `gpt-4o` (OpenAI default)
- Timeout: `30` seconds
- Chat streaming: enabled
- History: 50 messages
- Context: 4096 tokens

## Example 13: Complete Production Config

Full-featured setup for production use.

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o",
      "api_base": "https://api.openai.com/v1",
      "timeout_secs": 45
    },
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-opus-4-1-20250805",
      "api_base": "https://api.anthropic.com/v1",
      "timeout_secs": 45
    },
    "chat": {
      "streaming_enabled": true,
      "max_history_messages": 100,
      "auto_scroll_to_latest": true,
      "context_window_size": 8192
    }
  }
}
```

### Setup

```bash
# Set both provider keys
export OPENAI_API_KEY="sk-proj-..."
export ANTHROPIC_API_KEY="sk-ant-..."

# Start Zed
zed
```

### Switching Providers

Edit the `provider` field to switch:
```json
"provider": "anthropic"
```

Then reload Zed settings (Cmd/Ctrl+Shift+P → "Reload Zed").

## Environment Variable Setup by OS

### macOS/Linux (Bash/Zsh)

**Temporary (current session):**
```bash
export OPENAI_API_KEY="sk-proj-..."
zed
```

**Permanent (add to shell profile):**
```bash
# Add to ~/.zshrc (Zsh) or ~/.bashrc (Bash)
echo 'export OPENAI_API_KEY="sk-proj-..."' >> ~/.zshrc
source ~/.zshrc
```

### macOS (Finder/GUI)

1. Open Terminal
2. Run:
   ```bash
   nano ~/.zshrc
   ```
3. Add line:
   ```bash
   export OPENAI_API_KEY="sk-proj-..."
   ```
4. Press Ctrl+X, then Y, then Enter
5. Close and reopen Terminal
6. Verify: `echo $OPENAI_API_KEY`

### Windows (PowerShell)

**Temporary (current session):**
```powershell
$env:OPENAI_API_KEY = "sk-proj-..."
```

**Permanent (User Environment Variable):**
1. Press Windows+X, select "System"
2. Click "Advanced system settings"
3. Click "Environment Variables..."
4. Click "New..." under "User variables"
5. Variable name: `OPENAI_API_KEY`
6. Variable value: `sk-proj-...`
7. Click "OK"
8. Restart your terminal/IDE

**Verify:**
```powershell
echo $env:OPENAI_API_KEY
```

## Troubleshooting Examples

### "Environment variable not found: OPENAI_API_KEY"

**Problem:** Config references `${OPENAI_API_KEY}` but variable isn't set.

**Solution:**
```bash
# Check if variable is set
echo $OPENAI_API_KEY

# If empty, set it
export OPENAI_API_KEY="sk-proj-..."

# Verify
echo $OPENAI_API_KEY

# Restart Zed
```

### "Missing required field: openai.api_key"

**Problem:** API key field is missing or empty in JSON.

**Solution:**
```json
{
  "openai": {
    "api_key": "${OPENAI_API_KEY}",
    "model": "gpt-4o"
  }
}
```

Make sure:
1. `api_key` field exists
2. It uses `${VARIABLE_NAME}` syntax
3. Environment variable is set
4. JSON syntax is valid

### "Invalid provider 'gpt-4o'"

**Problem:** Using model name instead of provider name.

**Solution:**
```json
{
  "provider": "openai",
  "openai": {
    "api_key": "${OPENAI_API_KEY}",
    "model": "gpt-4o"
  }
}
```

- `provider` must be `"openai"` or `"anthropic"`
- `model` goes inside the provider config

## Comparison: When to Use What?

### Use OpenAI GPT-4o
- **Best for:** Complex code analysis, multimodal understanding, general purpose
- **Cost:** Moderate
- **Speed:** Fast
- **Use case:** Day-to-day coding (recommended)

### Use OpenAI o1
- **Best for:** Complex reasoning, advanced problem-solving
- **Cost:** Higher
- **Speed:** Slower (more thinking time)
- **Use case:** Difficult algorithms, architecture decisions

### Use OpenAI o3-mini
- **Best for:** Lightweight reasoning, quick analysis with some reasoning capability
- **Cost:** Lower than o1
- **Speed:** Faster than o1
- **Use case:** Moderate complexity tasks

### Use Claude Opus 4.1
- **Best for:** Complex analysis, long conversations, detailed reasoning
- **Cost:** Higher
- **Speed:** Medium
- **Use case:** Research, detailed documentation, complex refactoring

### Use Claude Sonnet 4
- **Best for:** Balanced speed and quality
- **Cost:** Medium
- **Speed:** Fast
- **Use case:** General coding tasks (recommended for Claude users)

### Use Claude Haiku 4.5
- **Best for:** Quick responses, simple tasks
- **Cost:** Lowest
- **Speed:** Fastest
- **Use case:** Quick inline assistance, cost-sensitive deployments

## Getting Started Checklist

- [ ] Choose a provider (OpenAI or Anthropic)
- [ ] Choose a model based on your needs
- [ ] Get API key from provider dashboard
- [ ] Set environment variable
- [ ] Add configuration to Zed `settings.json`
- [ ] Restart Zed
- [ ] Test by opening chat panel
- [ ] Verify it's responding correctly

See `CONFIG.md` for detailed documentation and troubleshooting.