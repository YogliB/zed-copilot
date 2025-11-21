# HTTP Integration Guide

Technical deep dive into HTTP client and streaming implementation for Zed Copilot.

> **Part of:** [Zed Copilot Documentation](../README.md)

## Overview

The HTTP integration layer in Phase 2.3 enables real API calls to AI providers (OpenAI and Anthropic). This document describes the architecture, configuration, and usage patterns.

## Architecture

### Component Structure

```
┌─────────────────────────────────────┐
│  AiProvider Trait (providers/*)     │
│  - OpenAiProvider                   │
│  - AnthropicProvider                │
│  - complete() & complete_stream()   │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│  HTTP Clients (http/*)              │
│  - OpenAiHttpClient                 │
│  - AnthropicHttpClient              │
│  - Streaming support via futures    │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│  HttpClient                         │
│  - Timeout handling                 │
│  - Retry with exponential backoff   │
│  - Error classification             │
│  - Streaming responses              │
└──────────────┬──────────────────────┘
               │
        ┌──────┴──────┬──────────────┐
        ▼             ▼              ▼
    RetryPolicy  RateLimiter   reqwest::Client
```</text>

### Key Components

#### HttpClient (`src/http/client.rs`)
Wrapper around `reqwest::Client` that handles:
- HTTP POST requests with authentication headers
- Request/response serialization
- Timeout configuration (default: 30 seconds)
- Retry orchestration with exponential backoff
- Error classification (transient vs. permanent)

#### RetryPolicy (`src/http/retry.rs`)
Implements exponential backoff with jitter:
- Base delay: 1000 ms
- Max delay: 32000 ms (32 seconds)
- Max retries: 3 attempts
- Formula: `delay = min(base * 2^(attempt-1), max_delay) * jitter(0.8-1.2)`
- Only retries on transient errors (network, timeouts, 5xx)

#### RateLimiter (`src/http/rate_limiter.rs`)
Token bucket algorithm for rate limiting:
- Tracks requests per minute per provider
- Prevents exceeding provider API rate limits
- Default limits:
  - OpenAI: 3500 requests/minute (RPM)
  - Anthropic: 1000 RPM
- Returns wait duration when rate limit approached

#### Provider HTTP Clients
- `OpenAiHttpClient` (`src/http/openai.rs`): Builds OpenAI chat completion requests
- `AnthropicHttpClient` (`src/http/anthropic.rs`): Builds Anthropic message requests

## Request/Response Flow

### OpenAI Chat Completion
```
Input: prompt = "What is Rust?"
        ↓
Build request:
{
  "model": "gpt-4",
  "messages": [{"role": "user", "content": "What is Rust?"}],
  "temperature": 0.7,
  "max_tokens": 1024
}
        ↓
POST https://api.openai.com/v1/chat/completions
Authorization: Bearer sk-...
        ↓
Response: {"choices": [{"message": {"content": "Rust is a systems programming language..."}}]}
        ↓
Parse & extract: "Rust is a systems programming language..."
```

### Anthropic Messages
```
Input: prompt = "What is Rust?"
        ↓
Build request:
{
  "model": "claude-3-sonnet",
  "messages": [{"role": "user", "content": "What is Rust?"}],
  "max_tokens": 1024
}
        ↓
POST https://api.anthropic.com/v1/messages
Authorization: Bearer sk-ant-...
        ↓
Response: {"content": [{"type": "text", "text": "Rust is a systems programming language..."}]}
        ↓
Parse & extract: "Rust is a systems programming language..."
```

## Configuration

### Timeout
- Default: 30 seconds
- Configurable via `HttpClient::new(duration, retry_policy)`
- Applies to entire request (including retries)

### Retry Policy
- Default: 3 retries with exponential backoff
- Configurable via `RetryPolicy::new(max_retries, base_delay_ms, max_delay_ms)`
- Only transient errors trigger retry:
  - Network errors (connection reset, timeout)
  - Server errors (5xx status codes)
