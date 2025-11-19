use zed_extension_api as zed;

/// ZedCopilot extension structure
/// Implements the Extension trait to provide AI-powered code completion
/// and assistance features within Zed IDE
pub struct ZedCopilot;

impl zed::Extension for ZedCopilot {
    fn new() -> Self {
        println!("[Zed Copilot] Extension initialized");
        ZedCopilot
    }
}

zed::register_extension!(ZedCopilot);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zed_copilot_creation() {
        // Verify that ZedCopilot can be instantiated
        let _extension = ZedCopilot::new();
        // If this test passes, the struct is properly defined
    }

    #[test]
    fn test_extension_trait_implemented() {
        // This test verifies that ZedCopilot implements the Extension trait
        // by successfully creating an instance via the trait's new() method
        let _extension = <ZedCopilot as zed::Extension>::new();
        // Successful instantiation means the trait is properly implemented
    }

    // Future tests as features are added:
    // - test_ai_provider_initialization
    // - test_completion_engine
    // - test_context_extraction
    // - test_error_handling
}
