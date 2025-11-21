use crate::providers::error::{ProviderError, ProviderResult};
use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client,
};
use futures::Stream;
use std::pin::Pin;

pub struct OpenAiHttpClient {
    client: Client<OpenAIConfig>,
}

impl OpenAiHttpClient {
    pub fn new(api_base: String) -> ProviderResult<Self> {
        let config = OpenAIConfig::new().with_api_base(api_base);

        Ok(OpenAiHttpClient {
            client: Client::with_config(config),
        })
    }

    pub async fn complete(
        &self,
        prompt: &str,
        model: &str,
        _api_key: &str,
    ) -> ProviderResult<String> {
        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()
            .map_err(|e| ProviderError::ConfigError(format!("Failed to build message: {}", e)))?;

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages([user_message.into()])
            .temperature(0.7)
            .max_tokens(1024u32)
            .build()
            .map_err(|e| ProviderError::ConfigError(format!("Failed to build request: {}", e)))?;

        let response = self
            .client
            .chat()
            .create(request)
            .await
            .map_err(|e| map_openai_error(e))?;

        response
            .choices
            .first()
            .and_then(|choice| choice.message.content.as_ref())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                ProviderError::ParseError("Missing content in OpenAI response".to_string())
            })
    }

    pub async fn complete_stream(
        &self,
        prompt: &str,
        model: &str,
        _api_key: &str,
    ) -> ProviderResult<Pin<Box<dyn Stream<Item = ProviderResult<String>> + Send>>> {
        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()
            .map_err(|e| ProviderError::ConfigError(format!("Failed to build message: {}", e)))?;

        let request = CreateChatCompletionRequestArgs::default()
            .model(model)
            .messages([user_message.into()])
            .temperature(0.7)
            .max_tokens(1024u32)
            .build()
            .map_err(|e| ProviderError::ConfigError(format!("Failed to build request: {}", e)))?;

        let stream = self
            .client
            .chat()
            .create_stream(request)
            .await
            .map_err(|e| map_openai_error(e))?;

        let boxed_stream = Box::pin(futures::stream::unfold(stream, |mut stream| async move {
            match futures::StreamExt::next(&mut stream).await {
                Some(Ok(response)) => {
                    let content = response
                        .choices
                        .first()
                        .and_then(|choice| choice.delta.content.as_ref())
                        .map(|s| s.to_string());

                    match content {
                        Some(text) => Some((Ok(text), stream)),
                        None => Some((Ok(String::new()), stream)),
                    }
                }
                Some(Err(e)) => Some((Err(map_openai_error(e)), stream)),
                None => None,
            }
        }));

        Ok(boxed_stream)
    }
}

fn map_openai_error(error: async_openai::error::OpenAIError) -> ProviderError {
    use async_openai::error::OpenAIError;

    match error {
        OpenAIError::ApiError(api_err) => {
            ProviderError::ApiError(format!("OpenAI API error: {}", api_err))
        }
        OpenAIError::Reqwest(req_err) => {
            if req_err.is_timeout() {
                ProviderError::NetworkError("OpenAI request timeout".to_string())
            } else if req_err.is_connect() {
                ProviderError::NetworkError("OpenAI connection error".to_string())
            } else {
                ProviderError::NetworkError(format!("OpenAI request error: {}", req_err))
            }
        }
        OpenAIError::InvalidArgument(msg) => {
            ProviderError::ConfigError(format!("OpenAI invalid argument: {}", msg))
        }
        OpenAIError::StreamError(msg) => {
            ProviderError::ApiError(format!("OpenAI streaming error: {}", msg))
        }
        _ => ProviderError::ApiError(format!("OpenAI error: {}", error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_http_client_new() {
        let result = OpenAiHttpClient::new("https://api.openai.com/v1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_openai_http_client_new_custom_base() {
        let result = OpenAiHttpClient::new("https://custom.openai.com/v1".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_map_openai_error_api_error() {
        let error = async_openai::error::OpenAIError::InvalidArgument("test error".to_string());

        let provider_error = map_openai_error(error);
        match provider_error {
            ProviderError::ConfigError(msg) => {
                assert!(msg.contains("OpenAI invalid argument"));
            }
            _ => panic!("Expected ConfigError"),
        }
    }
}
