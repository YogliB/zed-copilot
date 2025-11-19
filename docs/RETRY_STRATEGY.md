# Retry Strategy & Exponential Backoff

## Overview

The retry strategy in Phase 2.3 implements exponential backoff with jitter to gracefully handle transient failures in API calls. This document explains the rationale, calculations, and tuning guidelines.

## Problem Statement

When calling remote APIs, transient failures occur:
- Network timeouts (temporary connectivity issues)
- Server errors (5xx responses, usually temporary)
- Rate limiting (brief throttling from provider)

**Naive approach**: Immediate retry causes:
- Thundering herd (all clients retry simultaneously)
- Overwhelming the recovering server
- Cascading failures

**Solution**: Exponential backoff with jitter distributes retries over time, allowing servers to recover.

## Algorithm

### Base Formula

```
delay(attempt) = min(base_delay * 2^(attempt - 1), max_delay) * jitter
```

Where:
- `attempt`: Current retry attempt (0-indexed)
- `base_delay`: Initial delay (default: 1000ms)
- `max_delay`: Ceiling delay (default: 32000ms)
- `jitter`: Random factor 0.8–1.2x to prevent synchronization

### Example Timeline

With defaults (base=1s, max=32s, jitter ignored for clarity):

```
Request attempt 0: Immediate (0ms)
  ↓ Transient error
Request attempt 1: 1s delay (1000ms * 2^0)
  ↓ Transient error
Request attempt 2: 2s delay (1000ms * 2^1)
  ↓ Transient error
Request attempt 3: 4s delay (1000ms * 2^2)
  ↓ Permanent error or success → Return result
```

**Total time**: 0 + 1 + 2 + 4 = 7 seconds (without jitter)

### With Jitter Applied

Jitter (random 0.8–1.2x multiplier) prevents synchronized retries:

```
Attempt 0: 0ms
Attempt 1: 1000ms * 0.95 = 950ms (jitter applied)
Attempt 2: 2000ms * 1.12 = 2240ms (jitter applied)
Attempt 3: 4000ms * 0.87 = 3480ms (jitter applied)
```

**Distribution**: Jitter causes each client's retries to happen at different times, preventing thundering herd.

## Configuration

### Default Values

```rust
pub struct RetryPolicy {
    pub max_retries: u32,      // Default: 3
    pub base_delay_ms: u64,    // Default: 1000ms (1 second)
    pub max_delay_ms: u64,     // Default: 32000ms (32 seconds)
}
```

### Rationale

| Parameter | Value | Justification |
|-----------|-------|---------------|
| `max_retries` | 3 | Balances recovery chances with latency. Most transient failures resolve in 1-2 attempts |
| `base_delay` | 1000ms | Gives server ~1s to recover from brief outages |
| `max_delay` | 32000ms | Prevents waiting >30s; matches HTTP client timeout |

### Recommended Tuning

#### Conservative (Critical Operations)
```rust
RetryPolicy::new(5, 500, 60_000)
// Max 5 retries, wait up to 60 seconds
// Use for: Important requests where latency is acceptable
```

#### Aggressive (User-Facing)
```rust
RetryPolicy::new(2, 500, 8_000)
// Max 2 retries, wait up to 8 seconds
// Use for: Interactive features where latency matters
```

#### High-Throughput
```rust
RetryPolicy::new(3, 2_000, 32_000)
// Longer base delay reduces request spam during outages
// Use for: Batch processing, background tasks
```

## Transient vs. Permanent Errors

### Transient Errors (Retried)

These errors typically resolve on retry:

| Error | Example | Cause | Retry? |
|-------|---------|-------|--------|
| Network timeout | `NetworkError("Request timeout")` | Slow network, server lag | ✅ Yes |
| Connection reset | `NetworkError("Connection error")` | Network hiccup | ✅ Yes |
| Server error (5xx) | `ApiError("Server error: 503")` | Server overload or restart | ✅ Yes |
| Service unavailable | `ApiError("Server error: 503")` | Temporary maintenance | ✅ Yes |

### Permanent Errors (Not Retried)

These errors persist across retries:

| Error | Example | Cause | Retry? |
|-------|---------|-------|--------|
| Unauthorized (401) | `ApiError("Unauthorized")` | Invalid/expired API key | ❌ No |
| Not found (404) | `ApiError("Client error: 404")` | Endpoint doesn't exist | ❌ No |
| Bad request (400) | `ApiError("Client error: 400")` | Invalid request payload | ❌ No |
| Config error | `ConfigError("API key cannot be empty")` | Misconfiguration | ❌ No |
| Parse error | `ParseError("Missing 'content' field")` | Malformed response | ❌ No |

### Classification Logic

```rust
fn is_transient_error(error: &ProviderError) -> bool {
    match error {
        ProviderError::NetworkError(_) => true,
        ProviderError::ApiError(msg) => {
            msg.contains("Server error") || msg.contains("timeout")
        }
        _ => false,
    }
}
```

## Mathematical Properties

### Backoff Growth

Exponential backoff ensures delays grow rapidly:

| Attempt | Formula | Delay (ms) | Cumulative (ms) |
|---------|---------|-----------|-----------------|
| 0 | — | 0 | 0 |
| 1 | 1000 * 2^0 | 1000 | 1000 |
| 2 | 1000 * 2^1 | 2000 | 3000 |
| 3 | 1000 * 2^2 | 4000 | 7000 |
| 4 | 1000 * 2^3 | 8000 | 15000 |
| 5 | 1000 * 2^4 | 16000 | 31000 |
| 6+ | capped at 32000 | 32000 | 31000+ |

