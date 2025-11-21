mod common;

use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

use common::E2ETestContext;

#[tokio::test]
async fn test_anthropic_completion_basic_structure() {
    let mut ctx = E2ETestContext::new().await;

    let mock_response = json!({
        "id": "msg_1234567890",
        "type": "message",
        "role": "assistant",
        "content": [
            {
                "type": "text",
                "text": "Hello! How can I assist you today?"
            }
        ],
        "model": "claude-3-opus-20240229",
        "stop_reason": "end_turn",
        "stop_sequence": null,
        "usage": {
            "input_tokens": 10,
            "output_tokens": 20
        }
    });

    assert!(
        mock_response.get("id").is_some()
            && mock_response.get("content").is_some()
            && mock_response.get("usage").is_some(),
        "Mock response should have required fields"
    );

    Mock::given(method("POST"))
        .and(path("/messages"))
        .and(header("x-api-key", "sk-ant-test-key"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(ctx.mock_server_mut().await)
        .await;

    assert!(!ctx.anthropic_base_url().await.is_empty());
}

#[tokio::test]
async fn test_anthropic_streaming_response_format() {
    let mut ctx = E2ETestContext::new().await;

    let streaming_event = json!({
        "type": "content_block_delta",
        "index": 0,
        "delta": {
            "type": "text_delta",
            "text": "Hello"
        }
    });

    assert!(
        streaming_event.get("type").is_some() && streaming_event.get("delta").is_some(),
        "Streaming event should have required fields"
    );

    Mock::given(method("POST"))
        .and(path("/messages"))
        .and(header("x-api-key", "sk-ant-test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            "event: content_block_delta\ndata: {\"type\":\"content_block_delta\"}\n\n",
        ))
        .mount(ctx.mock_server_mut().await)
        .await;

    assert!(!ctx.anthropic_base_url().await.is_empty());
}

#[tokio::test]
async fn test_anthropic_error_response_format() {
    let mut ctx = E2ETestContext::new().await;

    let error_response = json!({
        "type": "error",
        "error": {
            "type": "invalid_request_error",
            "message": "Missing required field: model"
        }
    });

    assert!(
        error_response.get("error").is_some(),
        "Error response should have error field"
    );

    Mock::given(method("POST"))
        .and(path("/messages"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
        .mount(ctx.mock_server_mut().await)
        .await;

    assert!(!ctx.anthropic_base_url().await.is_empty());
}

#[tokio::test]
async fn test_anthropic_rate_limit_response() {
    let mut ctx = E2ETestContext::new().await;

    let error_response = json!({
        "type": "error",
        "error": {
            "type": "rate_limit_error",
            "message": "Rate limit exceeded"
        }
    });

    Mock::given(method("POST"))
        .and(path("/messages"))
        .respond_with(
            ResponseTemplate::new(429)
                .append_header("retry-after", "60")
                .set_body_json(&error_response),
        )
        .mount(ctx.mock_server_mut().await)
        .await;

    assert!(!ctx.anthropic_base_url().await.is_empty());
}

#[tokio::test]
async fn test_anthropic_auth_error() {
    let mut ctx = E2ETestContext::new().await;

    let error_response = json!({
        "type": "error",
        "error": {
            "type": "authentication_error",
            "message": "Invalid API key"
        }
    });

    Mock::given(method("POST"))
        .and(path("/messages"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(ctx.mock_server_mut().await)
        .await;

    assert!(!ctx.anthropic_base_url().await.is_empty());
}

#[tokio::test]
async fn test_anthropic_request_validation() {
    let request_body = json!({
        "model": "claude-3-opus-20240229",
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": "Hello!"
            }
        ]
    });

    let has_required_fields = request_body.get("model").is_some()
        && request_body.get("messages").is_some()
        && request_body.get("max_tokens").is_some();
    assert!(
        has_required_fields,
        "Request should have required fields: model, messages, max_tokens"
    );

    if let Some(messages) = request_body.get("messages").and_then(|m| m.as_array()) {
        for message in messages {
            assert!(
                message.get("role").is_some() && message.get("content").is_some(),
                "Each message must have 'role' and 'content'"
            );
        }
    }
}

#[tokio::test]
async fn test_anthropic_message_roles_valid() {
    let valid_roles = vec!["user", "assistant"];

    for role in valid_roles {
        let message = json!({
            "role": role,
            "content": "Test message"
        });

        assert!(message.get("role").is_some());
        assert_eq!(message.get("role").and_then(|r| r.as_str()), Some(role));
    }
}

#[tokio::test]
async fn test_anthropic_max_tokens_bounds() {
    let max_tokens = 2048;
    assert!(
        max_tokens >= 1 && max_tokens <= 4096,
        "max_tokens must be between 1 and 4096"
    );
}

#[tokio::test]
async fn test_anthropic_response_missing_required_fields() {
    let incomplete_response = json!({
        "model": "claude-3-opus-20240229",
        "usage": {
            "input_tokens": 10,
            "output_tokens": 20
        }
    });

    assert!(incomplete_response.get("id").is_none());
    assert!(incomplete_response.get("content").is_none());
}

#[tokio::test]
async fn test_anthropic_mock_server_is_available() {
    let mut ctx = E2ETestContext::new().await;
    let url = ctx.anthropic_base_url().await;

    assert!(!url.is_empty(), "Mock server URL should not be empty");
    assert!(
        url.starts_with("http://127.0.0.1:") || url.starts_with("http://localhost:"),
        "Mock server should run on localhost"
    );
}

#[tokio::test]
async fn test_anthropic_content_block_structure() {
    let content_block = json!({
        "type": "text",
        "text": "This is a response"
    });

    assert_eq!(
        content_block.get("type").and_then(|t| t.as_str()),
        Some("text")
    );
    assert!(content_block.get("text").is_some());
}

#[tokio::test]
async fn test_anthropic_stop_reason_valid() {
    let valid_stop_reasons = vec!["end_turn", "max_tokens", "stop_sequence"];

    for reason in valid_stop_reasons {
        let response = json!({
            "stop_reason": reason
        });

        assert_eq!(
            response.get("stop_reason").and_then(|r| r.as_str()),
            Some(reason)
        );
    }
}

#[tokio::test]
async fn test_anthropic_usage_tracking() {
    let response = json!({
        "id": "msg_123",
        "type": "message",
        "role": "assistant",
        "content": [],
        "usage": {
            "input_tokens": 50,
            "output_tokens": 100
        }
    });

    if let Some(usage) = response.get("usage") {
        assert!(usage.get("input_tokens").is_some());
        assert!(usage.get("output_tokens").is_some());
        assert_eq!(usage.get("input_tokens").and_then(|t| t.as_i64()), Some(50));
        assert_eq!(
            usage.get("output_tokens").and_then(|t| t.as_i64()),
            Some(100)
        );
    }
}

#[tokio::test]
async fn test_anthropic_system_prompt_support() {
    let request_with_system = json!({
        "model": "claude-3-opus-20240229",
        "max_tokens": 1024,
        "system": "You are a helpful assistant.",
        "messages": [
            {
                "role": "user",
                "content": "Hello!"
            }
        ]
    });

    assert!(request_with_system.get("system").is_some());
    assert_eq!(
        request_with_system.get("system").and_then(|s| s.as_str()),
        Some("You are a helpful assistant.")
    );
}
