use serde_json::json;

mod lazy_mock_server;
pub use lazy_mock_server::LazyMockServer;

mod e2e_helpers;
pub use e2e_helpers::E2ETestContext;

pub struct TestContext {
    pub extension_name: String,
}

impl TestContext {
    pub fn new() -> Self {
        TestContext {
            extension_name: "zed-copilot".to_string(),
        }
    }
}

impl Default for TestContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a single error scenario for parameterized E2E testing.
/// Used to validate that the client handles various API error responses correctly.
///
/// # Example
///
/// ```ignore
/// MockErrorScenario {
///     name: "invalid_api_key",
///     status: 401,
///     body_fn: || json!({"error": {"type": "invalid_request_error"}}),
///     expected_error_type: "invalid_request_error",
/// }
/// ```
///
/// # Fields
///
/// - `name`: Unique identifier for this scenario (used in test output for debugging)
/// - `status`: HTTP status code the mock server will return
/// - `body_fn`: Function that generates the error response JSON (use closure to avoid const issues)
/// - `expected_error_type`: The error.type field expected in the response
///
/// # Usage
///
/// Scenarios are grouped by API (e.g., `get_openai_error_scenarios()`) to keep related
/// error cases together. When adding new scenarios:
/// 1. Ensure the response matches the actual API spec
/// 2. Use unique, descriptive names
/// 3. Include all required fields for the error type
/// 4. Keep scenarios focused (auth errors separate from rate limits, etc.)
pub struct MockErrorScenario {
    pub name: &'static str,
    pub status: u16,
    pub body_fn: fn() -> serde_json::Value,
    pub expected_error_type: &'static str,
}

/// Returns all OpenAI error scenarios for parameterized E2E testing.
///
/// These scenarios represent realistic error responses from the OpenAI API,
/// grouped by HTTP status code and error type. Use in parameterized tests to validate
/// that the client handles each error case consistently.
///
/// # Scenarios
///
/// - **malformed_request** (400): Invalid request with missing required field
/// - **rate_limit_exceeded** (429): Rate limit exceeded with retry guidance
/// - **invalid_api_key** (401): Authentication failure due to invalid API key
///
/// # Example
///
/// ```ignore
/// #[tokio::test]
/// async fn test_openai_error_scenarios() {
///     let scenarios = get_openai_error_scenarios();
///
///     for scenario in scenarios {
///         // Setup mock server
///         let body = (scenario.body_fn)();
///         Mock::given(method("POST"))
///             .respond_with(ResponseTemplate::new(scenario.status).set_body_json(&body))
///             .mount(&ctx.mock_server)
///             .await;
///
///         // Assertions
///         assert_eq!(body.get("error").and_then(|e| e.get("type")).and_then(|t| t.as_str()),
///                    Some(scenario.expected_error_type));
///     }
/// }
/// ```
///
/// # Adding New Scenarios
///
/// 1. Verify the response matches OpenAI API documentation
/// 2. Use a unique, descriptive name
/// 3. Include all fields from the actual API response
/// 4. Document the scenario in the "Scenarios" section above
/// 5. Run tests to ensure they still pass
pub fn get_openai_error_scenarios() -> Vec<MockErrorScenario> {
    vec![
        MockErrorScenario {
            name: "malformed_request",
            status: 400,
            body_fn: || {
                json!({
                    "error": {
                        "message": "Invalid request: missing required field 'model'",
                        "type": "invalid_request_error",
                        "param": "model",
                        "code": "missing_field"
                    }
                })
            },
            expected_error_type: "invalid_request_error",
        },
        MockErrorScenario {
            name: "rate_limit_exceeded",
            status: 429,
            body_fn: || {
                json!({
                    "error": {
                        "message": "Rate limit exceeded",
                        "type": "server_error",
                        "code": "rate_limit_exceeded"
                    }
                })
            },
            expected_error_type: "server_error",
        },
        MockErrorScenario {
            name: "invalid_api_key",
            status: 401,
            body_fn: || {
                json!({
                    "error": {
                        "message": "Incorrect API key provided",
                        "type": "invalid_request_error",
                        "param": null,
                        "code": "invalid_api_key"
                    }
                })
            },
            expected_error_type: "invalid_request_error",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let context = TestContext::new();
        assert_eq!(context.extension_name, "zed-copilot");
    }

    #[test]
    fn test_context_default() {
        let context = TestContext::default();
        assert_eq!(context.extension_name, "zed-copilot");
    }

    #[test]
    fn test_context_creation_succeeds() {
        let context = TestContext::new();
        assert!(!context.extension_name.is_empty());
    }

    #[test]
    fn test_multiple_contexts_can_coexist() {
        let ctx1 = TestContext::new();
        let ctx2 = TestContext::new();
        let ctx3 = TestContext::default();
        assert_eq!(ctx1.extension_name, ctx2.extension_name);
        assert_eq!(ctx2.extension_name, ctx3.extension_name);
    }

    #[test]
    fn test_error_scenarios_are_valid() {
        let scenarios = get_openai_error_scenarios();

        assert!(!scenarios.is_empty(), "Error scenarios should not be empty");

        for scenario in scenarios {
            assert!(
                !scenario.name.is_empty(),
                "Scenario name should not be empty"
            );
            assert!(
                scenario.status > 0,
                "Scenario status should be valid HTTP status"
            );

            let body = (scenario.body_fn)();
            assert!(
                body.get("error").is_some(),
                "Scenario body should contain 'error' field for {}",
                scenario.name
            );
            assert!(
                !scenario.expected_error_type.is_empty(),
                "Expected error type should not be empty for {}",
                scenario.name
            );
        }
    }

    #[test]
    fn test_error_scenario_names_are_unique() {
        let scenarios = get_openai_error_scenarios();
        let mut names = Vec::new();

        for scenario in scenarios {
            assert!(
                !names.contains(&scenario.name),
                "Duplicate scenario name: {}",
                scenario.name
            );
            names.push(scenario.name);
        }
    }
}
