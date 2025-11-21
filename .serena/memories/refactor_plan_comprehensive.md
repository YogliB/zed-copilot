# Comprehensive LLM Provider Refactoring Plan

## Scope
Refactor both OpenAI and Anthropic providers to use official/well-maintained SDKs, leverage built-in retries, and add streaming support to the AiProvider trait.

## Libraries Selected

### OpenAI
- **Library**: async-openai v0.28+
- **URL**: https://github.com/64bit/async-openai
- **Reputation**: High (85.4/100)
- **Benefits**: Type-safe, official-grade, built-in retries

### Anthropic
- **Library**: anthropic_rust v0.1.3+ (OFFICIAL)
- **URL**: https://github.com/anthropics/anthropic-sdk-rust
- **Reputation**: Official Anthropic SDK
- **Benefits**: Type-safe, official, async-first, streaming support

## Implementation Tasks

### Task 1: Update Cargo.toml
- Add `async-openai = "0.28"`
- Add `anthropic_rust = "0.1"`
- Remove custom retry logic dependency (use built-in retries)

### Task 2: Refactor OpenAI Provider
- Update `src/http/openai.rs` to use async-openai::Client
- Remove manual payload/response handling (~35 LOC reduction)
- Map async-openai errors to ProviderError

### Task 3: Refactor Anthropic Provider
- Update `src/http/anthropic.rs` to use anthropic_rust::Client
- Remove manual payload/response handling (~35 LOC reduction)
- Map anthropic_rust errors to ProviderError

### Task 4: Enhance AiProvider Trait
- Add streaming support methods
- Add streaming examples
- Maintain backward compatibility with existing complete() method

### Task 5: Update Tests
- Remove payload/response parsing tests
- Add configuration tests
- Verify both providers with new clients

## Files to Modify
1. `zed-copilot/Cargo.toml`
2. `zed-copilot/src/http/openai.rs`
3. `zed-copilot/src/http/anthropic.rs`
4. `zed-copilot/src/providers/openai.rs` (minimal changes)
5. `zed-copilot/src/providers/anthropic.rs` (minimal changes)
6. `zed-copilot/src/providers/trait_def.rs` (add streaming)

## Estimated LOC Changes
- **Remove**: ~110 LOC (manual HTTP handling)
- **Add**: ~80 LOC (library usage + streaming)
- **Net**: ~30 LOC reduction + significant quality improvement
