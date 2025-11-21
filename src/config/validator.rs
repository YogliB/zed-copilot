use crate::config::errors::{ConfigError, ConfigResult};
use crate::config::structs::{AnthropicConfig, ChatConfig, OpenAiConfig, RootConfig};

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate(config: &RootConfig) -> ConfigResult<()> {
        if !config.enabled {
            return Ok(());
        }

        Self::validate_provider_selection(config)?;
        Self::validate_selected_provider(config)?;
        Self::validate_chat_config(&config.chat)?;

        Ok(())
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn validate_provider_selection(config: &RootConfig) -> ConfigResult<()> {
        if config.provider.is_empty() {
            return Err(ConfigError::MissingField(
                "provider field must be set to 'openai' or 'anthropic'".to_string(),
            ));
        }

        match config.provider.as_str() {
            "openai" | "anthropic" => Ok(()),
            invalid => Err(ConfigError::InvalidProvider(invalid.to_string())),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn validate_selected_provider(config: &RootConfig) -> ConfigResult<()> {
        match config.provider.as_str() {
            "openai" => Self::validate_openai_config(&config.openai),
            "anthropic" => Self::validate_anthropic_config(&config.anthropic),
            _ => Err(ConfigError::InvalidProvider(config.provider.clone())),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn validate_openai_config(config: &Option<OpenAiConfig>) -> ConfigResult<()> {
        let config = config
            .as_ref()
            .ok_or_else(|| ConfigError::MissingField("openai configuration section".to_string()))?;

        if config.api_key.is_empty() {
            return Err(ConfigError::MissingField(
                "openai.api_key is required".to_string(),
            ));
        }

        if config.model.is_empty() {
            return Err(ConfigError::ValidationError(
                "openai.model cannot be empty".to_string(),
            ));
        }

        if config.timeout_secs == 0 {
            return Err(ConfigError::ValidationError(
                "openai.timeout_secs must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn validate_anthropic_config(config: &Option<AnthropicConfig>) -> ConfigResult<()> {
        let config = config.as_ref().ok_or_else(|| {
            ConfigError::MissingField("anthropic configuration section".to_string())
        })?;

        if config.api_key.is_empty() {
            return Err(ConfigError::MissingField(
                "anthropic.api_key is required".to_string(),
            ));
        }

        if config.model.is_empty() {
            return Err(ConfigError::ValidationError(
                "anthropic.model cannot be empty".to_string(),
            ));
        }

        if config.timeout_secs == 0 {
            return Err(ConfigError::ValidationError(
                "anthropic.timeout_secs must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn validate_chat_config(config: &Option<ChatConfig>) -> ConfigResult<()> {
        if let Some(chat) = config {
            if chat.max_history_messages == 0 {
                return Err(ConfigError::ValidationError(
                    "chat.max_history_messages must be greater than 0".to_string(),
                ));
            }

            if chat.context_window_size == 0 {
                return Err(ConfigError::ValidationError(
                    "chat.context_window_size must be greater than 0".to_string(),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_disabled_extension() {
        let mut config = RootConfig::default();
        config.enabled = false;
        assert!(ConfigValidator::validate(&config).is_ok());
    }

    #[test]
    fn test_validate_missing_provider_field() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = String::new();
        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        match result {
            Err(ConfigError::MissingField(msg)) => {
                assert!(msg.contains("provider"));
            }
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn test_validate_invalid_provider() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = "invalid_provider".to_string();
        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        match result {
            Err(ConfigError::InvalidProvider(_)) => (),
            _ => panic!("Expected InvalidProvider error"),
        }
    }

    #[test]
    fn test_validate_openai_missing_config() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = "openai".to_string();
        config.openai = None;
        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_openai_missing_api_key() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = "openai".to_string();
        if let Some(openai) = config.openai.as_mut() {
            openai.api_key = String::new();
        }
        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        match result {
            Err(ConfigError::MissingField(msg)) => {
                assert!(msg.contains("api_key"));
            }
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn test_validate_openai_invalid_timeout() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = "openai".to_string();
        if let Some(openai) = config.openai.as_mut() {
            openai.timeout_secs = 0;
        }
        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_anthropic_missing_config() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = "anthropic".to_string();
        config.anthropic = None;
        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_anthropic_missing_api_key() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = "anthropic".to_string();
        if let Some(anthropic) = config.anthropic.as_mut() {
            anthropic.api_key = String::new();
        }
        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
        match result {
            Err(ConfigError::MissingField(msg)) => {
                assert!(msg.contains("api_key"));
            }
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn test_validate_chat_invalid_max_history() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = "openai".to_string();
        if let Some(chat) = config.chat.as_mut() {
            chat.max_history_messages = 0;
        }
        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_chat_invalid_context_window() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = "openai".to_string();
        if let Some(chat) = config.chat.as_mut() {
            chat.context_window_size = 0;
        }
        let result = ConfigValidator::validate(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_openai_valid_config() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = "openai".to_string();
        if let Some(openai) = config.openai.as_mut() {
            openai.api_key = "sk-test-key".to_string();
        }
        assert!(ConfigValidator::validate(&config).is_ok());
    }

    #[test]
    fn test_validate_anthropic_valid_config() {
        let mut config = RootConfig::default();
        config.enabled = true;
        config.provider = "anthropic".to_string();
        if let Some(anthropic) = config.anthropic.as_mut() {
            anthropic.api_key = "sk-ant-test-key".to_string();
        }
        assert!(ConfigValidator::validate(&config).is_ok());
    }
}
