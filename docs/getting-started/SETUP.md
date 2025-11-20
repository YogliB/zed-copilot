# Setup Guide

Complete installation and environment setup for Zed Copilot.

> **Part of:** [Zed Copilot Documentation](../README.md)

---

## System Requirements

- **OS:** macOS 11+, Linux, or Windows with WSL2
- **Disk Space:** ~2GB free
- **Memory:** 4GB+ RAM recommended
- **Network:** Internet connection for API calls

---

## Prerequisites

### 1. Install Rust

Rust **must** be installed via rustup (not Homebrew or apt).

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows (WSL2):**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Verify:**
```bash
rustc --version
cargo --version
```

Expected: Version numbers for both commands.

### 2. Install Zed IDE

Download from [zed.dev](https://zed.dev/download)

**Verify:**
```bash
zed --version
```

### 3. Get API Key

Choose one provider and get an API key:

- **OpenAI:** https://platform.openai.com/api-keys
- **Anthropic:** https://console.anthropic.com/keys

📖 See [EXAMPLES.md](EXAMPLES.md) for configuration examples.

---

## Installation

### Clone Repository

```bash
git clone https://github.com/zed-industries/zed-copilot.git
cd zed-copilot
```

### Build Extension

```bash
cargo build --release
```

**Expected output:**
```
Finished release [optimized] target(s) in XX.XXs
```

**First build takes 2-5 minutes** as dependencies download.

### Install in Zed

1. Open Zed
2. Open Extensions panel: `Cmd+Shift+P` → `zed: open extensions`
3. Click **Install Dev Extension**
4. Select the `zed-copilot` directory
5. Wait for "Extension installed" confirmation

### Verify Installation

Open Zed logs: `Cmd+Shift+P` → `zed: open log`

Look for:
```
[Zed Copilot] Extension initialized
```

✅ Extension loaded successfully!

---

## Configuration

### Set Environment Variable

**macOS/Linux (Bash/Zsh):**
```bash
# Temporary (current session)
export OPENAI_API_KEY="sk-..."

# Permanent (add to ~/.zshrc or ~/.bashrc)
echo 'export OPENAI_API_KEY="sk-..."' >> ~/.zshrc
source ~/.zshrc
```

**Windows (PowerShell):**
```powershell
# Temporary
$env:OPENAI_API_KEY = "sk-..."

# Permanent (System Settings)
# Windows+X → System → Advanced → Environment Variables
```

**Verify:**
```bash
echo $OPENAI_API_KEY
```

### Configure Zed Settings

Open settings: `Cmd+,` (Mac) or `Ctrl+,` (Linux/Windows)

Add configuration (choose your provider):

**OpenAI (GPT-4o):**
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

**Anthropic (Claude Sonnet):**
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

**Restart Zed** for changes to take effect.

📖 See [EXAMPLES.md](EXAMPLES.md) for 13+ configuration examples and advanced options.

---

## Troubleshooting

### Rust Not Found

**Symptom:** `cargo: command not found`

**Solution:**
```bash
# Check if rustup is installed
which rustup

# If not found, install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify
cargo --version
```

**Make permanent:** Add to shell profile:
```bash
echo '. "$HOME/.cargo/env"' >> ~/.zshrc
```

### Build Fails

**Symptom:** `cargo build --release` shows errors

**Common causes:**

1. **Outdated Rust:** Update to latest stable:
   ```bash
   rustup update stable
   cargo --version
   ```

2. **Missing build tools:**
   - **macOS:** `xcode-select --install`
   - **Ubuntu:** `sudo apt-get install build-essential`
   - **Fedora:** `sudo dnf install gcc`

3. **Corrupted cache:** Clean and rebuild:
   ```bash
   cargo clean
   cargo build --release
   ```

4. **Network issues:** Dependencies can't download:
   ```bash
   cargo update
   cargo build --release
   ```

### Extension Won't Load

**Symptom:** Extension not in Extensions panel or shows error

**Solutions:**

1. **Verify Rust installation:**
   ```bash
   which rustup
   # Should show: /Users/yourname/.cargo/bin/rustup
   # NOT: /usr/local/bin/rustup (Homebrew - won't work)
   ```

2. **Check build succeeded:**
   ```bash
   cd /path/to/zed-copilot
   cargo build --release
   # Should show: "Finished release [optimized]"
   ```

3. **Check Zed logs:**
   - Open logs: `Cmd+Shift+P` → `zed: open log`
   - Look for errors mentioning "zed-copilot"
   - Common: WebAssembly loading errors

4. **Reinstall extension:**
   - Extensions panel → Remove dev extension
   - `cargo clean && cargo build --release`
   - Reinstall via Install Dev Extension

### API Key Not Recognized

**Symptom:** "Environment variable not found: OPENAI_API_KEY"

**Solutions:**

1. **Verify variable is set:**
   ```bash
   echo $OPENAI_API_KEY
   ```

2. **If empty, export it:**
   ```bash
   export OPENAI_API_KEY="sk-..."
   echo $OPENAI_API_KEY  # Should show your key
   ```

3. **Restart Zed** after setting variable

4. **Check settings.json syntax:**
   ```json
   "api_key": "${OPENAI_API_KEY}"
   ```
   Not: `"api_key": "$OPENAI_API_KEY"` (missing braces)

### Invalid Provider Error

**Symptom:** "Invalid provider 'gpt-4o'" or similar

**Problem:** Using model name instead of provider name

**Solution:**
```json
{
  "provider": "openai",      // Must be "openai" or "anthropic"
  "openai": {
    "model": "gpt-4o"        // Model goes here
  }
}
```

### Chat Panel Doesn't Appear

**This is expected!** Chat UI is coming in Phase 3 (Q2 2025).

Currently:
- Extension loads successfully
- No visible UI yet
- Configuration works
- Chat interface under development

See [ROADMAP.md](ROADMAP.md) for timeline.

### Zed Crashes on Load

**Symptom:** Zed closes when extension loads

**Solutions:**

1. **Check logs before crash:**
   ```bash
   zed --foreground
   # Watch for errors in terminal
   ```

2. **Try minimal build:**
   ```bash
   cargo clean
   cargo build --release --verbose
   ```

3. **Report issue:** Include:
   - Log output from `zed --foreground`
   - Rust version: `rustc --version`
   - OS version: `uname -a`
   - Error messages

### API Errors

**Symptom:** API calls fail or timeout

**Solutions:**

1. **Verify API key is valid:**
   - Log into provider dashboard
   - Check key is active and has credits

2. **Check model name:**
   - OpenAI: `gpt-4o`, `o1`, `o3-mini`
   - Anthropic: `claude-sonnet-4-20250514`, `claude-opus-4-1-20250805`

3. **Test API key manually:**
   ```bash
   # OpenAI
   curl https://api.openai.com/v1/models \
     -H "Authorization: Bearer $OPENAI_API_KEY"

   # Anthropic
   curl https://api.anthropic.com/v1/messages \
     -H "x-api-key: $ANTHROPIC_API_KEY" \
     -H "anthropic-version: 2023-06-01" \
     -H "content-type: application/json" \
     -d '{"model":"claude-sonnet-4-20250514","max_tokens":10,"messages":[{"role":"user","content":"Hi"}]}'
   ```

4. **Check network/firewall:**
   - Ensure outbound HTTPS is allowed
   - Check proxy settings if behind corporate firewall

---

## Advanced Setup

### Custom API Endpoint

Use custom endpoint (proxy, self-hosted):

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o",
      "api_base": "https://custom.example.com/v1"
    }
  }
}
```

### Multiple Providers

Configure both, switch by changing `provider`:

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",  // Change to "anthropic" to switch
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4o"
    },
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-sonnet-4-20250514"
    }
  }
}
```

