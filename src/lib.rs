use zed_extension_api as zed;

pub mod providers;

pub struct ZedCopilot;

impl ZedCopilot {
    pub fn new() -> Self {
        println!("[Zed Copilot] Extension initialized");
        ZedCopilot
    }
}

impl Default for ZedCopilot {
    fn default() -> Self {
        Self::new()
    }
}

impl zed::Extension for ZedCopilot {
    fn new() -> Self {
        <Self as std::default::Default>::default()
    }
}

zed::register_extension!(ZedCopilot);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zed_copilot_new() {
        let extension = ZedCopilot::new();
        let _ = extension;
    }

    #[test]
    fn test_zed_copilot_default() {
        let extension = ZedCopilot::default();
        let _ = extension;
    }

    #[test]
    fn test_extension_trait_new() {
        let _extension = <ZedCopilot as zed::Extension>::new();
    }

    #[test]
    fn test_multiple_instances() {
        let _ext1 = ZedCopilot::new();
        let _ext2 = ZedCopilot::new();
        let _ext3 = ZedCopilot::default();
    }

    #[test]
    fn test_extension_initialization_does_not_panic() {
        let result = std::panic::catch_unwind(|| {
            let _extension = ZedCopilot::new();
        });
        assert!(result.is_ok());
    }
}
