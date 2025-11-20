# Reference

Configuration reference and schema documentation for Zed Copilot.

> **Part of:** [Zed Copilot Documentation](../../README.md#-documentation)

---

## 📖 Documents in This Folder

### [CONFIG.md](CONFIG.md)
**Configuration schema reference** — Complete guide to all configuration options.

Best for: Looking up configuration fields, understanding schema, advanced options.

**Contains:**
- Configuration overview
- Full schema documentation
- OpenAI configuration options
- Anthropic configuration options
- Chat configuration options
- Environment variable syntax
- Security best practices
- Advanced configuration (custom endpoints, multiple providers)
- Troubleshooting configuration errors
- Complete example configuration

---

## 🎯 When to Use This

### I need to configure Zed Copilot
→ Start with [../getting-started/EXAMPLES.md](../getting-started/EXAMPLES.md) for quick copy-paste configs

### I need detailed schema information
→ Read [CONFIG.md](CONFIG.md)

### I'm stuck on a configuration error
→ Check [CONFIG.md](CONFIG.md#troubleshooting)

### I need advanced configuration (proxy, custom endpoint, etc.)
→ Read [CONFIG.md](CONFIG.md#advanced-configuration)

---

## 📚 Document Relationships

```
CONFIG.md ◄──── EXAMPLES.md (see CONFIG for schema details)
```

---

## 💡 Quick Reference

### Configuration File Location
- **macOS/Linux:** `~/.config/zed/settings.json`
- **Windows:** `%APPDATA%\Zed\settings.json`

### Minimal Configuration
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

### Environment Variables
```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."
```

---

## 📊 Configuration Fields at a Glance

### Root Level
- `enabled` — Enable/disable extension (boolean)
- `provider` — Active provider: "openai" or "anthropic" (string)
- `openai` — OpenAI configuration (object)
- `anthropic` — Anthropic configuration (object)
- `chat` — Chat settings (object)

### OpenAI Configuration
- `api_key` — API key (required, use `${OPENAI_API_KEY}`)
- `model` — Model name (optional, default: "gpt-4o")
- `api_base` — API endpoint (optional, default: OpenAI's endpoint)
- `timeout_secs` — Request timeout (optional, default: 30)

### Anthropic Configuration
- `api_key` — API key (required, use `${ANTHROPIC_API_KEY}`)
- `model` — Model name (optional, default: "claude-opus-4-1-20250805")
- `api_base` — API endpoint (optional, default: Anthropic's endpoint)
- `timeout_secs` — Request timeout (optional, default: 30)

### Chat Configuration
- `streaming_enabled` — Enable streaming responses (optional, default: true)
- `max_history_messages` — Message history limit (optional, default: 50)
- `auto_scroll_to_latest` — Auto-scroll chat (optional, default: true)
- `context_window_size` — Context tokens (optional, default: 4096)

---

## ❓ FAQs

### Where do I find the configuration file?
See "Configuration File Location" above. Or use:
```bash
cat ~/.config/zed/settings.json  # macOS/Linux
type %APPDATA%\Zed\settings.json  # Windows
```

### What syntax should I use for API keys?
Always use environment variable syntax: `${OPENAI_API_KEY}`

Never hardcode keys like: `"api_key": "sk-..."`

See [CONFIG.md](CONFIG.md#environment-variable-interpolation) for details.

### What models are supported?

**OpenAI:** gpt-4o, o1, o3-mini, gpt-4-turbo

**Anthropic:** claude-opus-4-1-20250805, claude-sonnet-4-20250514, claude-haiku-4-5-20251001

See [../getting-started/EXAMPLES.md](../getting-started/EXAMPLES.md#comparison-when-to-use-what) for comparison.

### Can I use a custom API endpoint?

Yes! See [CONFIG.md](CONFIG.md#custom-api-endpoints).

```json
{
  "openai": {
    "api_base": "https://custom-endpoint.example.com/v1"
  }
}
```

### How do I switch between providers?

Change the `provider` field and restart Zed:

```json
{
  "provider": "anthropic"  // Change from "openai"
}
```

### Can I configure both providers?

Yes! Configure both and switch by changing `provider`:

See [CONFIG.md](CONFIG.md#complete-configuration-example) for example.

---

## 🔗 Related Documentation

- **[../getting-started/EXAMPLES.md](../getting-started/EXAMPLES.md)** — Configuration examples (start here!)
- **[../getting-started/SETUP.md](../getting-started/SETUP.md)** — Installation and troubleshooting
- **[../../README.md#-documentation](../../README.md#-documentation)** — Documentation index

---

## 💡 Tips

### For New Users
1. Start with [../getting-started/EXAMPLES.md](../getting-started/EXAMPLES.md)
2. Copy an example that matches your use case
3. Use [CONFIG.md](CONFIG.md) for fine-tuning

### For Advanced Users
1. Read [CONFIG.md](CONFIG.md) for complete schema
2. Check "Advanced Configuration" section
3. Review "Security Best Practices"

### For Troubleshooting
1. Check [CONFIG.md](CONFIG.md#troubleshooting)
2. Verify environment variables: `echo $OPENAI_API_KEY`
3. Validate JSON syntax
4. Check [../getting-started/SETUP.md](../getting-started/SETUP.md#configuration)

---

## 🚀 Next Steps

- **Ready to install?** → [../getting-started/QUICKSTART.md](../getting-started/QUICKSTART.md)
- **Need examples?** → [../getting-started/EXAMPLES.md](../getting-started/EXAMPLES.md)
- **Having issues?** → [../getting-started/SETUP.md](../getting-started/SETUP.md#troubleshooting)
- **Want more details?** → [CONFIG.md](CONFIG.md)

---

**Back to:** [Documentation Index](../../README.md#-documentation)