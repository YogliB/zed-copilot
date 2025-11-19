use crate::providers::error::{ProviderError, ProviderResult};
use crate::providers::trait_def::AiProvider;

pub struct OpenAiProvider {
    api_key: String,
    model: String,
    api_base: String,
}

impl OpenAiProvider {
    pub fn new(api_key: String, model: String) -> ProviderResult<Self> {
        if api_key.is_empty() {
            return Err(ProviderError::ConfigError(
                "OpenAI API key cannot be empty".to_string(),
            ));
        }

        if model.is_empty() {
            return Err(ProviderError::ConfigError(
                "OpenAI model cannot be empty".to_string(),
            ));
        }

        Ok(OpenAiProvider {
            api_key,
            model,
            api_base: "https://api.openai.com/v1".to_string(),
        })
    }

    pub fn with_api_base(mut self, api_base: String) -> Self {
        self.api_base = api_base;
        self
    }

    fn build_request_payload(&self, prompt: &str) -> serde_json::Value {
        serde_json::json!({
            "model": self.model,
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
}

#[async_trait::async_trait]
impl AiProvider for OpenAiProvider {
    async fn complete(&self, prompt: &str) -> ProviderResult<String> {
        if prompt.is_empty() {
            return Err(ProviderError::ApiError(
                "Prompt cannot be empty".to_string(),
            ));
        }

        let _payload = self.build_request_payload(prompt);

        Err(ProviderError::ApiError(
            "HTTP client not yet implemented".to_string(),
        ))
    }

    async fn is_available(&self) -> bool {
        !self.api_key.is_empty()
    }

    fn name(&self) -> &str {
        "openai"
    }

    fn model(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_provider_new_valid() {
        let provider = OpenAiProvider::new(
            "sk-test-key".to_string(),
            "gpt-4".to_string(),
        );
        assert!(provider.is_ok());
    }

    #[test]
    fn test_openai_provider_new_empty_key() {
        let result = OpenAiProvider::new("".to_string(), "gpt-4".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_openai_provider_new_empty_model() {
        let result = OpenAiProvider::new("sk-test-key".to_string(), "".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_openai_provider_name() {
        let provider = OpenAiProvider::new(
            "sk-test-key".to_string(),
            "gpt-4".to_string(),
        )
        .unwrap();
        assert_eq!(provider.name(), "openai");
    }

    #[test]
    fn test_openai_provider_model() {
        let provider = OpenAiProvider::new(
            "sk-test-key".to_string(),
            "gpt-3.5-turbo".to_string(),
        )
        .unwrap();
        assert_eq!(provider.model(), "gpt-3.5-turbo");
    }

    #[test]
    fn test_openai_provider_with_api_base() {
        let provider = OpenAiProvider::new(
            "sk-test-key".to_string(),
            "gpt-4".to_string(),
        )
        .unwrap()
        .with_api_base("https://custom.openai.com/v1".to_string());
        assert_eq!(provider.api_base, "https://custom.openai.com/v1");
    }

    #[tokio::test]
    async fn test_openai_provider_is_available_with_key() {
        let provider = OpenAiProvider::new(
            "sk-test-key".to_string(),
            "gpt-4".to_string(),
        )
        .unwrap();
        assert!(provider.is_available().await);
    }

    #[tokio::test]
    async fn test_openai_provider_complete_empty_prompt() {
        let provider = OpenAiProvider::new(
            "sk-test-key".to_string(),
            "gpt-4".to_string(),
        )
        .unwrap();
        let result = provider.complete("").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_openai_build_request_payload() {
        let provider = OpenAiProvider::new(
            "sk-test-key".to_string(),
            "gpt-4".to_string(),
        )
        .unwrap();
        let payload = provider.build_request_payload("test prompt");
        assert_eq!(payload["model"], "gpt-4");
        assert_eq!(payload["messages"][0]["content"], "test prompt");
    }
}
