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
}
