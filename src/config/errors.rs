use std::fmt;

#[derive(Debug, Clone)]
pub enum ConfigError {
    ValidationError(String),
    MissingField(String),
    EnvVarNotFound(String),
    InvalidProvider(String),
    ParseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ValidationError(msg) => write!(f, "Configuration validation failed: {}", msg),
            ConfigError::MissingField(msg) => write!(f, "Missing required field: {}", msg),
            ConfigError::EnvVarNotFound(msg) => {
                write!(f, "Environment variable not found: {}. Set it before running Zed.", msg)
            }
            ConfigError::InvalidProvider(msg) => {
                write!(f, "Invalid or unsupported provider: {}. Supported providers: openai, anthropic", msg)
            }
            ConfigError::ParseError(msg) => write!(f, "Failed to parse configuration: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::ParseError(err.to_string())
    }
}

pub type ConfigResult<T> = Result<T, ConfigError>;
