# AI Provider Integration

## Overview

Zed Copilot uses a trait-based abstraction to support multiple AI providers. This design enables seamless provider switching, fallback mechanisms, and easy addition of new providers without modifying core logic.

## Architecture

### Core Trait

The `AiProvider` trait defines the interface that all providers must implement:

```rust
#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn complete(&self, prompt: &str) -> ProviderResult<String>;
    async fn is_available(&self) -> bool;
    fn name(&self) -> &str;
    fn model(&self) -> &str;
}
```

**Methods:**

- `complete(prompt)` — Sends a prompt to the provider and returns the completion result
- `is_available()` — Checks if the provider can be used (e.g., API key is present)
- `name()` — Returns the provider identifier (e.g., "openai", "anthropic")
- `model()` — Returns the configured model name

### Error Handling

All provider operations return `ProviderResult<T>`, a type alias for `Result<T, ProviderError>`:

```rust
pub enum ProviderError {
    ApiError(String),           // API request/response failures
    ConfigError(String),        // Configuration validation issues
    NetworkError(String),       // Network connectivity problems
    ParseError(String),         // Response parsing failures
    NotAvailable(String),       // Provider unavailable
}
```

## Supported Providers

### OpenAI

**Models:** `gpt-4`, `gpt-3.5-turbo`

**Features:**
- Chat completions API
- Configurable API base URL (for custom endpoints)
- Temperature and max tokens configuration

**Usage:**

```rust
use zed_copilot::providers::ProviderFactory;

let provider = ProviderFactory::create_openai(
    "sk-...".to_string(),
    "gpt-4".to_string()
)?;

let result = provider.complete("Write a function to sort arrays").await?;
println!("{}", result);
```

**With Custom API Base:**

```rust
let provider = ProviderFactory::create_openai_with_base(
    "sk-...".to_string(),
    "gpt-4".to_string(),
    "https://custom.openai.com/v1".to_string()
)?;
```

### Anthropic Claude

**Models:** `claude-3-opus`, `claude-3-sonnet`, `claude-3-haiku`

**Features:**
- Messages API
- Configurable API base URL (for custom endpoints)
- Max tokens configuration

**Usage:**

```rust
use zed_copilot::providers::ProviderFactory;

let provider = ProviderFactory::create_anthropic(
    "sk-ant-...".to_string(),
    "claude-3-sonnet".to_string()
)?;

let result = provider.complete("Explain async/await in Rust").await?;
println!("{}", result);
```

**With Custom API Base:**

```rust
let provider = ProviderFactory::create_anthropic_with_base(
    "sk-ant-...".to_string(),
    "claude-3-sonnet".to_string(),
    "https://custom.anthropic.com/v1".to_string()
)?;
```

## Provider Factory

The `ProviderFactory` provides convenient static methods for creating providers:

```rust
use zed_copilot::providers::ProviderFactory;

// Create OpenAI provider
let openai = ProviderFactory::create_openai(key, model)?;

// Create Anthropic provider
let anthropic = ProviderFactory::create_anthropic(key, model)?;

// Create with custom API base
let custom = ProviderFactory::create_openai_with_base(key, model, base)?;
```

## Implementation Guide

### Creating a New Provider

To add a new provider (e.g., Ollama, local LLM, etc.):

1. **Create provider file** — `src/providers/yourprovider.rs`

```rust
use crate::providers::error::{ProviderError, ProviderResult};
use crate::providers::trait_def::AiProvider;

pub struct YourProvider {
    api_key: String,
    model: String,
    api_base: String,
}

impl YourProvider {
    pub fn new(api_key: String, model: String) -> ProviderResult<Self> {
        if api_key.is_empty() {
            return Err(ProviderError::ConfigError(
                "API key cannot be empty".to_string(),
            ));
        }

        Ok(YourProvider {
            api_key,
            model,
            api_base: "https://api.yourprovider.com/v1".to_string(),
        })
    }
}

#[async_trait::async_trait]
impl AiProvider for YourProvider {
    async fn complete(&self, prompt: &str) -> ProviderResult<String> {
        // Implement API call logic
        todo!()
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    fn name(&self) -> &str {
        "yourprovider"
    }

    fn model(&self) -> &str {
        &self.model
    }
}
```

2. **Update module exports** — `src/providers/mod.rs`

