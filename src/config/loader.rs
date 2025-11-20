use crate::config::errors::{ConfigError, ConfigResult};
use crate::config::structs::RootConfig;
use std::env;
use std::path::Path;

pub struct EnvInterpolator;

impl EnvInterpolator {
    pub fn interpolate(value: &str) -> ConfigResult<String> {
        let mut result = String::new();
        let mut chars = value.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' {
                if chars.peek() == Some(&'{') {
                    chars.next();
                    let var_name = Self::read_until_closing_brace(&mut chars)?;
                    let var_value = env::var(&var_name)
                        .map_err(|_| ConfigError::EnvVarNotFound(var_name.clone()))?;
                    result.push_str(&var_value);
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        }

        Ok(result)
    }

    fn read_until_closing_brace(
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> ConfigResult<String> {
        let mut var_name = String::new();
        let mut found_close = false;

        for ch in chars.by_ref() {
            if ch == '}' {
                found_close = true;
                break;
            }
            var_name.push(ch);
        }

        if !found_close {
            return Err(ConfigError::ValidationError(
                "Unclosed environment variable reference".to_string(),
            ));
        }

        if var_name.is_empty() {
            return Err(ConfigError::ValidationError(
                "Empty environment variable name".to_string(),
            ));
        }

        Ok(var_name)
    }
}

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load_from_json_string(json: &str) -> ConfigResult<RootConfig> {
        let config: RootConfig = serde_json::from_str(json)?;
        Ok(config)
    }

    pub fn load_from_file(path: &Path) -> ConfigResult<RootConfig> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::ParseError(format!("Failed to read config file: {}", e)))?;
        Self::load_from_json_string(&content)
    }

    pub fn load_from_zed_settings() -> ConfigResult<RootConfig> {
        Ok(RootConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_interpolator_single_variable() {
        env::set_var("TEST_VAR", "test_value");
        let result = EnvInterpolator::interpolate("Value: ${TEST_VAR}");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Value: test_value");
    }

    #[test]
    fn test_env_interpolator_multiple_variables() {
        env::set_var("VAR1", "value1");
        env::set_var("VAR2", "value2");
        let result = EnvInterpolator::interpolate("${VAR1} and ${VAR2}");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "value1 and value2");
    }

    #[test]
    fn test_env_interpolator_missing_variable() {
        let result = EnvInterpolator::interpolate("${NONEXISTENT_VAR_XYZ}");
        assert!(result.is_err());
        match result {
            Err(ConfigError::EnvVarNotFound(var)) => {
                assert_eq!(var, "NONEXISTENT_VAR_XYZ");
            }
            _ => panic!("Expected EnvVarNotFound error"),
        }
    }

    #[test]
    fn test_env_interpolator_no_variables() {
        let result = EnvInterpolator::interpolate("plain text");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "plain text");
    }

    #[test]
    fn test_env_interpolator_empty_variable_name() {
        let result = EnvInterpolator::interpolate("${}");
        assert!(result.is_err());
        match result {
            Err(ConfigError::ValidationError(msg)) => {
                assert!(msg.contains("Empty environment variable name"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_env_interpolator_unclosed_brace() {
        let result = EnvInterpolator::interpolate("${UNCLOSED");
        assert!(result.is_err());
        match result {
            Err(ConfigError::ValidationError(msg)) => {
                assert!(msg.contains("Unclosed"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_env_interpolator_dollar_sign_only() {
        let result = EnvInterpolator::interpolate("Price: $100");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Price: $100");
    }

    #[test]
    fn test_config_loader_valid_json() {
        let json = r#"{"enabled": true, "provider": "openai"}"#;
        let result = ConfigLoader::load_from_json_string(json);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.enabled);
        assert_eq!(config.provider, "openai");
    }

    #[test]
    fn test_config_loader_invalid_json() {
        let json = r#"{"invalid json"#;
        let result = ConfigLoader::load_from_json_string(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_loader_defaults() {
        let json = r#"{}"#;
        let result = ConfigLoader::load_from_json_string(json);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(!config.enabled);
        assert_eq!(config.provider, "");
    }
}
