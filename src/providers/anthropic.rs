use crate::http::anthropic::AnthropicHttpClient;
use crate::providers::error::{ProviderError, ProviderResult};
use crate::providers::trait_def::AiProvider;
use futures::Stream;
use std::pin::Pin;

pub struct AnthropicProvider {
    api_key: String,
    model: String,
    api_base: String,
    http_client: AnthropicHttpClient,
}

impl AnthropicProvider {
    pub fn new(api_key: String, model: String) -> ProviderResult<Self> {
        if api_key.is_empty() {
            return Err(ProviderError::ConfigError(
                "Anthropic API key cannot be empty".to_string(),
            ));
        }

        if model.is_empty() {
            return Err(ProviderError::ConfigError(
                "Anthropic model cannot be empty".to_string(),
            ));
        }

        let api_base = "https://api.anthropic.com/v1".to_string();
        let http_client = AnthropicHttpClient::new(api_base.clone())?;

        Ok(AnthropicProvider {
            api_key,
            model,
            api_base,
            http_client,
        })
    }

    pub fn with_api_base(mut self, api_base: String) -> ProviderResult<Self> {
        self.api_base = api_base.clone();
        self.http_client = AnthropicHttpClient::new(api_base)?;
        Ok(self)
    }
}

#[async_trait::async_trait(?Send)]
impl AiProvider for AnthropicProvider {
    async fn complete(&self, prompt: &str) -> ProviderResult<String> {
        if prompt.is_empty() {
            return Err(ProviderError::ApiError(
                "Prompt cannot be empty".to_string(),
            ));
        }

        self.http_client
            .complete(prompt, &self.model, &self.api_key)
            .await
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    fn name(&self) -> &str {
        "anthropic"
    }

    fn model(&self) -> &str {
        &self.model
    }

    async fn complete_stream(
        &self,
        prompt: &str,
    ) -> ProviderResult<Pin<Box<dyn Stream<Item = ProviderResult<String>> + Send>>> {
        if prompt.is_empty() {
            return Err(ProviderError::ApiError(
                "Prompt cannot be empty".to_string(),
            ));
        }

        self.http_client
            .complete_stream(prompt, &self.model, &self.api_key)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_provider_new_valid() {
        let provider =
            AnthropicProvider::new("sk-ant-test-key".to_string(), "claude-3-sonnet".to_string());
        assert!(provider.is_ok());
    }

    #[test]
    fn test_anthropic_provider_new_empty_key() {
        let result = AnthropicProvider::new("".to_string(), "claude-3-sonnet".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_anthropic_provider_new_empty_model() {
        let result = AnthropicProvider::new("sk-ant-test-key".to_string(), "".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_anthropic_provider_name() {
        let provider =
            AnthropicProvider::new("sk-ant-test-key".to_string(), "claude-3-sonnet".to_string())
                .unwrap();
        assert_eq!(provider.name(), "anthropic");
    }

    #[test]
    fn test_anthropic_provider_model() {
        let provider =
            AnthropicProvider::new("sk-ant-test-key".to_string(), "claude-3-opus".to_string())
                .unwrap();
        assert_eq!(provider.model(), "claude-3-opus");
    }

    #[test]
    fn test_anthropic_provider_with_api_base() {
        let provider =
            AnthropicProvider::new("sk-ant-test-key".to_string(), "claude-3-sonnet".to_string())
                .unwrap()
                .with_api_base("https://custom.anthropic.com/v1".to_string())
                .unwrap();
        assert_eq!(provider.api_base, "https://custom.anthropic.com/v1");
    }

    #[tokio::test]
    async fn test_anthropic_provider_is_available_with_key() {
        let provider =
            AnthropicProvider::new("sk-ant-test-key".to_string(), "claude-3-sonnet".to_string())
                .unwrap();
        assert!(provider.is_available().await);
    }

    #[tokio::test]
    async fn test_anthropic_provider_complete_empty_prompt() {
        let provider =
            AnthropicProvider::new("sk-ant-test-key".to_string(), "claude-3-sonnet".to_string())
                .unwrap();
        let result = provider.complete("").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_anthropic_build_request_payload() {
        let provider =
            AnthropicProvider::new("sk-ant-test-key".to_string(), "claude-3-sonnet".to_string())
                .unwrap();
        let payload = serde_json::json!({
            "model": provider.model(),
            "messages": [
                {
                    "role": "user",
                    "content": "test prompt"
                }
            ],
            "max_tokens": 1024
        });
        assert_eq!(payload["model"], "claude-3-sonnet");
        assert_eq!(payload["messages"][0]["content"], "test prompt");
    }
}