- Permanent errors do NOT retry:
  - Authentication errors (401)
  - Client errors (4xx except timeouts)
  - Parse errors

### Rate Limiting
- Token bucket algorithm with per-provider tracking
- Configurable limits via `RateLimiter::new(requests_per_minute)`
- Pre-configured defaults for known providers

## Error Handling

### Error Types
All HTTP errors map to `ProviderError` variants:

```rust
pub enum ProviderError {
    ApiError(String),           // API-level errors (4xx, 5xx)
    ConfigError(String),        // Configuration issues
    NetworkError(String),       // Network problems (connectivity, timeout)
    ParseError(String),         // Response parsing failures
    NotAvailable(String),       // Provider unavailable
}
```

### Transient Error Detection
An error is considered transient if:
- It's a `NetworkError` variant, OR
- It's an `ApiError` containing "Server error" or "timeout"

Transient errors trigger automatic retry; others are returned immediately.

### Status Code Handling
- **2xx**: Success, parse response
- **401**: Non-transient, invalid API key
- **429**: Non-transient (rate limit), user should backoff
- **5xx**: Transient, triggers retry
- **4xx (other)**: Non-transient, client error

## Usage Examples

### Basic Provider Usage (Non-Streaming)
```rust
use zed_copilot::providers::openai::OpenAiProvider;
use zed_copilot::providers::trait_def::AiProvider;

// Create provider
let provider = OpenAiProvider::new(
    "sk-...".to_string(),
    "gpt-4".to_string()
)?;

// Call API (with automatic retry on transient errors)
let response = provider.complete("What is Rust?").await?;
println!("{}", response);
```

### Streaming Provider Usage
```rust
use zed_copilot::providers::openai::OpenAiProvider;
use zed_copilot::providers::trait_def::AiProvider;
use futures::StreamExt;

// Create provider
let provider = OpenAiProvider::new(
    "sk-...".to_string(),
    "gpt-4".to_string()
)?;

// Stream response tokens in real-time
let mut stream = provider.complete_stream("What is Rust?").await?;

while let Some(result) = stream.next().await {
    match result {
        Ok(token) => print!("{}", token),
        Err(e) => eprintln!("Stream error: {}", e),
    }
}
println!(); // Newline after stream completes
```

### Custom HTTP Client
```rust
use zed_copilot::http::{HttpClient, RetryPolicy};
use std::time::Duration;

// Create custom HTTP client with 60s timeout and 5 retries
let retry_policy = RetryPolicy::new(5, 1000, 32000);
let http_client = HttpClient::new(
    Duration::from_secs(60),
    retry_policy
)?;

// Use with provider
let provider = OpenAiProvider::new(
    "sk-...".to_string(),
    "gpt-4".to_string()
)?;
// Note: Providers currently use default HTTP client; 
// customization would require architectural changes
```

### Rate Limiting
```rust
use zed_copilot::http::RateLimiter;

let limiter = RateLimiter::new(100); // 100 requests/minute

// Check if request can proceed
if limiter.check_rate_limit("openai").await {
    // Make request
} else {
    // Back off and retry
}
```

## Testing

### Unit Tests
- HTTP client timeout and error handling
- Retry logic (backoff calculations, max retries)
- Rate limiter (token bucket, sliding window)
- Request/response building for both providers
- Response parsing (success and error cases)

### Integration Tests
- Mock HTTP server with `httpmock`
- Successful API calls
- Retry behavior on transient failures
- Rate limiting enforcement
- Error handling (4xx, 5xx, timeouts)

### Running Tests
```bash
# All tests (94 unit tests + 37 E2E tests)
cargo test

# Only unit tests
cargo test --lib

# Only HTTP unit tests
cargo test --lib http::

# Only E2E tests
cargo test --test '*'

# Specific test
cargo test --lib http::retry::tests::test_backoff_exponential_growth
```

