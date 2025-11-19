use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RetryPolicy {
    max_retries: u32,
    base_delay_ms: u64,
    max_delay_ms: u64,
}

impl RetryPolicy {
    pub fn new(max_retries: u32, base_delay_ms: u64, max_delay_ms: u64) -> Self {
        RetryPolicy {
            max_retries,
            base_delay_ms,
            max_delay_ms,
        }
    }

    pub fn default_policy() -> Self {
        RetryPolicy {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 32000,
        }
    }

    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

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

    pub fn should_retry(&self, attempt: u32, is_transient_error: bool) -> bool {
        is_transient_error && attempt < self.max_retries
    }
}

fn generate_jitter() -> f64 {
    let rand_val = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() % 100) as f64 / 100.0;
    0.8 + (rand_val * 0.4)
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::default_policy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy_new() {
        let policy = RetryPolicy::new(5, 500, 16000);
        assert_eq!(policy.max_retries, 5);
        assert_eq!(policy.base_delay_ms, 500);
        assert_eq!(policy.max_delay_ms, 16000);
    }

    #[test]
    fn test_retry_policy_defaults() {
        let policy = RetryPolicy::default_policy();
        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.base_delay_ms, 1000);
        assert_eq!(policy.max_delay_ms, 32000);
    }

    #[test]
    fn test_backoff_attempt_zero() {
        let policy = RetryPolicy::default_policy();
        let backoff = policy.calculate_backoff(0);
        assert_eq!(backoff, Duration::from_millis(0));
    }

    #[test]
    fn test_backoff_exponential_growth() {
        let policy = RetryPolicy::new(10, 1000, 100000);

        let backoff_1 = policy.calculate_backoff(1);
        let backoff_2 = policy.calculate_backoff(2);
        let backoff_3 = policy.calculate_backoff(3);

        assert!(backoff_1.as_millis() > 0);
        assert!(backoff_2.as_millis() > backoff_1.as_millis());
        assert!(backoff_3.as_millis() > backoff_2.as_millis());
    }

    #[test]
    fn test_backoff_respects_max_delay() {
        let policy = RetryPolicy::new(10, 1000, 10000);

        let backoff_5 = policy.calculate_backoff(5);
        assert!(backoff_5.as_millis() <= 10000);

        let backoff_10 = policy.calculate_backoff(10);
        assert!(backoff_10.as_millis() <= 10000);
    }

    #[test]
    fn test_backoff_applies_jitter() {
        let policy = RetryPolicy::new(5, 1000, 32000);

        let backoff_1 = policy.calculate_backoff(1);
        let backoff_1_again = policy.calculate_backoff(1);

        assert!(backoff_1.as_millis() >= 800 && backoff_1.as_millis() <= 1200);
        assert!(
            backoff_1_again.as_millis() >= 800 && backoff_1_again.as_millis() <= 1200
        );
    }

    #[test]
    fn test_should_retry_transient_error() {
        let policy = RetryPolicy::default_policy();

        assert!(policy.should_retry(0, true));
        assert!(policy.should_retry(1, true));
        assert!(policy.should_retry(2, true));
        assert!(!policy.should_retry(3, true));
    }

    #[test]
    fn test_should_retry_non_transient_error() {
        let policy = RetryPolicy::default_policy();

        assert!(!policy.should_retry(0, false));
        assert!(!policy.should_retry(1, false));
        assert!(!policy.should_retry(2, false));
        assert!(!policy.should_retry(3, false));
    }

    #[test]
    fn test_should_retry_max_attempts() {
        let policy = RetryPolicy::new(2, 100, 1000);

        assert!(policy.should_retry(0, true));
        assert!(policy.should_retry(1, true));
        assert!(!policy.should_retry(2, true));
    }
}
