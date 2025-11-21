use zed_extension_api as zed;

#[cfg(test)]
pub mod config;
#[cfg(not(test))]
mod config;

#[cfg(test)]
pub mod http;
#[cfg(test)]
pub mod providers;

pub mod chat;
pub mod ui;

use chat::ZedContextProvider;
use ui::ChatPanel;

pub struct ZedCopilot {
    chat_panel: Option<ChatPanel>,
    context_provider: ZedContextProvider,
}

impl ZedCopilot {
    pub fn new() -> Self {
        log::info!("[Zed Copilot] Extension initialized");
        ZedCopilot {
            chat_panel: None,
            context_provider: ZedContextProvider::new(),
        }
    }

    pub fn open_chat_panel(&mut self) {
        let mut panel = ChatPanel::new();
        panel.open();
        self.chat_panel = Some(panel);
        log::info!("Chat panel opened via command");
    }

    pub fn close_chat_panel(&mut self) {
        if let Some(mut panel) = self.chat_panel.take() {
            panel.close();
            log::info!("Chat panel closed via command");
        }
    }

    pub fn is_chat_panel_open(&self) -> bool {
        self.chat_panel
            .as_ref()
            .map(|panel| panel.is_visible())
            .unwrap_or(false)
    }

    pub fn get_chat_panel(&self) -> Option<&ChatPanel> {
        self.chat_panel.as_ref()
    }

    pub fn get_chat_panel_mut(&mut self) -> Option<&mut ChatPanel> {
        self.chat_panel.as_mut()
    }

    pub fn get_context_provider(&self) -> &ZedContextProvider {
        &self.context_provider
    }

    pub fn get_context_provider_mut(&mut self) -> &mut ZedContextProvider {
        &mut self.context_provider
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
    use crate::chat::ContextProvider;

    #[test]
    fn test_zed_copilot_new() {
        let extension = ZedCopilot::new();
        assert!(!extension.is_chat_panel_open());
    }

    #[test]
    fn test_zed_copilot_default() {
        let extension = ZedCopilot::default();
        assert!(!extension.is_chat_panel_open());
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

    #[test]
    fn test_open_chat_panel() {
        let mut extension = ZedCopilot::new();
        assert!(!extension.is_chat_panel_open());

        extension.open_chat_panel();
        assert!(extension.is_chat_panel_open());
    }

    #[test]
    fn test_close_chat_panel() {
        let mut extension = ZedCopilot::new();
        extension.open_chat_panel();
        assert!(extension.is_chat_panel_open());

        extension.close_chat_panel();
        assert!(!extension.is_chat_panel_open());
    }

    #[test]
    fn test_chat_panel_lifecycle() {
        let mut extension = ZedCopilot::new();
        assert!(!extension.is_chat_panel_open());

        extension.open_chat_panel();
        assert!(extension.is_chat_panel_open());
        assert!(extension.get_chat_panel().is_some());

        extension.close_chat_panel();
        assert!(!extension.is_chat_panel_open());
        assert!(extension.get_chat_panel().is_none());
    }

    #[test]
    fn test_chat_panel_rapid_open_close() {
        let mut extension = ZedCopilot::new();

        for _ in 0..5 {
            extension.open_chat_panel();
            assert!(extension.is_chat_panel_open());
            extension.close_chat_panel();
            assert!(!extension.is_chat_panel_open());
        }
    }

    #[test]
    fn test_get_chat_panel_mut() {
        let mut extension = ZedCopilot::new();
        extension.open_chat_panel();

        if let Some(panel) = extension.get_chat_panel_mut() {
            assert!(panel.is_visible());
            panel.add_message(ui::Message::new("user", "Test"));
            assert_eq!(panel.message_count(), 1);
        } else {
            panic!("Chat panel should be open");
        }
    }

    #[test]
    fn test_context_provider_available() {
        let extension = ZedCopilot::new();
        let provider = extension.get_context_provider();
        let result = provider.get_file_context();
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_provider_mutable_access() {
        let mut extension = ZedCopilot::new();
        let provider = extension.get_context_provider_mut();
        provider.set_modified(true);
    }

    #[test]
    fn test_extension_initialization_with_panel_and_provider() {
        let mut extension = ZedCopilot::new();
        assert!(!extension.is_chat_panel_open());

        extension.open_chat_panel();
        if let Some(panel) = extension.get_chat_panel_mut() {
            panel.add_message(ui::Message::new("system", "Ready"));
        }

        let provider = extension.get_context_provider();
        let _ = provider.get_file_context();

        assert!(extension.is_chat_panel_open());
    }
}
