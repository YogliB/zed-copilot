use crate::http::retry::RetryPolicy;
use crate::providers::error::{ProviderError, ProviderResult};
use reqwest::Client;
use std::time::Duration;

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    timeout: Duration,
    retry_policy: RetryPolicy,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            timeout: Duration::from_secs(30),
            retry_policy: RetryPolicy::default(),
        }
    }
}

impl HttpClient {
    pub fn new(timeout: Duration, retry_policy: RetryPolicy) -> ProviderResult<Self> {
        let client = Client::builder().build().map_err(|e| {
            ProviderError::NetworkError(format!("Failed to build HTTP client: {}", e))
        })?;

        Ok(HttpClient {
            client,
            timeout,
            retry_policy,
        })
    }

    pub fn with_retry_policy(mut self, retry_policy: RetryPolicy) -> Self {
        self.retry_policy = retry_policy;
        self
    }

    pub async fn post(
        &self,
        url: &str,
        body: serde_json::Value,
        api_key: &str,
    ) -> ProviderResult<String> {
        let mut attempt = 0;

        loop {
            match self.execute_post_request(url, &body, api_key).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    let is_transient = is_transient_error(&e);

                    if self.retry_policy.should_retry(attempt, is_transient) {
                        let backoff = self.retry_policy.calculate_backoff(attempt);
                        tokio::time::sleep(backoff).await;
                        attempt += 1;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    async fn execute_post_request(
        &self,
        url: &str,
        body: &serde_json::Value,
        api_key: &str,
    ) -> ProviderResult<String> {
        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    ProviderError::NetworkError("Request timeout".to_string())
                } else {
                    ProviderError::NetworkError(format!("HTTP request failed: {}", e))
                }
            })?;

        let status = response.status();

        if status.is_success() {
            response.text().await.map_err(|e| {
                ProviderError::NetworkError(format!("Failed to read response body: {}", e))
            })
        } else if status.is_server_error() {
            Err(ProviderError::ApiError(format!("Server error: {}", status)))
        } else if status.is_client_error() {
            Err(ProviderError::ApiError(format!("Client error: {}", status)))
        } else {
            Err(ProviderError::ApiError(format!(
                "Unexpected status code: {}",
                status
            )))
        }
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    pub fn retry_policy(&self) -> &RetryPolicy {
        &self.retry_policy
    }
}

fn is_transient_error(error: &ProviderError) -> bool {
    match error {
        ProviderError::NetworkError(_) => true,
        ProviderError::ApiError(msg) => msg.contains("Server error") || msg.contains("timeout"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_new_valid() {
        let result = HttpClient::new(Duration::from_secs(30), RetryPolicy::default());
        assert!(result.is_ok());
    }

    #[test]
    fn test_http_client_default() {
        let client = HttpClient::default();
        assert_eq!(client.timeout(), Duration::from_secs(30));
    }

    #[test]
    fn test_http_client_with_retry_policy() {
        let policy = RetryPolicy::new(5, 500, 16000);
        let client = HttpClient::default().with_retry_policy(policy.clone());

        assert_eq!(client.retry_policy().max_retries(), 5);
    }

    #[test]
    fn test_is_transient_error_network() {
        let error = ProviderError::NetworkError("Connection reset".to_string());
        assert!(is_transient_error(&error));
    }

    #[test]
    fn test_is_transient_error_server() {
        let error = ProviderError::ApiError("Server error: 500".to_string());
        assert!(is_transient_error(&error));
    }

    #[test]
    fn test_is_transient_error_timeout() {
        let error = ProviderError::NetworkError("Request timeout".to_string());
        assert!(is_transient_error(&error));
    }

    #[test]
    fn test_is_transient_error_client_error() {
        let error = ProviderError::ApiError("Client error: 400".to_string());
        assert!(!is_transient_error(&error));
    }

    #[test]
    fn test_is_transient_error_not_found() {
        let error = ProviderError::ApiError("Client error: 404".to_string());
        assert!(!is_transient_error(&error));
    }
}