## API Compatibility

### OpenAI
- **API Version**: v1 (Chat Completions API)
- **Base URL**: `https://api.openai.com/v1`
- **Endpoint**: `/chat/completions`
- **Models Tested**: gpt-4, gpt-4o, gpt-3.5-turbo
- **Documentation**: https://platform.openai.com/docs/api-reference/chat/create

### Anthropic Claude
- **API Version**: v1 (Messages API)
- **Base URL**: `https://api.anthropic.com/v1`
- **Endpoint**: `/messages`
- **Models Tested**: claude-3-sonnet, claude-3-opus
- **Documentation**: https://docs.anthropic.com/en/api/messages

## Streaming Support

### Current Implementation ✅

Both OpenAI and Anthropic providers support streaming responses via the `complete_stream()` method:

```rust
use zed_copilot::providers::openai::OpenAiProvider;
use zed_copilot::providers::trait_def::AiProvider;
use futures::StreamExt;

// Create provider
let provider = OpenAiProvider::new(
    "sk-...".to_string(),
    "gpt-4".to_string()
)?;

// Stream response tokens
let mut stream = provider.complete_stream("What is Rust?").await?;

while let Some(result) = stream.next().await {
    match result {
        Ok(token) => print!("{}", token),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**Features:**
- Real-time token delivery via `futures::Stream`
- Works with both OpenAI and Anthropic providers
- Automatic error handling in stream
- Compatible with async/await patterns

**Implementation Details:**
- OpenAI: Uses `create_stream()` from async-openai SDK
- Anthropic: Uses `stream_chat()` from anthropic_rust SDK
- Returns `Pin<Box<dyn Stream<Item = ProviderResult<String>> + Send>>`
- Handles partial content, deltas, and stream termination

### Configuration

Streaming can be enabled/disabled via chat configuration:

```json
{
  "chat": {
    "streaming_enabled": true
  }
}
```

Default: `true` (streaming enabled)

## Future Enhancements

### Phase 4+ (Additional Providers)
- GitHub Copilot LSP integration
- Local models via Ollama
- Self-hosted LLM services
- Custom/fine-tuned model support

### Performance Optimization
- Connection pooling (already via reqwest)
- Request batching for multiple completions
- Response caching for identical prompts
- Adaptive retry backoff based on provider patterns

## Security Considerations

### API Key Handling
- **Never log API keys**: Keys are masked in error messages
- **Environment variables**: Configuration uses env var interpolation
- **Bearer token format**: Added automatically in Authorization header
- **TLS/HTTPS**: All requests use HTTPS (enforced by api_base URLs)

### Request Validation
- Empty prompt detection prevents unnecessary API calls
- API key validation on provider initialization
- Response validation ensures required fields exist

### Error Messages
- Sensitive fields (API keys, full response bodies) not logged
- Timeout and network errors provide context without exposing data
- Parse errors include field names but not values

## Troubleshooting

### Request Timeout
- **Symptom**: `NetworkError("Request timeout")`
- **Cause**: Network latency or provider slow response
- **Solution**: Increase timeout via `HttpClient::new()`, or check provider status

### Rate Limited
- **Symptom**: `ApiError("Rate limited")`
- **Cause**: Exceeded provider API rate limits
- **Solution**: Reduce request frequency, use rate limiter, or upgrade API plan

### Invalid API Key
- **Symptom**: `ApiError("Unauthorized: Invalid API key")`
- **Cause**: Missing, incorrect, or expired API key
- **Solution**: Verify key in configuration, check provider dashboard

### Parsing Error
- **Symptom**: `ParseError("Missing 'content' in response")`
- **Cause**: Provider API changed or returned unexpected format
- **Solution**: Check provider API docs, verify model is supported

### Connection Error
- **Symptom**: `NetworkError("Connection error")`
- **Cause**: Network connectivity issue
- **Solution**: Check internet connection, firewall rules, proxy settings