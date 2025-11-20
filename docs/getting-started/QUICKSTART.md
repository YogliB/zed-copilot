# Quick Start — 5 Minutes to Running

Get Zed Copilot running in 5 minutes flat.

> **Part of:** [Zed Copilot Documentation](../../README.md#-documentation)

---

## Step 1: Prerequisites (2 min)

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify:
```bash
rustup --version
```

### Get an API Key

Choose one provider:

- **OpenAI:** https://platform.openai.com/api-keys
- **Anthropic:** https://console.anthropic.com/keys

Copy your key somewhere safe.

---

## Step 2: Install Extension (2 min)

```bash
# Clone repository
git clone https://github.com/zed-industries/zed-copilot.git
cd zed-copilot

# Build extension
cargo build --release
```

Expected output:
```
Finished release [optimized] target(s) in XX.XXs
```

---

## Step 3: Load in Zed (1 min)

1. Open Zed
2. Press `Cmd+Shift+P` (Mac) or `Ctrl+Shift+P` (Linux/Windows)
3. Type: `zed: open extensions`
4. Click **Install Dev Extension**
5. Select the `zed-copilot` folder
6. Wait for "Extension installed" message

---

## Step 4: Configure (<1 min)

### Set API Key

```bash
# For OpenAI
export OPENAI_API_KEY="sk-..."

# For Anthropic
export ANTHROPIC_API_KEY="sk-ant-..."
```

### Add to Zed Settings

Press `Cmd+,` (Mac) or `Ctrl+,` (Linux/Windows) and add:

**OpenAI:**
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

**Anthropic:**
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

Restart Zed.

---

## Step 5: Verify

Open Zed logs:
- Press `Cmd+Shift+P` / `Ctrl+Shift+P`
- Type: `zed: open log`
- Look for: `[Zed Copilot] Extension initialized`

✅ **Success!** Extension is loaded.

---

## What's Next?

- 📖 **[EXAMPLES.md](EXAMPLES.md)** — 13+ configuration examples
- 🔧 **[SETUP.md](SETUP.md)** — Troubleshooting and advanced setup
- 🏗️ **[development/DEVELOPMENT.md](../development/DEVELOPMENT.md)** — Architecture and development guide
- 🚀 **[development/ROADMAP.md](../development/ROADMAP.md)** — Upcoming features

---

## Common Issues

### "cargo: command not found"

Install Rust via rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Extension won't load

```bash
# Rebuild
cargo clean
cargo build --release

# Check Rust version
rustup --version
```

### API key errors

```bash
# Verify environment variable is set
echo $OPENAI_API_KEY

# Re-export if empty
export OPENAI_API_KEY="sk-..."
```

---

**Need more help?** See [SETUP.md](SETUP.md) for detailed troubleshooting.

---

**Back to:** [Documentation](../../README.md#-getting-started)