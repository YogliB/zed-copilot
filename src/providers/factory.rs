use crate::providers::anthropic::AnthropicProvider;
use crate::providers::error::ProviderResult;
use crate::providers::openai::OpenAiProvider;
use crate::providers::trait_def::AiProvider;

pub struct ProviderFactory;

impl ProviderFactory {
    pub fn create_openai(api_key: String, model: String) -> ProviderResult<Box<dyn AiProvider>> {
        let provider = OpenAiProvider::new(api_key, model)?;
        Ok(Box::new(provider))
    }

    pub fn create_anthropic(api_key: String, model: String) -> ProviderResult<Box<dyn AiProvider>> {
        let provider = AnthropicProvider::new(api_key, model)?;
        Ok(Box::new(provider))
    }

    pub fn create_openai_with_base(
        api_key: String,
        model: String,
        api_base: String,
    ) -> ProviderResult<Box<dyn AiProvider>> {
        let provider = OpenAiProvider::new(api_key, model)?.with_api_base(api_base)?;
        Ok(Box::new(provider))
    }

    pub fn create_anthropic_with_base(
        api_key: String,
        model: String,
        api_base: String,
    ) -> ProviderResult<Box<dyn AiProvider>> {
        let provider = AnthropicProvider::new(api_key, model)?.with_api_base(api_base)?;
        Ok(Box::new(provider))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_create_openai() {
        let result = ProviderFactory::create_openai("sk-test-key".to_string(), "gpt-4".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_factory_create_openai_invalid() {
        let result = ProviderFactory::create_openai("".to_string(), "gpt-4".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_factory_create_anthropic() {
        let result = ProviderFactory::create_anthropic(
            "sk-ant-test-key".to_string(),
            "claude-3-sonnet".to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_factory_create_anthropic_invalid() {
        let result =
            ProviderFactory::create_anthropic("".to_string(), "claude-3-sonnet".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_factory_create_openai_with_base() {
        let result = ProviderFactory::create_openai_with_base(
            "sk-test-key".to_string(),
            "gpt-4".to_string(),
            "https://custom.openai.com/v1".to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_factory_create_anthropic_with_base() {
        let result = ProviderFactory::create_anthropic_with_base(
            "sk-ant-test-key".to_string(),
            "claude-3-sonnet".to_string(),
            "https://custom.anthropic.com/v1".to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_factory_openai_provider_name() {
        let provider =
            ProviderFactory::create_openai("sk-test-key".to_string(), "gpt-4".to_string()).unwrap();
        assert_eq!(provider.name(), "openai");
    }

    #[test]
    fn test_factory_anthropic_provider_name() {
        let provider = ProviderFactory::create_anthropic(
            "sk-ant-test-key".to_string(),
            "claude-3-sonnet".to_string(),
        )
        .unwrap();
        assert_eq!(provider.name(), "anthropic");
    }
}
