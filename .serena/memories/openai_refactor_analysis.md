# OpenAI Refactoring Analysis - Context7 Reviewed

## Library Information
- **Library**: async-openai (Unofficial Rust OpenAI Client)
- **Source**: https://github.com/64bit/async-openai
- **Reputation**: High
- **Benchmark Score**: 85.4
- **Code Snippets**: 36 available examples
- **Latest Version**: Currently at least 0.28+ (recommended)
- **Context7 ID**: /64bit/async-openai

## Current Implementation (zed-copilot)

### Files to Refactor
1. `zed-copilot/src/http/openai.rs` (Custom HTTP layer - 55 lines)
2. `zed-copilot/src/providers/openai.rs` (Provider trait impl - 95 lines)
3. `zed-copilot/Cargo.toml` (Add dependency)

### Current Architecture
- **OpenAiHttpClient**: Manual reqwest client, hand-built JSON payloads
  - `build_request_payload()` → Manual JSON construction
  - `parse_response()` → Manual JSON parsing
  - `post()` → Custom retry loop with RetryPolicy
  
- **OpenAiProvider**: Implements `AiProvider` trait
  - Delegates HTTP to OpenAiHttpClient
  - Handles validation (api_key, model)
  - Config: custom API base support
  
- **Error Handling**: Custom ProviderError enum (5 variants)
- **Configuration**: Temperature 0.7, max_tokens 1024

### Current Features to Preserve
✓ Custom API base URL support (`with_api_base`)
✓ Retry policy integration (max_retries, backoff calculation)
✓ HTTP timeout (30 seconds default)
✓ Model parameter validation
✓ API key validation
✓ Temperature (0.7) and max_tokens (1024) settings

## async-openai Library Capabilities (Context7)

### Configuration & Initialization
- **OpenAIConfig**: Fluent builder API
  - `.with_api_key()`
  - `.with_org_id()`
  - `.with_project_id()`
  - `.with_api_base()` ✓ Custom base URL supported!
  
- **Client Creation**:
  ```rust
  let config = OpenAIConfig::new()
    .with_api_key("sk-...")
    .with_api_base("https://custom.com/v1");
  let client = Client::with_config(config);
  ```

- **Custom HTTP Client Support**:
  ```rust
  let http_client = reqwest::ClientBuilder::new()
    .timeout(Duration::from_secs(30))
    .build()?;
  let client = Client::new().with_http_client(http_client);
  ```
  
### Chat Completions API
- **Request Building**: Type-safe with `CreateChatCompletionRequestArgs`
  - `.model()` - supports any model
  - `.messages()` - accepts message builders
  - `.temperature()` - Option<f32>
  - `.max_tokens()` - Option<u32>
  - `.build()` - returns Result (validation)
  
- **Response Types**:
  - `CreateChatCompletionResponse` with strongly-typed choices
  - `.choices[0].message.content` - Type-safe access
  
- **Error Handling**:
  - async_openai has its own error types (need investigation)
  - Implements std::error::Error trait
  - Maps to APIError, APIRequestError, etc.

### Advanced Features
- Streaming support: `.create_stream()` method
- BYOT (Bring Your Own Types): For flexible responses
- Multiple model support: Azure OpenAI compatible
- Built-in type safety

## Key Differences & Migration Path

| Aspect | Current | async-openai | Migration |
|--------|---------|---|---|
| HTTP | Manual reqwest | Abstracted (reqwest internally) | Remove HTTP layer |
| Request Build | `serde_json::json!()` | Type-safe builders | Replace with RequestArgs |
| Response Parse | Manual string parse | Typed deserialiation | Direct field access |
| Config | Hardcoded/manual | OpenAIConfig struct | Use config builder |
| Retry | Custom loop in HttpClient | Possible built-in support | Verify/adapt |
| Error Types | ProviderError enum | async_openai error types | Map errors |
| Custom Base URL | ✓ Supported | ✓ Supported via config | Keep as feature |

## Implementation Phases - Revised

### Phase 1: Dependency Setup
**File**: `Cargo.toml`
- Add: `async-openai = "0.28"` (or latest)
- Keep: `reqwest` (transitive), `tokio`, `serde_json`
- Remove: Manual retry logic dependency can stay in HttpClient for now

### Phase 2: Refactor OpenAiHttpClient
**File**: `src/http/openai.rs`
- Remove: `build_request_payload()`, `parse_response()`
- Add: OpenAIConfig initialization with custom base URL support
- Keep: Retry logic (wrap async-openai client)
- Result: Thin wrapper ~20-30 lines vs current ~55

### Phase 3: Update OpenAiProvider  
**File**: `src/providers/openai.rs`
- Change: Constructor creates async_openai::Client via OpenAiHttpClient
- Update: `complete()` method uses CreateChatCompletionRequestArgs
- Update: Error mapping from async-openai error types
- Keep: trait interface (AiProvider), validation, feature parity
- Result: ~70-80 lines vs current ~95

### Phase 4: Error Mapping
- Map `async_openai::error` to ProviderError
- Preserve existing error variants where possible
- Add new variants if needed

### Phase 5: Testing
- Update mocks/tests for new request/response types
- Keep existing test coverage
- Verify custom API base still works
- Verify retry policy still functions

## Risk Assessment
- **Complexity**: 5 TODOs, Depth 2, Low Risk
- **Breaking Changes**: Internal only (AiProvider trait unchanged)
- **Test Coverage**: Need to mock async-openai responses
- **Benefits**: 40-50 fewer LOC, better type safety, fewer bugs

## Next Steps
1. Add async-openai to Cargo.toml
2. Create new OpenAiHttpClient using async-openai::Client
3. Refactor OpenAiProvider to use CreateChatCompletionRequestArgs
4. Map error types appropriately
5. Test and verify all features work
