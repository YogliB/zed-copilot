pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: u64,
}

impl Message {
    pub fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        Message {
            role: role.into(),
            content: content.into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }
}

pub struct InputField {
    content: String,
    cursor_position: usize,
    max_length: usize,
}

impl InputField {
    pub fn new() -> Self {
        InputField {
            content: String::new(),
            cursor_position: 0,
            max_length: 4096,
        }
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.content.len() < self.max_length {
            self.content.insert(self.cursor_position, ch);
            self.cursor_position += ch.len_utf8();
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.content.remove(self.cursor_position);
        }
    }

    pub fn submit(&mut self) -> String {
        let content = self.content.trim().to_string();
        self.content.clear();
        self.cursor_position = 0;
        content
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn clear(&mut self) {
        self.content.clear();
        self.cursor_position = 0;
    }
}

impl Default for InputField {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MessageListView {
    messages: Vec<Message>,
    scroll_position: usize,
}

impl MessageListView {
    pub fn new() -> Self {
        MessageListView {
            messages: Vec::new(),
            scroll_position: 0,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.scroll_position = self.messages.len().saturating_sub(1);
    }

    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.scroll_position = 0;
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn scroll_to_latest(&mut self) {
        self.scroll_position = self.messages.len().saturating_sub(1);
    }
}

impl Default for MessageListView {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SubmitButton {
    enabled: bool,
    hovered: bool,
}

impl SubmitButton {
    pub fn new() -> Self {
        SubmitButton {
            enabled: true,
            hovered: false,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    pub fn is_clickable(&self) -> bool {
        self.enabled
    }
}

impl Default for SubmitButton {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ChatPanel {
    visible: bool,
    focused: bool,
    message_list: MessageListView,
    input_field: InputField,
    submit_button: SubmitButton,
}

impl ChatPanel {
    pub fn new() -> Self {
        ChatPanel {
            visible: false,
            focused: false,
            message_list: MessageListView::new(),
            input_field: InputField::new(),
            submit_button: SubmitButton::new(),
        }
    }

    pub fn open(&mut self) {
        self.visible = true;
        log::debug!("Chat panel opened");
    }

    pub fn close(&mut self) {
        self.visible = false;
        self.focused = false;
        log::debug!("Chat panel closed");
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
        if focused {
            log::debug!("Chat panel focused");
        } else {
            log::debug!("Chat panel blurred");
        }
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn add_message(&mut self, message: Message) {
        self.message_list.add_message(message);
    }

    pub fn get_messages(&self) -> &[Message] {
        self.message_list.get_messages()
    }

    pub fn clear_messages(&mut self) {
        self.message_list.clear();
    }

    pub fn message_count(&self) -> usize {
        self.message_list.message_count()
    }

    pub fn get_input_field(&self) -> &InputField {
        &self.input_field
    }

    pub fn get_input_field_mut(&mut self) -> &mut InputField {
        &mut self.input_field
    }

    pub fn get_submit_button(&self) -> &SubmitButton {
        &self.submit_button
    }

    pub fn get_submit_button_mut(&mut self) -> &mut SubmitButton {
        &mut self.submit_button
    }
}

impl Default for ChatPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_panel_starts_hidden() {
        let panel = ChatPanel::new();
        assert!(!panel.is_visible());
    }

    #[test]
    fn test_chat_panel_open() {
        let mut panel = ChatPanel::new();
        assert!(!panel.is_visible());

        panel.open();
        assert!(panel.is_visible());
    }

    #[test]
    fn test_chat_panel_close() {
        let mut panel = ChatPanel::new();
        panel.open();
        assert!(panel.is_visible());

        panel.close();
        assert!(!panel.is_visible());
    }

    #[test]
    fn test_chat_panel_focus_state() {
        let mut panel = ChatPanel::new();
        assert!(!panel.is_focused());

        panel.set_focused(true);
        assert!(panel.is_focused());

        panel.set_focused(false);
        assert!(!panel.is_focused());
    }

    #[test]
    fn test_chat_panel_lifecycle() {
        let mut panel = ChatPanel::new();
        assert!(!panel.is_visible());
        assert!(!panel.is_focused());

        panel.open();
        assert!(panel.is_visible());
        assert!(!panel.is_focused());

        panel.set_focused(true);
        assert!(panel.is_visible());
        assert!(panel.is_focused());

        panel.set_focused(false);
        assert!(panel.is_visible());
        assert!(!panel.is_focused());

        panel.close();
        assert!(!panel.is_visible());
        assert!(!panel.is_focused());
    }

    #[test]
    fn test_message_list_add_message() {
        let mut panel = ChatPanel::new();
        assert_eq!(panel.message_count(), 0);

        let msg = Message::new("user", "Hello");
        panel.add_message(msg);
        assert_eq!(panel.message_count(), 1);
    }

    #[test]
    fn test_message_list_clear() {
        let mut panel = ChatPanel::new();
        panel.add_message(Message::new("user", "Hello"));
        panel.add_message(Message::new("assistant", "Hi"));
        assert_eq!(panel.message_count(), 2);

        panel.clear_messages();
        assert_eq!(panel.message_count(), 0);
    }

    #[test]
    fn test_input_field_insert_char() {
        let mut input = InputField::new();
        assert!(input.is_empty());

        input.insert_char('H');
        input.insert_char('i');
        assert_eq!(input.get_content(), "Hi");
    }

    #[test]
    fn test_input_field_delete_char() {
        let mut input = InputField::new();
        input.insert_char('H');
        input.insert_char('i');
        assert_eq!(input.get_content(), "Hi");

        input.delete_char();
        assert_eq!(input.get_content(), "H");
    }

    #[test]
    fn test_input_field_submit() {
        let mut input = InputField::new();
        input.insert_char('H');
        input.insert_char('i');
        assert_eq!(input.get_content(), "Hi");

        let submitted = input.submit();
        assert_eq!(submitted, "Hi");
        assert!(input.is_empty());
    }

    #[test]
    fn test_input_field_submit_with_whitespace() {
        let mut input = InputField::new();
        input.content = "  Hello  ".to_string();

        let submitted = input.submit();
        assert_eq!(submitted, "Hello");
        assert!(input.is_empty());
    }

    #[test]
    fn test_input_field_max_length() {
        let mut input = InputField::new();
        input.max_length = 5;

        for ch in "HelloWorld".chars() {
            input.insert_char(ch);
        }

        assert_eq!(input.get_content(), "Hello");
    }

    #[test]
    fn test_submit_button_enabled_state() {
        let mut button = SubmitButton::new();
        assert!(button.is_enabled());

        button.set_enabled(false);
        assert!(!button.is_enabled());

        button.set_enabled(true);
        assert!(button.is_enabled());
    }

    #[test]
    fn test_submit_button_hovered_state() {
        let mut button = SubmitButton::new();
        assert!(!button.is_hovered());

        button.set_hovered(true);
        assert!(button.is_hovered());

        button.set_hovered(false);
        assert!(!button.is_hovered());
    }

    #[test]
    fn test_submit_button_clickable() {
        let mut button = SubmitButton::new();
        assert!(button.is_clickable());

        button.set_enabled(false);
        assert!(!button.is_clickable());
    }

    #[test]
    fn test_message_creation() {
        let msg = Message::new("user", "Test message");
        assert_eq!(msg.role, "user");
        assert_eq!(msg.content, "Test message");
        assert!(msg.timestamp > 0);
    }

    #[test]
    fn test_message_list_view_scroll() {
        let mut view = MessageListView::new();
        view.add_message(Message::new("user", "1"));
        view.add_message(Message::new("assistant", "2"));
        view.add_message(Message::new("user", "3"));

        view.scroll_to_latest();
        assert_eq!(view.scroll_position, 2);
    }

    #[test]
    fn test_panel_rapid_open_close() {
        let mut panel = ChatPanel::new();

        for _ in 0..10 {
            panel.open();
            assert!(panel.is_visible());
            panel.close();
            assert!(!panel.is_visible());
        }
    }
}
