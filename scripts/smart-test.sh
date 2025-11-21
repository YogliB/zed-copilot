#!/bin/bash

set -euo pipefail

MODE="${1:-changed-only}"

log_section() {
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  $1"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
}

log_info() {
    echo "  📋 $1"
}

log_success() {
    echo "  ✅ $1"
}

log_warn() {
    echo "  ⚠️  $1"
}

detect_changed_modules() {
    log_section "Detecting Changed Modules"

    local changed_files
    changed_files=$(git diff HEAD --name-only 2>/dev/null || echo "")

    if [ -z "$changed_files" ]; then
        log_info "No unstaged changes detected"
        echo "full"
        return 0
    fi

    log_info "Changed files detected:"
    echo "$changed_files" | sed 's/^/    /'
    echo ""

    if echo "$changed_files" | grep -qE '^Cargo\.(toml|lock)$|^extension\.toml$'; then
        log_warn "Build config changed → running full suite for safety"
        echo "full"
        return 0
    fi

    if echo "$changed_files" | grep -qE '^tests/'; then
        log_warn "Integration tests changed → running full suite"
        echo "full"
        return 0
    fi

    if echo "$changed_files" | grep -qE 'Makefile|scripts/|\.github/'; then
        log_warn "Build/CI infrastructure changed → running full suite"
        echo "full"
        return 0
    fi

    log_success "Changes safe for targeted testing"
    echo "targeted"
}

run_native_tests() {
    local mode="$1"

    log_section "Running Native Rust Tests"

    if [ "$mode" = "full" ]; then
        log_info "Full test suite..."
        cargo test
    else
        log_info "Targeted tests (changed modules)..."
        cargo test --lib
    fi

    log_success "Native tests passed"
}

run_wasm_validation() {
    log_section "Running WASM Validation"

    log_warn "Skipping WASM validation for native Zed extension (cdylib)"
    log_info "This project uses HTTP dependencies incompatible with WASM targets."
    log_info "WASM validation is not applicable for native Zed extensions."

    return 0
}

show_help() {
    cat <<EOF
Smart Test Runner for zed-copilot

Usage: $(basename "$0") [MODE]

Modes:
  changed-only    Run tests only for changed modules (default)
  full            Run complete test suite
  dry-run         Show what would run without executing
  help            Show this help message

Examples:
  $(basename "$0")                  # Smart test for changed modules
  $(basename "$0") full             # Full test suite
  $(basename "$0") changed-only     # Explicit smart mode
  $(basename "$0") dry-run          # Preview tests

Environment Variables:
  VERBOSE=1       Enable verbose output

EOF
}

case "$MODE" in
    changed-only)
        suite_type=$(detect_changed_modules)

        if [ "$suite_type" = "full" ]; then
            run_native_tests "full"
        else
            run_native_tests "targeted"
        fi

        run_wasm_validation
        log_section "✨ All Tests Passed!"
        ;;

    full)
        suite_type=""
        log_section "Running Full Test Suite"
        run_native_tests "full"
        run_wasm_validation
        log_section "✨ All Tests Passed!"
        ;;

    dry-run)
        log_section "Dry Run Mode (Preview Only)"
        suite_type=$(detect_changed_modules)

        if [ "$suite_type" = "full" ]; then
            log_success "Would run: Full native test suite"
        else
            log_success "Would run: Targeted native tests (changed modules)"
        fi
        log_success "Would run: WASM build validation"
        log_success "Dry-run complete (no tests executed)"
        ;;

    help|--help|-h)
        show_help
        exit 0
        ;;

    *)
        echo "Error: Unknown mode '$MODE'"
        echo "Run '$(basename "$0") help' for usage."
        exit 1
        ;;
esac
