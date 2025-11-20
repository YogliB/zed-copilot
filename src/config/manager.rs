use crate::config::errors::ConfigResult;
use crate::config::loader::{ConfigLoader, EnvInterpolator};
use crate::config::structs::{AnthropicConfig, ChatConfig, OpenAiConfig, RootConfig};
use crate::config::validator::ConfigValidator;

#[derive(Debug, Clone)]
pub enum ProviderConfig {
    OpenAi(OpenAiConfig),
    Anthropic(AnthropicConfig),
}

impl ProviderConfig {
    pub fn api_key(&self) -> &str {
        match self {
            ProviderConfig::OpenAi(config) => &config.api_key,
            ProviderConfig::Anthropic(config) => &config.api_key,
        }
    }

    pub fn model(&self) -> &str {
        match self {
            ProviderConfig::OpenAi(config) => &config.model,
            ProviderConfig::Anthropic(config) => &config.model,
        }
    }

    pub fn api_base(&self) -> &str {
        match self {
            ProviderConfig::OpenAi(config) => &config.api_base,
            ProviderConfig::Anthropic(config) => &config.api_base,
        }
    }

    pub fn timeout_secs(&self) -> u64 {
        match self {
            ProviderConfig::OpenAi(config) => config.timeout_secs,
            ProviderConfig::Anthropic(config) => config.timeout_secs,
        }
    }

    pub fn provider_name(&self) -> &str {
        match self {
            ProviderConfig::OpenAi(_) => "openai",
            ProviderConfig::Anthropic(_) => "anthropic",
        }
    }
}

pub struct ConfigManager {
    config: RootConfig,
}

impl ConfigManager {
    pub fn initialize() -> ConfigResult<Self> {
        let config = ConfigLoader::load_from_zed_settings()?;
        ConfigValidator::validate(&config)?;

        Ok(ConfigManager { config })
    }

    pub fn initialize_from_json(json: &str) -> ConfigResult<Self> {
        let mut config = ConfigLoader::load_from_json_string(json)?;
        Self::interpolate_env_vars(&mut config)?;
        ConfigValidator::validate(&config)?;

        Ok(ConfigManager { config })
    }

    pub fn initialize_from_file(path: &std::path::Path) -> ConfigResult<Self> {
        let mut config = ConfigLoader::load_from_file(path)?;
        Self::interpolate_env_vars(&mut config)?;
        ConfigValidator::validate(&config)?;

        Ok(ConfigManager { config })
    }

    pub fn get_active_provider(&self) -> ConfigResult<ProviderConfig> {
        match self.config.provider.as_str() {
            "openai" => {
                let openai = self.config.openai.as_ref().ok_or_else(|| {
                    crate::config::errors::ConfigError::MissingField(
                        "openai configuration".to_string(),
                    )
                })?;
                Ok(ProviderConfig::OpenAi(openai.clone()))
            }
            "anthropic" => {
                let anthropic = self.config.anthropic.as_ref().ok_or_else(|| {
                    crate::config::errors::ConfigError::MissingField(
                        "anthropic configuration".to_string(),
                    )
                })?;
                Ok(ProviderConfig::Anthropic(anthropic.clone()))
            }
            _ => Err(crate::config::errors::ConfigError::InvalidProvider(
                self.config.provider.clone(),
            )),
        }
    }

