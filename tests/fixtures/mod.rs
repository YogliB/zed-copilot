use serde_json::{json, Value};

// OpenAI Response Templates

pub fn openai_completion_response() -> Value {
    json!({
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
    })
}

pub fn openai_streaming_chunk() -> Value {
    json!({
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
    })
}

pub fn openai_error_response(message: &str, code: &str) -> Value {
    json!({
        "error": {
            "message": message,
            "type": "invalid_request_error",
            "param": null,
            "code": code
        }
    })
}

pub fn openai_rate_limit_response() -> Value {
    json!({
        "error": {
            "message": "Rate limit exceeded",
            "type": "server_error",
            "code": "rate_limit_exceeded"
        }
    })
}

pub fn openai_auth_error() -> Value {
    json!({
        "error": {
            "message": "Incorrect API key provided",
            "type": "invalid_request_error",
            "param": null,
            "code": "invalid_api_key"
        }
    })
}

pub fn openai_missing_required_fields_response() -> Value {
    json!({
        "object": "chat.completion",
        "created": 1699473200,
        "model": "gpt-4",
        "choices": []
    })
}

// Anthropic Response Templates

pub fn anthropic_completion_response() -> Value {
    json!({
        "id": "msg_1234567890abcdef",
        "type": "message",
        "role": "assistant",
        "content": [
            {
                "type": "text",
                "text": "Hello! How can I assist you?"
            }
        ],
        "model": "claude-3-opus-20240229",
        "stop_reason": "end_turn",
        "stop_sequence": null,
        "usage": {
            "input_tokens": 10,
            "output_tokens": 20
        }
    })
}

pub fn anthropic_error_response(message: &str, error_type: &str) -> Value {
    json!({
        "type": "error",
        "error": {
            "type": error_type,
            "message": message
        }
    })
}

pub fn anthropic_rate_limit_response() -> Value {
    json!({
        "type": "error",
        "error": {
            "type": "rate_limit_error",
            "message": "Rate limit exceeded. Please retry after 60 seconds."
        }
    })
}

pub fn anthropic_auth_error() -> Value {
    json!({
        "type": "error",
        "error": {
            "type": "authentication_error",
            "message": "Invalid API key"
        }
    })
}

// Request Templates

pub fn valid_openai_request() -> Value {
    json!({
        "model": "gpt-4",
        "messages": [
            {
                "role": "user",
                "content": "Hello!"
            }
        ],
        "temperature": 0.7,
        "max_tokens": 100
    })
}

pub fn valid_anthropic_request() -> Value {
    json!({
        "model": "claude-3-opus-20240229",
        "messages": [
            {
                "role": "user",
                "content": "Hello!"
            }
        ],
        "max_tokens": 100,
        "temperature": 0.7
    })
}

// Assertion Helpers

pub fn assert_has_required_fields(value: &Value, fields: &[&str]) {
    for field in fields {
        assert!(
            value.get(field).is_some(),
            "Missing required field: '{}' in response: {}",
            field,
            value
        );
    }
}

pub fn assert_has_any_field(value: &Value, fields: &[&str]) {
    let has_any = fields.iter().any(|field| value.get(field).is_some());
    assert!(
        has_any,
        "Response must have at least one of: {:?}, got: {}",
        fields, value
    );
}

pub fn assert_is_error_response(value: &Value) {
    assert!(
        value.get("error").is_some(),
        "Response should be an error, got: {}",
        value
    );
}

pub fn assert_valid_message(message: &Value) {
    assert_has_required_fields(message, &["role", "content"]);

    let role = message
        .get("role")
        .and_then(|r| r.as_str())
        .unwrap_or("invalid");
    assert!(
        ["user", "assistant", "system"].contains(&role),
        "Invalid message role: {}",
        role
    );
}

pub fn assert_valid_messages_array(messages: &Value) {
    assert!(
        messages.is_array(),
        "Messages must be an array, got: {}",
        messages
    );

    if let Some(arr) = messages.as_array() {
        for message in arr {
            assert_valid_message(message);
        }
    }
}

pub fn assert_temperature_in_bounds(temp: f64, min: f64, max: f64) {
    assert!(
        temp >= min && temp <= max,
        "Temperature {} is out of bounds [{}, {}]",
        temp,
        min,
        max
    );
}

pub fn assert_temperature_out_of_bounds(temp: f64, min: f64, max: f64) {
    assert!(
        !(temp >= min && temp <= max),
        "Temperature {} should be out of bounds [{}, {}]",
        temp,
        min,
        max
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixtures_compile() {
        let _ = openai_completion_response();
        let _ = openai_streaming_chunk();
        let _ = openai_error_response("test", "test_code");
        let _ = anthropic_completion_response();
        let _ = anthropic_error_response("test", "test_type");
        let _ = valid_openai_request();
        let _ = valid_anthropic_request();
    }

    #[test]
    fn test_openai_response_has_required_fields() {
        let response = openai_completion_response();
        assert_has_required_fields(&response, &["id", "object", "choices", "usage"]);
    }

    #[test]
    fn test_assert_valid_message_accepts_valid() {
        let message = json!({
            "role": "user",
            "content": "test"
        });
        assert_valid_message(&message);
    }

    #[test]
    #[should_panic]
    fn test_assert_valid_message_rejects_invalid_role() {
        let message = json!({
            "role": "invalid",
            "content": "test"
        });
        assert_valid_message(&message);
    }

    #[test]
    fn test_temperature_bounds() {
        assert_temperature_in_bounds(0.0, 0.0, 2.0);
        assert_temperature_in_bounds(1.0, 0.0, 2.0);
        assert_temperature_in_bounds(2.0, 0.0, 2.0);
        assert_temperature_out_of_bounds(-0.1, 0.0, 2.0);
        assert_temperature_out_of_bounds(2.1, 0.0, 2.0);
    }
}