```rust
pub mod yourprovider;
```

3. **Extend factory** — `src/providers/factory.rs`

```rust
pub fn create_yourprovider(
    api_key: String,
    model: String,
) -> ProviderResult<Box<dyn AiProvider>> {
    let provider = YourProvider::new(api_key, model)?;
    Ok(Box::new(provider))
}
```

4. **Add tests** — Include unit tests in the provider file

5. **Update documentation** — Add provider to this document

## Request/Response Format

### OpenAI

**Request:**
```json
{
  "model": "gpt-4",
  "messages": [
    {
      "role": "user",
      "content": "Your prompt here"
    }
  ],
  "temperature": 0.7,
  "max_tokens": 1024
}
```

**Response:**
```json
{
  "choices": [
    {
      "message": {
        "content": "The completion text"
      }
    }
  ]
}
```

### Anthropic

**Request:**
```json
{
  "model": "claude-3-sonnet",
  "messages": [
    {
      "role": "user",
      "content": "Your prompt here"
    }
  ],
  "max_tokens": 1024
}
```

**Response:**
```json
{
  "content": [
    {
      "text": "The completion text"
    }
  ]
}
```

## Error Handling Patterns

### Handle Provider Errors

```rust
match provider.complete("prompt").await {
    Ok(response) => println!("Success: {}", response),
    Err(ProviderError::ApiError(msg)) => eprintln!("API error: {}", msg),
    Err(ProviderError::ConfigError(msg)) => eprintln!("Config error: {}", msg),
    Err(ProviderError::NetworkError(msg)) => eprintln!("Network error: {}", msg),
    Err(ProviderError::ParseError(msg)) => eprintln!("Parse error: {}", msg),
    Err(ProviderError::NotAvailable(msg)) => eprintln!("Provider unavailable: {}", msg),
}
```

### Provider Fallback

```rust
let primary = ProviderFactory::create_openai(openai_key, "gpt-4")?;
let fallback = ProviderFactory::create_anthropic(anthropic_key, "claude-3-sonnet")?;

let result = match primary.complete(prompt).await {
    Ok(response) => response,
    Err(_) => fallback.complete(prompt).await?,
};
```

### Check Provider Availability

```rust
if provider.is_available().await {
    let result = provider.complete(prompt).await?;
} else {
    eprintln!("Provider {} is not available", provider.name());
}
```

## Configuration Strategy (Phase 2.2)

Providers will be configured in Zed settings:

```json
{
  "zed_copilot": {
    "enabled": true,
    "provider": "openai",
    "openai": {
      "api_key": "${OPENAI_API_KEY}",
      "model": "gpt-4",
      "api_base": "https://api.openai.com/v1"
    },
    "anthropic": {
      "api_key": "${ANTHROPIC_API_KEY}",
      "model": "claude-3-sonnet",
      "api_base": "https://api.anthropic.com/v1"
    }
  }
}
```

**Security notes:**
- API keys use environment variable interpolation
- No credentials are hardcoded
- Per-provider configuration for flexibility

## Testing

All providers include comprehensive unit tests:

```bash
cargo test --lib providers
```

**Test coverage includes:**
- Provider instantiation (valid/invalid inputs)
- Trait method implementations
- Error handling
- API payload structure
- Factory methods

## Performance Considerations

- Providers use async/await for non-blocking API calls
- Request payloads are built efficiently with `serde_json`
- Provider health is tracked via `is_available()`
- Future phases will add request caching and retry logic

## Logging & Observability

Providers emit structured logs:

- `[Provider] {name} initialized`
- `[{provider_name}] Completion request: {token_count} tokens`
- `[{provider_name}] Completion success: {response_length} chars`
- `[{provider_name}] Error: {error_message}`

## Future Enhancements

- **Local providers** — Ollama, local LLM via HTTP
- **Provider fallback** — Automatic provider switching on failure
- **Request caching** — Cache identical prompts to reduce API calls
- **Retry logic** — Exponential backoff for transient failures
- **Rate limiting** — Per-provider rate limit enforcement
- **Custom models** — Support for fine-tuned models
- **Streaming** — Streaming responses for long completions

## Related Documentation

- [ROADMAP.md](./ROADMAP.md) — Phase 2 overview
- [DEVELOPMENT.md](./DEVELOPMENT.md) — Development workflow