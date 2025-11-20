# Git Hooks Quick Reference

## 🚀 Quick Start

```bash
# After cloning, install hooks (one-time)
bash scripts/setup-hooks.sh

# Then develop normally — hooks run automatically
git add .
git commit -m "Your message"    # Pre-commit validates (fmt + clippy)
git push                         # Pre-push validates (smart tests)
```

---

## 📋 What Happens

| Command | Hook | Checks | Time | If Fails |
|---------|------|--------|------|----------|
| `git commit` | pre-commit | Format, Linting | ~5-10s | ❌ Commit blocked |
| `git push` | pre-push | Smart tests, WASM | ~20-45s | ❌ Push blocked |

---

## 🎯 Smart Test Detection

**Pre-push hook automatically decides:**

| Change Type | Action |
|------------|--------|
| `src/module.rs` changed | Run targeted tests for that module |
| `Cargo.toml` changed | Run FULL test suite |
| `tests/` changed | Run FULL test suite |
| Build config changed | Run FULL test suite |

**Why?** Safety first — when dependencies change, test everything.

---

## 🔧 Manual Test Commands

Run anytime to test locally:

```bash
scripts/smart-test.sh              # Smart mode (changed modules)
scripts/smart-test.sh full         # Full test suite
scripts/smart-test.sh dry-run      # Preview without running
scripts/smart-test.sh help         # Show help
```

---

## ⏭️ When You Need to Bypass

```bash
# Skip pre-commit (not recommended)
git commit --no-verify

# Skip pre-push (strongly discouraged)
git push --no-verify

# Force full test suite locally first
scripts/smart-test.sh full && git push
```

⚠️ **Warning:** Bypassing tests may allow bugs to reach CI.

---

## 🐛 Troubleshooting

| Problem | Solution |
|---------|----------|
| Hooks not running | `bash scripts/setup-hooks.sh` |
| "smart-test.sh not found" | `chmod +x scripts/smart-test.sh` |
| WASM target missing | `rustup target add wasm32-unknown-unknown` |
| Tests fail in hook but pass locally | Run `bash .git/hooks/pre-push` manually to debug |

---

## 📚 Full Documentation

See **[docs/GIT_HOOKS.md](docs/GIT_HOOKS.md)** for:
- Detailed behavior explanation
- Real-world scenario examples
- Comprehensive troubleshooting
- Performance optimization tips
- Development guidance

---

## ✅ Typical Workflow

```bash
# 1. Clone and setup (one-time)
git clone <repo>
cd zed-copilot
bash scripts/setup-hooks.sh

# 2. Make changes
echo "code changes" > src/module.rs

# 3. Commit (pre-commit runs ~5 sec)
git add .
git commit -m "feat: describe change"
# ✓ Pre-commit passes: format + linting OK

# 4. Push (pre-push runs ~30 sec)
git push
# ✓ Smart detection: only src/module.rs changed
# ✓ Runs targeted tests for that module
# ✓ Validates WASM build
# ✓ Push succeeds!
```

---

## 🎯 Key Features

✅ **Fast iteration** — Commit frequently, pre-commit only checks format  
✅ **Test safety** — Pre-push validates before code reaches CI  
✅ **Smart filtering** — Only tests changed modules (unless dependencies change)  
✅ **Clear feedback** — Helpful error messages with next steps  
✅ **Always escapable** — Use `--no-verify` if absolutely necessary  

---

## 🔍 Hook Files Location

```
zed-copilot/
├── scripts/
│   ├── smart-test.sh        ← Core test detection
│   ├── pre-commit           ← Installed to .git/hooks/
│   ├── pre-push             ← Installed to .git/hooks/
│   └── setup-hooks.sh       ← Run once to install
└── docs/
    └── GIT_HOOKS.md         ← Full documentation
```

---

## 📊 Performance Notes

- Pre-commit: 5-10 seconds (very fast)
- Pre-push (changed-only): 20-35 seconds typical
- Pre-push (full suite): 30-45 seconds when deps change

**Optimization:** Commit frequently to keep changes small → faster tests

---

## 💡 Pro Tips

1. **Preview tests before pushing:** `scripts/smart-test.sh dry-run`
2. **Batch changes:** Group related changes → fewer pre-push runs
3. **Check what changed:** `git diff HEAD --name-only` (what pre-push sees)
4. **Force full suite:** `scripts/smart-test.sh full && git push`
5. **Verbose mode:** `VERBOSE=1 scripts/smart-test.sh changed-only`

---

## ❓ Common Questions

**Q: Can I commit without pre-commit?**  
A: Yes, but not recommended: `git commit --no-verify`

**Q: What if pre-push is too slow?**  
A: Commit more frequently (keeps changes small). Or use `--changed-only` mode (default).

**Q: Do hooks run in CI?**  
A: No, only locally. CI has its own checks.

**Q: Can I customize which tests run?**  
A: Yes, edit `scripts/smart-test.sh` detection logic.

**Q: What if a test fails?**  
A: Fix the bug, commit again, and push. Pre-push will run tests again.

---

## 🚀 Getting Help

- **Full guide:** [docs/GIT_HOOKS.md](docs/GIT_HOOKS.md)
- **Setup issues:** Check [GIT_HOOKS.md#troubleshooting](docs/GIT_HOOKS.md#troubleshooting)
- **Contributing:** [docs/development/CONTRIBUTING.md](docs/development/CONTRIBUTING.md)

---

**Last Updated:** 2025-01-09  
**Version:** 1.0 (Smart Changed-Only Mode)