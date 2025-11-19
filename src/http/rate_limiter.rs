use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<RateLimiterState>>,
    requests_per_minute: u32,
}

struct RateLimiterState {
    tokens: f64,
    last_refill: SystemTime,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32) -> Self {
        RateLimiter {
            state: Arc::new(Mutex::new(RateLimiterState {
                tokens: requests_per_minute as f64,
                last_refill: SystemTime::now(),
            })),
            requests_per_minute,
        }
    }

    pub fn default_openai() -> Self {
        RateLimiter::new(3500)
    }

    pub fn default_anthropic() -> Self {
        RateLimiter::new(1000)
    }

    pub async fn acquire(&self) -> Duration {
        let mut state = self.state.lock().await;

        let now = SystemTime::now();
        let elapsed = now
            .duration_since(state.last_refill)
            .unwrap_or(Duration::from_secs(0));

        let elapsed_minutes = elapsed.as_secs_f64() / 60.0;
        let tokens_to_add = elapsed_minutes * self.requests_per_minute as f64;

        state.tokens = (state.tokens + tokens_to_add).min(self.requests_per_minute as f64);
        state.last_refill = now;

        if state.tokens >= 1.0 {
            state.tokens -= 1.0;
            Duration::from_secs(0)
        } else {
            let tokens_needed = 1.0 - state.tokens;
            let wait_time_seconds = tokens_needed / (self.requests_per_minute as f64 / 60.0);
            Duration::from_secs_f64(wait_time_seconds)
        }
    }

    pub fn requests_per_minute(&self) -> u32 {
        self.requests_per_minute
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_new() {
        let limiter = RateLimiter::new(100);
        assert_eq!(limiter.requests_per_minute(), 100);
    }

    #[tokio::test]
    async fn test_rate_limiter_default_openai() {
        let limiter = RateLimiter::default_openai();
        assert_eq!(limiter.requests_per_minute(), 3500);
    }

    #[tokio::test]
    async fn test_rate_limiter_default_anthropic() {
        let limiter = RateLimiter::default_anthropic();
        assert_eq!(limiter.requests_per_minute(), 1000);
    }

    #[tokio::test]
    async fn test_acquire_no_wait() {
        let limiter = RateLimiter::new(100);
        let wait = limiter.acquire().await;
        assert_eq!(wait, Duration::from_secs(0));
    }

    #[tokio::test]
    async fn test_acquire_multiple_no_wait() {
        let limiter = RateLimiter::new(10);

        for _ in 0..10 {
            let wait = limiter.acquire().await;
            assert_eq!(wait, Duration::from_secs(0));
        }
    }

    #[tokio::test]
    async fn test_acquire_rate_limit_exceeded() {
        let limiter = RateLimiter::new(2);

        limiter.acquire().await;
        limiter.acquire().await;

        let wait = limiter.acquire().await;
        assert!(wait > Duration::from_secs(0));
    }

    #[tokio::test]
    async fn test_clone() {
        let limiter1 = RateLimiter::new(2);
        let limiter2 = limiter1.clone();

        let wait1 = limiter1.acquire().await;
        let wait2 = limiter1.acquire().await;
        let wait3 = limiter2.acquire().await;

        assert_eq!(wait1, Duration::from_secs(0));
        assert_eq!(wait2, Duration::from_secs(0));
        assert!(wait3 > Duration::from_secs(0));
    }
}
