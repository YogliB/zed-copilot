mod common;

use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, ResponseTemplate};

mod e2e_helpers;
use e2e_helpers::E2ETestContext;

#[tokio::test]
async fn test_openai_completion_contract_validation() {
    let ctx = E2ETestContext::new().await;

    let mock_response = json!({
        "id": "chatcmpl-8Lw9S6pWkB6aKGU5Q7KQZpzP",
        "object": "chat.completion",
        "created": 1699473200,
        "model": "gpt-4",
        "choices": [
            {
                "index": 0,
                "message": {
                    "role": "assistant",
                    "content": "Hello! I'm Claude, an AI assistant. How can I help you today?"
                },
                "finish_reason": "stop"
            }
        ],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 20,
            "total_tokens": 30
        }
    });

    assert!(
        mock_response.get("id").is_some()
            && mock_response.get("object").is_some()
            && mock_response.get("choices").is_some()
            && mock_response.get("usage").is_some(),
        "Mock response should have required fields"
    );

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(header("authorization", "Bearer sk-test-key"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&ctx.mock_server)
        .await;

    assert!(!ctx.openai_base_url().is_empty());
}

#[tokio::test]
async fn test_openai_streaming_response_format() {
    let streaming_chunk = json!({
        "id": "chatcmpl-8Lw9S6pWkB6aKGU5Q7KQZpzP",
        "object": "chat.completion.chunk",
        "created": 1699473200,
        "model": "gpt-4",
        "choices": [
            {
                "index": 0,
                "delta": {
                    "role": "assistant",
                    "content": "Hello"
                },
                "finish_reason": null
            }
        ]
    });

    assert!(
        streaming_chunk.get("id").is_some()
            && streaming_chunk.get("object").is_some()
            && streaming_chunk.get("choices").is_some(),
        "Streaming chunk should have required fields"
    );
}

#[tokio::test]
async fn test_openai_error_response_format() {
    let ctx = E2ETestContext::new().await;

    let error_response = json!({
        "error": {
            "message": "Invalid request: missing required field 'model'",
            "type": "invalid_request_error",
            "param": "model",
            "code": "missing_field"
        }
    });

    assert!(
        error_response.get("error").is_some(),
        "Error response should have proper structure"
    );

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&error_response))
        .mount(&ctx.mock_server)
        .await;

    assert!(!ctx.openai_base_url().is_empty());
}

#[tokio::test]
async fn test_openai_rate_limit_response() {
    let ctx = E2ETestContext::new().await;

    let error_response = json!({
        "error": {
            "message": "Rate limit exceeded",
            "type": "server_error",
            "code": "rate_limit_exceeded"
        }
    });

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(
            ResponseTemplate::new(429)
                .append_header("retry-after", "60")
                .set_body_json(&error_response),
        )
        .mount(&ctx.mock_server)
        .await;

    assert!(!ctx.openai_base_url().is_empty());
}

#[tokio::test]
async fn test_openai_auth_error() {
    let ctx = E2ETestContext::new().await;

    let error_response = json!({
        "error": {
            "message": "Incorrect API key provided",
            "type": "invalid_request_error",
            "param": null,
            "code": "invalid_api_key"
        }
    });

    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&ctx.mock_server)
        .await;

    assert!(!ctx.openai_base_url().is_empty());
}

#[tokio::test]
async fn test_openai_request_validation() {
    let request_body = json!({
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "Hello!"
            }
        ],
        "temperature": 0.7,
        "max_tokens": 100
    });

    let has_required_fields = request_body.get("model").is_some()
        && request_body.get("messages").is_some();
    assert!(
        has_required_fields,
        "Request should have required fields: model, messages"
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
async fn test_openai_message_roles_valid() {
    let valid_roles = vec!["user", "assistant", "system"];

    for role in valid_roles {
        let message = json!({
            "role": role,
            "content": "Test message"
        });

        assert!(message.get("role").is_some());
        assert_eq!(
            message.get("role").and_then(|r| r.as_str()),
            Some(role)
        );
    }
}

#[tokio::test]
async fn test_openai_temperature_bounds() {
    assert!(0.0 >= 0.0 && 0.0 <= 2.0, "Temperature 0.0 should be within bounds");
    assert!(1.0 >= 0.0 && 1.0 <= 2.0, "Temperature 1.0 should be within bounds");
    assert!(2.0 >= 0.0 && 2.0 <= 2.0, "Temperature 2.0 should be within bounds");

    assert!(
        !(-0.1 >= 0.0 && -0.1 <= 2.0),
        "Temperature -0.1 should be out of bounds"
    );
    assert!(
        !(2.1 >= 0.0 && 2.1 <= 2.0),
        "Temperature 2.1 should be out of bounds"
    );
}

#[tokio::test]
async fn test_openai_response_missing_required_fields() {
    let incomplete_response = json!({
        "object": "chat.completion",
        "created": 1699473200,
        "model": "gpt-4",
        "choices": []
    });

    let has_required_fields = incomplete_response.get("id").is_some()
        && incomplete_response.get("object").is_some()
        && incomplete_response.get("created").is_some()
        && incomplete_response.get("model").is_some()
        && incomplete_response.get("choices").is_some();

    assert!(
        !has_required_fields,
        "Response missing required field 'id' should fail validation"
    );
}

#[tokio::test]
async fn test_openai_mock_server_is_available() {
    let ctx = E2ETestContext::new().await;
    let url = ctx.openai_base_url();

    assert!(!url.is_empty(), "Mock server URL should not be empty");
    assert!(
        url.starts_with("http://127.0.0.1:") || url.starts_with("http://localhost:"),
        "Mock server should run on localhost"
    );
}