### Debug Mode

Run Zed in foreground to see real-time logs:

```bash
zed --foreground
```

Press `Ctrl+C` to exit.

---

## Uninstallation

### Remove Extension

1. Open Extensions panel
2. Find "Zed Copilot"
3. Click Remove or Uninstall

### Remove Files

```bash
rm -rf ~/path/to/zed-copilot
```

### Remove Config

Edit `settings.json` and remove `zed_copilot` section.

### Remove Environment Variables

Edit shell profile (`~/.zshrc` or `~/.bashrc`):
```bash
# Remove lines like:
# export OPENAI_API_KEY="..."
```

Reload shell:
```bash
source ~/.zshrc
```

---

## Related Documentation

- **[QUICKSTART.md](QUICKSTART.md)** — 5-minute setup guide
- **[EXAMPLES.md](EXAMPLES.md)** — Configuration examples
- **[../reference/CONFIG.md](../reference/CONFIG.md)** — Configuration reference
- **[../development/DEVELOPMENT.md](../development/DEVELOPMENT.md)** — Architecture guide

---

## Getting Help

### Diagnostic Info

When reporting issues, include:

```bash
# System info
uname -a

# Rust info
rustc --version
cargo --version
rustup --version

# Zed info
zed --version

# Environment
echo $OPENAI_API_KEY  # First few chars only
```

### Resources

- **Documentation:** [README.md](../README.md)
- **Quick Start:** [QUICKSTART.md](QUICKSTART.md)
- **Examples:** [EXAMPLES.md](EXAMPLES.md)
- **Issues:** [GitHub Issues](https://github.com/zed-industries/zed-copilot/issues)
- **Discussions:** [GitHub Discussions](https://github.com/zed-industries/zed-copilot/discussions)

---

**Still stuck?** Open an issue with:
- What you tried
- Error messages
- Diagnostic info above
- Steps to reproduce

We're here to help! 🚀

---

**Back to:** [Getting Started](../README.md#quick-navigation)