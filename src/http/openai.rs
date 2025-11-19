use crate::providers::error::{ProviderError, ProviderResult};
use crate::http::client::HttpClient;
use serde_json::{json, Value};
use std::time::Duration;

pub struct OpenAiHttpClient {
    http_client: HttpClient,
    api_base: String,
}

impl OpenAiHttpClient {
    pub fn new(api_base: String) -> ProviderResult<Self> {
        let http_client = HttpClient::new(Duration::from_secs(30), Default::default())?;
        Ok(OpenAiHttpClient {
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
        let url = format!("{}/chat/completions", self.api_base);

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
            "temperature": 0.7,
            "max_tokens": 1024
        })
    }

    fn parse_response(&self, response_text: &str) -> ProviderResult<String> {
        let response: Value = serde_json::from_str(response_text)
            .map_err(|e| ProviderError::ParseError(format!("Failed to parse OpenAI response: {}", e)))?;

        response["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| {
                ProviderError::ParseError("Missing content in OpenAI response".to_string())
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_http_client_new() {
        let result = OpenAiHttpClient::new("https://api.openai.com/v1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_openai_http_client_build_request_payload() {
        let client = OpenAiHttpClient::new("https://api.openai.com/v1".to_string()).unwrap();
        let payload = client.build_request_payload("test prompt", "gpt-4");

        assert_eq!(payload["model"], "gpt-4");
        assert_eq!(payload["messages"][0]["role"], "user");
        assert_eq!(payload["messages"][0]["content"], "test prompt");
        assert_eq!(payload["temperature"], 0.7);
        assert_eq!(payload["max_tokens"], 1024);
    }

    #[test]
    fn test_openai_http_client_parse_response_success() {
        let client = OpenAiHttpClient::new("https://api.openai.com/v1".to_string()).unwrap();
        let response = r#"{
            "choices": [
                {
                    "message": {
                        "content": "This is a test response"
                    }
                }
            ]
        }"#;

        let result = client.parse_response(response);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "This is a test response");
    }

    #[test]
    fn test_openai_http_client_parse_response_missing_content() {
        let client = OpenAiHttpClient::new("https://api.openai.com/v1".to_string()).unwrap();
        let response = r#"{
            "choices": [
                {
                    "message": {}
                }
            ]
        }"#;

        let result = client.parse_response(response);
        assert!(result.is_err());
    }

    #[test]
    fn test_openai_http_client_parse_response_invalid_json() {
        let client = OpenAiHttpClient::new("https://api.openai.com/v1".to_string()).unwrap();
        let response = "invalid json";

        let result = client.parse_response(response);
        assert!(result.is_err());
    }

    #[test]
    fn test_openai_http_client_with_http_client() {
        let client = OpenAiHttpClient::new("https://api.openai.com/v1".to_string()).unwrap();
        let new_http_client = HttpClient::new(Duration::from_secs(60), Default::default()).unwrap();
        let updated = client.with_http_client(new_http_client);

        assert_eq!(updated.http_client.timeout(), Duration::from_secs(60));
    }
}
