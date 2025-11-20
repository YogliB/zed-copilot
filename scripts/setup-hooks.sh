#!/bin/bash

set -euo pipefail

BOLD='\033[1m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
GIT_HOOKS_DIR="$PROJECT_ROOT/.git/hooks"

echo ""
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}  Git Hooks Setup${NC}"
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

if [[ ! -d "$GIT_HOOKS_DIR" ]]; then
    echo -e "${RED}✗ Error: Not in a git repository${NC}"
    echo -e "${YELLOW}  Expected .git/hooks directory at: $GIT_HOOKS_DIR${NC}"
    exit 1
fi

echo -e "${CYAN}Project root: $PROJECT_ROOT${NC}"
echo -e "${CYAN}Git hooks dir: $GIT_HOOKS_DIR${NC}"
echo ""

echo -e "${BOLD}Installing hooks...${NC}"
echo ""

if [[ ! -f "$SCRIPT_DIR/pre-commit" ]]; then
    echo -e "${RED}✗ Error: pre-commit hook template not found${NC}"
    exit 1
fi

if [[ ! -f "$SCRIPT_DIR/pre-push" ]]; then
    echo -e "${RED}✗ Error: pre-push hook template not found${NC}"
    exit 1
fi

echo -e "${CYAN}→ Installing pre-commit hook${NC}"
cp "$SCRIPT_DIR/pre-commit" "$GIT_HOOKS_DIR/pre-commit"
chmod +x "$GIT_HOOKS_DIR/pre-commit"
echo -e "${GREEN}  ✓ pre-commit installed${NC}"

echo -e "${CYAN}→ Installing pre-push hook${NC}"
cp "$SCRIPT_DIR/pre-push" "$GIT_HOOKS_DIR/pre-push"
chmod +x "$GIT_HOOKS_DIR/pre-push"
echo -e "${GREEN}  ✓ pre-push installed${NC}"

echo ""
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Hooks installed successfully!${NC}"
echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo -e "${YELLOW}What happens now:${NC}"
echo ""
echo -e "  ${CYAN}Pre-Commit Hook${NC}"
echo -e "    • Runs on: git commit"
echo -e "    • Checks: Code formatting (cargo fmt)"
echo -e "    • Checks: Linting (cargo clippy)"
echo -e "    • Time: ~5-10 seconds"
echo -e "    • Bypass: git commit --no-verify (not recommended)"
echo ""
echo -e "  ${CYAN}Pre-Push Hook${NC}"
echo -e "    • Runs on: git push"
echo -e "    • Checks: Tests for changed modules (smart mode)"
echo -e "    • Checks: WASM validation"
echo -e "    • Time: ~20-45 seconds"
echo -e "    • Bypass: git push --no-verify (not recommended)"
echo ""

echo -e "${YELLOW}Quick reference:${NC}"
echo ""
echo -e "  ${CYAN}scripts/smart-test.sh changed-only${NC}     Smart test for changed modules"
echo -e "  ${CYAN}scripts/smart-test.sh full${NC}             Run complete test suite"
echo -e "  ${CYAN}scripts/smart-test.sh dry-run${NC}          Preview what would run"
echo -e "  ${CYAN}scripts/smart-test.sh help${NC}             Show help"
echo ""

echo -e "${YELLOW}Next step:${NC}"
echo -e "  Try your first commit: ${CYAN}git add . && git commit -m 'test'${NC}"
echo ""