    pub fn get_chat_config(&self) -> ChatConfig {
        self.config.chat.clone().unwrap_or_default()
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    pub fn provider_name(&self) -> &str {
        &self.config.provider
    }

    fn interpolate_env_vars(config: &mut RootConfig) -> ConfigResult<()> {
        if let Some(openai) = &mut config.openai {
            openai.api_key = EnvInterpolator::interpolate(&openai.api_key)?;
        }

        if let Some(anthropic) = &mut config.anthropic {
            anthropic.api_key = EnvInterpolator::interpolate(&anthropic.api_key)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_config_openai_helpers() {
        let openai = OpenAiConfig {
            api_key: "test_key".to_string(),
            model: "gpt-4".to_string(),
            api_base: "https://api.openai.com/v1".to_string(),
            timeout_secs: 30,
        };
        let config = ProviderConfig::OpenAi(openai);

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.model(), "gpt-4");
        assert_eq!(config.api_base(), "https://api.openai.com/v1");
        assert_eq!(config.timeout_secs(), 30);
        assert_eq!(config.provider_name(), "openai");
    }

    #[test]
    fn test_provider_config_anthropic_helpers() {
        let anthropic = AnthropicConfig {
            api_key: "test_key".to_string(),
            model: "claude-3-sonnet-20240229".to_string(),
            api_base: "https://api.anthropic.com/v1".to_string(),
            timeout_secs: 30,
        };
        let config = ProviderConfig::Anthropic(anthropic);

        assert_eq!(config.api_key(), "test_key");
        assert_eq!(config.model(), "claude-3-sonnet-20240229");
        assert_eq!(config.api_base(), "https://api.anthropic.com/v1");
        assert_eq!(config.timeout_secs(), 30);
        assert_eq!(config.provider_name(), "anthropic");
    }

    #[test]
    fn test_config_manager_initialize_from_valid_json() {
        let json = r#"
        {
            "enabled": true,
            "provider": "openai",
            "openai": {
                "api_key": "test_key",
                "model": "gpt-4",
                "api_base": "https://api.openai.com/v1",
                "timeout_secs": 30
            },
            "chat": {
                "streaming_enabled": true,
                "max_history_messages": 50,
                "auto_scroll_to_latest": true,
                "context_window_size": 4096
            }
        }
        "#;

        let manager = ConfigManager::initialize_from_json(json);
        assert!(manager.is_ok());
        let manager = manager.unwrap();
        assert!(manager.is_enabled());
        assert_eq!(manager.provider_name(), "openai");
    }

    #[test]
    fn test_config_manager_get_active_provider_openai() {
        let json = r#"
        {
            "enabled": true,
            "provider": "openai",
            "openai": {
                "api_key": "sk-test",
                "model": "gpt-4"
            }
        }
        "#;

        let manager = ConfigManager::initialize_from_json(json).unwrap();
        let provider = manager.get_active_provider();
        assert!(provider.is_ok());
        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "openai");
        assert_eq!(provider.api_key(), "sk-test");
    }

    #[test]
    fn test_config_manager_get_active_provider_anthropic() {
        let json = r#"
        {
            "enabled": true,
            "provider": "anthropic",
            "anthropic": {
                "api_key": "sk-ant-test",
                "model": "claude-3-sonnet-20240229"
            }
        }
        "#;

        let manager = ConfigManager::initialize_from_json(json).unwrap();
        let provider = manager.get_active_provider();
        assert!(provider.is_ok());
        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "anthropic");
        assert_eq!(provider.api_key(), "sk-ant-test");
    }

    #[test]
    fn test_config_manager_get_chat_config() {
        let json = r#"
        {
            "enabled": true,
            "provider": "openai",
            "openai": {
                "api_key": "test_key"
            },
            "chat": {
                "streaming_enabled": false,
                "max_history_messages": 100,
                "auto_scroll_to_latest": false,
                "context_window_size": 8192
            }
        }
        "#;

        let manager = ConfigManager::initialize_from_json(json).unwrap();
        let chat_config = manager.get_chat_config();
        assert!(!chat_config.streaming_enabled);
        assert_eq!(chat_config.max_history_messages, 100);
        assert!(!chat_config.auto_scroll_to_latest);
        assert_eq!(chat_config.context_window_size, 8192);
    }

    #[test]
    fn test_config_manager_chat_config_defaults() {
        let json = r#"
        {
            "enabled": true,
            "provider": "openai",
            "openai": {
                "api_key": "test_key"
            }
        }
        "#;

        let manager = ConfigManager::initialize_from_json(json).unwrap();
        let chat_config = manager.get_chat_config();
        assert!(chat_config.streaming_enabled);
        assert_eq!(chat_config.max_history_messages, 50);
        assert!(chat_config.auto_scroll_to_latest);
        assert_eq!(chat_config.context_window_size, 4096);
    }

    #[test]
    fn test_config_manager_env_interpolation() {
        std::env::set_var("TEST_API_KEY", "interpolated_key");
        let json = r#"
        {
            "enabled": true,
            "provider": "openai",
            "openai": {
                "api_key": "${TEST_API_KEY}"
            }
        }
        "#;

        let manager = ConfigManager::initialize_from_json(json).unwrap();
        let provider = manager.get_active_provider().unwrap();
        assert_eq!(provider.api_key(), "interpolated_key");
    }

    #[test]
    fn test_config_manager_disabled_extension() {
        let json = r#"
        {
            "enabled": false,
            "provider": "openai",
            "openai": {
                "api_key": ""
            }
        }
        "#;

        let manager = ConfigManager::initialize_from_json(json);
        assert!(manager.is_ok());
        assert!(!manager.unwrap().is_enabled());
    }

    #[test]
    fn test_config_manager_invalid_json() {
        let json = "invalid json";
        let result = ConfigManager::initialize_from_json(json);
        assert!(result.is_err());
    }
}
