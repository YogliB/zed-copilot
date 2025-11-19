use crate::providers::error::{ProviderError, ProviderResult};
use crate::http::client::HttpClient;
use serde_json::{json, Value};
use std::time::Duration;

pub struct AnthropicHttpClient {
    http_client: HttpClient,
    api_base: String,
}

impl AnthropicHttpClient {
    pub fn new(api_base: String) -> ProviderResult<Self> {
        let http_client = HttpClient::new(Duration::from_secs(30), Default::default())?;
        Ok(AnthropicHttpClient {
            http_client,
            api_base,
        })
    }

    pub fn with_http_client(mut self, http_client: HttpClient) -> Self {
        self.http_client = http_client;
        self
    }

    pub async fn complete(&self, prompt: &str, model: &str, api_key: &str) -> ProviderResult<String> {
        let request_body = self.build_request_payload(prompt, model);
        let url = format!("{}/messages", self.api_base);

        let response_text = self
            .http_client
            .post(&url, request_body, api_key)
            .await?;

        self.parse_response(&response_text)
    }

    fn build_request_payload(&self, prompt: &str, model: &str) -> Value {
        json!({
            "model": model,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "max_tokens": 1024
        })
    }

    fn parse_response(&self, response_text: &str) -> ProviderResult<String> {
        let response: Value = serde_json::from_str(response_text)
            .map_err(|e| ProviderError::ParseError(format!("Failed to parse Anthropic response: {}", e)))?;

        response["content"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                ProviderError::ParseError("Missing text in Anthropic response".to_string())
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_http_client_new() {
        let result = AnthropicHttpClient::new("https://api.anthropic.com/v1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_anthropic_http_client_build_request_payload() {
        let client = AnthropicHttpClient::new("https://api.anthropic.com/v1".to_string()).unwrap();
        let payload = client.build_request_payload("test prompt", "claude-3-sonnet");

        assert_eq!(payload["model"], "claude-3-sonnet");
        assert_eq!(payload["messages"][0]["role"], "user");
        assert_eq!(payload["messages"][0]["content"], "test prompt");
        assert_eq!(payload["max_tokens"], 1024);
    }

    #[test]
    fn test_anthropic_http_client_parse_response_success() {
        let client = AnthropicHttpClient::new("https://api.anthropic.com/v1".to_string()).unwrap();
        let response = r#"{
            "content": [
                {
                    "type": "text",
                    "text": "This is a test response"
                }
            ]
        }"#;

        let result = client.parse_response(response);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "This is a test response");
    }

    #[test]
    fn test_anthropic_http_client_parse_response_missing_text() {
        let client = AnthropicHttpClient::new("https://api.anthropic.com/v1".to_string()).unwrap();
        let response = r#"{
            "content": [
                {
                    "type": "text"
                }
            ]
        }"#;

        let result = client.parse_response(response);
        assert!(result.is_err());
    }

    #[test]
    fn test_anthropic_http_client_parse_response_invalid_json() {
        let client = AnthropicHttpClient::new("https://api.anthropic.com/v1".to_string()).unwrap();
        let response = "invalid json";

        let result = client.parse_response(response);
        assert!(result.is_err());
    }

    #[test]
    fn test_anthropic_http_client_with_http_client() {
        let client = AnthropicHttpClient::new("https://api.anthropic.com/v1".to_string()).unwrap();
        let new_http_client = HttpClient::new(Duration::from_secs(60), Default::default()).unwrap();
        let updated = client.with_http_client(new_http_client);

        assert_eq!(updated.http_client.timeout(), Duration::from_secs(60));
    }
}
