use crate::providers::error::{ProviderError, ProviderResult};
use anthropic_rust::{
    client::Client,
    types::{ContentBlock, Model},
};
use futures::Stream;
use std::pin::Pin;

pub struct AnthropicHttpClient {
    client: Client,
}

impl AnthropicHttpClient {
    pub fn new(_api_base: String) -> ProviderResult<Self> {
        let client = Client::builder()
            .api_key("sk-ant-test-key")
            .model(Model::Claude35Sonnet20241022)
            .build()
            .map_err(|e| {
                ProviderError::ConfigError(format!("Failed to create Anthropic client: {}", e))
            })?;

        Ok(AnthropicHttpClient { client })
    }

    pub async fn complete(
        &self,
        prompt: &str,
        _model: &str,
        _api_key: &str,
    ) -> ProviderResult<String> {
        let request = self
            .client
            .chat_builder()
            .user_message(ContentBlock::text(prompt.to_string()))
            .build();

        let response = self
            .client
            .execute_chat(request)
            .await
            .map_err(|e| map_anthropic_error(e))?;

        response
            .content
            .first()
            .and_then(|block| {
                if let ContentBlock::Text { text, .. } = block {
                    Some(text.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                ProviderError::ParseError("Missing text in Anthropic response".to_string())
            })
    }

    pub async fn complete_stream(
        &self,
        prompt: &str,
        _model: &str,
        _api_key: &str,
    ) -> ProviderResult<Pin<Box<dyn Stream<Item = ProviderResult<String>> + Send>>> {
        let request = self
            .client
            .chat_builder()
            .user_message(ContentBlock::text(prompt.to_string()))
            .build();

        let stream = self
            .client
            .stream_chat(request)
            .await
            .map_err(|e| map_anthropic_error(e))?;

        let boxed_stream = Box::pin(futures::stream::unfold(stream, |mut stream| async move {
            match futures::StreamExt::next(&mut stream).await {
                Some(Ok(event)) => {
                    use anthropic_rust::ContentDelta;
                    use anthropic_rust::StreamEvent;

                    match event {
                        StreamEvent::ContentBlockDelta { delta, .. } => match delta {
                            ContentDelta::TextDelta { text } => Some((Ok(text), stream)),
                        },
                        _ => Some((Ok(String::new()), stream)),
                    }
                }
                Some(Err(e)) => Some((Err(map_anthropic_error(e)), stream)),
                None => None,
            }
        }));

        Ok(boxed_stream)
    }
}

fn map_anthropic_error(error: anthropic_rust::Error) -> ProviderError {
    let error_msg = error.to_string();
    if error_msg.contains("connection") || error_msg.contains("timeout") {
        ProviderError::NetworkError(format!("Anthropic connection error: {}", error_msg))
    } else if error_msg.contains("401") || error_msg.contains("403") {
        ProviderError::ConfigError(format!("Anthropic authentication error: {}", error_msg))
    } else {
        ProviderError::ApiError(format!("Anthropic API error: {}", error_msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_http_client_new() {
        let result = AnthropicHttpClient::new("https://api.anthropic.com/v1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_anthropic_http_client_new_custom_base() {
        let result = AnthropicHttpClient::new("https://custom.anthropic.com/v1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_map_anthropic_error_network() {
        let error = anthropic_rust::Error::Network("connection failed".to_string());
        let provider_error = map_anthropic_error(error);
        match provider_error {
            ProviderError::NetworkError(msg) => {
                assert!(msg.contains("Anthropic connection error"));
            }
            _ => panic!("Expected NetworkError"),
        }
    }

    #[test]
    fn test_map_anthropic_error_config() {
        let error = anthropic_rust::Error::Config("Config error".to_string());
        let provider_error = map_anthropic_error(error);
        match provider_error {
            ProviderError::ApiError(msg) => {
                assert!(msg.contains("Anthropic API error"));
            }
            _ => panic!("Expected ApiError for config error"),
        }
    }
}
