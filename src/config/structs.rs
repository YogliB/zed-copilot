use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub provider: String,
    #[serde(default)]
    pub openai: Option<OpenAiConfig>,
    #[serde(default)]
    pub anthropic: Option<AnthropicConfig>,
    #[serde(default)]
    pub chat: Option<ChatConfig>,
}

impl Default for RootConfig {
    fn default() -> Self {
        RootConfig {
            enabled: false,
            provider: String::from("openai"),
            openai: Some(OpenAiConfig::default()),
            anthropic: Some(AnthropicConfig::default()),
            chat: Some(ChatConfig::default()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAiConfig {
    pub api_key: String,
    #[serde(default = "default_openai_model")]
    pub model: String,
    #[serde(default = "default_openai_api_base")]
    pub api_base: String,
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
}

impl Default for OpenAiConfig {
    fn default() -> Self {
        OpenAiConfig {
            api_key: String::new(),
            model: default_openai_model(),
            api_base: default_openai_api_base(),
            timeout_secs: default_timeout_secs(),
        }
    }
}

fn default_openai_model() -> String {
    String::from("gpt-4")
}

fn default_openai_api_base() -> String {
    String::from("https://api.openai.com/v1")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub api_key: String,
    #[serde(default = "default_anthropic_model")]
    pub model: String,
    #[serde(default = "default_anthropic_api_base")]
    pub api_base: String,
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        AnthropicConfig {
            api_key: String::new(),
            model: default_anthropic_model(),
            api_base: default_anthropic_api_base(),
            timeout_secs: default_timeout_secs(),
        }
    }
}

fn default_anthropic_model() -> String {
    String::from("claude-3-sonnet-20240229")
}

fn default_anthropic_api_base() -> String {
    String::from("https://api.anthropic.com/v1")
}

fn default_timeout_secs() -> u64 {
    30
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatConfig {
    #[serde(default = "default_streaming_enabled")]
    pub streaming_enabled: bool,
    #[serde(default = "default_max_history_messages")]
    pub max_history_messages: usize,
    #[serde(default = "default_auto_scroll_to_latest")]
    pub auto_scroll_to_latest: bool,
    #[serde(default = "default_context_window_size")]
    pub context_window_size: usize,
}

impl Default for ChatConfig {
    fn default() -> Self {
        ChatConfig {
            streaming_enabled: default_streaming_enabled(),
            max_history_messages: default_max_history_messages(),
            auto_scroll_to_latest: default_auto_scroll_to_latest(),
            context_window_size: default_context_window_size(),
        }
    }
}

fn default_streaming_enabled() -> bool {
    true
}

fn default_max_history_messages() -> usize {
    50
}

fn default_auto_scroll_to_latest() -> bool {
    true
}

fn default_context_window_size() -> usize {
    4096
}
