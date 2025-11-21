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
}