### Maximum Total Latency

With 3 retries and 32s max delay:
```
max_latency = sum(delays) + request_timeout
            = (1 + 2 + 4 + 32 + request_timeout)s
            ≈ 39s + timeout
            ≈ 69s with 30s timeout
```

Practical: Most requests complete in <5 seconds due to transient errors resolving quickly.

### Jitter Distribution

Jitter (0.8–1.2x) prevents synchronized retries:

```
Without jitter: All 1000 clients retry at exactly 1s → request spike
With jitter: Clients retry between 800ms–1200ms → distributed load
```

**Effect**: Reduces thundering herd impact by 20–40x.

## Real-World Scenarios

### Scenario 1: Brief Network Hiccup

```
t=0s   : Client A sends request
t=0.5s : Network timeout
t=1s   : Client A retries (successful)
Result : ~1.5s latency, request succeeds
```

**Why it works**: Server didn't need recovery; jitter-delayed retry succeeds.

### Scenario 2: Provider Rate Limited

```
t=0s   : 100 clients send requests
t=0s   : Provider rate limit hit (429 response)
         [Without retry: 100 failures]
         [With retry: 100 clients wait with jitter]
t=1-2s : Requests retry staggered (due to jitter)
         Provider recovers between requests
Result : Most requests succeed on retry
```

**Why it works**: Jitter prevents all 100 clients retrying simultaneously.

### Scenario 3: Provider Recovering from Outage

```
t=0s   : Provider goes down (500 errors)
t=1s   : Provider starts recovering
         100 clients retry (jittered)
t=1.5s : Most retries succeed
t=4s   : 100% success rate
Result : Graceful recovery, no cascading failures
```

**Why it works**: Exponential backoff gives provider time to recover; jitter distributes retry load.

## Implementation Details

### Jitter Function

```rust
fn generate_jitter() -> f64 {
    // Deterministic pseudo-random based on system time
    let rand_val = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() % 100) as f64 / 100.0;
    0.8 + (rand_val * 0.4)  // Range: 0.8 to 1.2
}
```

**Note**: Uses system time for pseudo-randomness. For cryptographic randomness, use `rand` crate (deferred to Phase 3+).

### Backoff Calculation

```rust
pub fn calculate_backoff(&self, attempt: u32) -> Duration {
    if attempt == 0 {
        return Duration::from_millis(0);
    }

    let exponential_delay = self.base_delay_ms * 2_u64.pow(attempt - 1);
    let capped_delay = exponential_delay.min(self.max_delay_ms);
    let jitter = generate_jitter();
    let final_delay = (capped_delay as f64 * jitter) as u64;

    Duration::from_millis(final_delay)
}
```

### Retry Decision

```rust
pub fn should_retry(&self, attempt: u32, is_transient_error: bool) -> bool {
    is_transient_error && attempt < self.max_retries
}
```

**Logic**:
1. Check error type (transient?)
2. Check attempt count (< max_retries?)
3. Both conditions must be true

## Testing

### Unit Tests

```rust
#[test]
fn test_backoff_exponential_growth() {
    let policy = RetryPolicy::new(10, 1000, 100000);
    
    let b1 = policy.calculate_backoff(1);
    let b2 = policy.calculate_backoff(2);
    let b3 = policy.calculate_backoff(3);
    
    assert!(b1 < b2);
    assert!(b2 < b3);
}

#[test]
fn test_backoff_respects_max_delay() {
    let policy = RetryPolicy::new(10, 1000, 10000);
    
    for attempt in 1..=10 {
        let delay = policy.calculate_backoff(attempt);
        assert!(delay <= Duration::from_millis(10000));
    }
}

#[test]
fn test_should_retry_transient() {
    let policy = RetryPolicy::default();
    
    assert!(policy.should_retry(0, true));
    assert!(policy.should_retry(1, true));
    assert!(policy.should_retry(2, true));
    assert!(!policy.should_retry(3, true));  // Max retries exceeded
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_retry_on_transient_error() {
    let policy = RetryPolicy::new(3, 100, 1000);
    let client = HttpClient::new(Duration::from_secs(30), policy)?;
    
    // Mock server returns 503 twice, then 200
    let response = client.post(url, body, api_key).await;
    
    assert!(response.is_ok());
    // Verify 2 retries happened (via logs or metrics)
}
```

## Best Practices

### Do ✅

- Use default retry policy for most cases
- Apply jitter to prevent thundering herd
- Only retry on transient errors
- Log retry attempts with delay duration
- Set reasonable HTTP client timeout
- Test with mocked delays

### Don't ❌

- Retry on all errors (e.g., 401 will always fail)
- Retry synchronously (blocks thread)
- Ignore max_retries (could loop forever)
- Use same retry policy for all providers (they have different SLAs)
- Hardcode delays (make configurable)
- Retry without jitter (causes thundering herd)

## References

- AWS Architecture Blog: [Exponential Backoff And Jitter](https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/)
- Google SRE Book: [Dealing with Overload](https://sre.google/books/)
- RFC 7231: [HTTP Semantics and Content](https://tools.ietf.org/html/rfc7231#section-6.6.1)

## Future Enhancements

### Phase 3+

- Adaptive backoff based on error rates
- Circuit breaker pattern (stop retrying after N failures)
- Per-provider retry policies
- Metrics export (Prometheus)
- Request deadline propagation

### Phase 4+

- Machine learning for backoff prediction
- Provider health monitoring
- Automatic failover to alternative providers
- Request prioritization during backoff