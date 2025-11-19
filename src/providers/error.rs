use std::fmt;

#[derive(Debug, Clone)]
pub enum ProviderError {
    ApiError(String),
    ConfigError(String),
    NetworkError(String),
    ParseError(String),
    NotAvailable(String),
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProviderError::ApiError(msg) => write!(f, "API error: {}", msg),
            ProviderError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            ProviderError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ProviderError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProviderError::NotAvailable(msg) => write!(f, "Provider not available: {}", msg),
        }
    }
}

impl std::error::Error for ProviderError {}

pub type ProviderResult<T> = Result<T, ProviderError>;
