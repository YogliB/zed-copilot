# Zed Extension Panel API Reference

## Overview

This document provides comprehensive guidance on implementing UI panels in Zed extensions using the `zed_extension_api` crate (v0.1+).

## Core Concepts

### Extension Trait

All Zed extensions must implement the `zed::Extension` trait, which serves as the entry point for the extension:

```rust
pub trait Extension {
    fn new() -> Self;
}
```

The extension struct is registered via the `zed::register_extension!` macro.

### Workspace & Editor Context

Zed extensions interact with the editor through:

- **Workspace**: The top-level container for all editor windows, files, and UI elements
- **Editor**: The active text editor instance
- **View**: UI components that render within the editor or workspace

### Panel Architecture

Panels in Zed are implemented as views that dock within the workspace. Key characteristics:

1. **Lifecycle**: Panels are created, shown, focused, blurred, and destroyed
2. **Event Handling**: Panels receive keyboard and mouse events
3. **Rendering**: Panels use Zed's view system for rendering UI elements
4. **State Management**: Panel state is managed by the extension; Zed provides lifecycle hooks

## Panel Lifecycle

### States

```
[Created] → [Attached to Workspace] → [Visible] → [Focused/Blurred] → [Destroyed]
```

### Lifecycle Methods

- **`open()`**: Create and attach panel to workspace; make visible
- **`close()`**: Hide panel; optionally detach from workspace
- **`is_visible()`**: Query current visibility state
- **`focus()`**: Bring panel to foreground (optional)
- **`blur()`**: Remove focus from panel (optional)

## Implementation Pattern

### Basic Panel Structure

```rust
pub struct ChatPanel {
    visible: bool,
    focused: bool,
    // Panel-specific state
}

impl ChatPanel {
    pub fn new() -> Self {
        ChatPanel {
            visible: false,
            focused: false,
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
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }
}
```

## UI Components

### Message List View

A scrollable list displaying chat messages:

```rust
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
        // Auto-scroll to latest message
        self.scroll_position = self.messages.len().saturating_sub(1);
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.scroll_position = 0;
    }
}

pub struct Message {
    pub role: String,      // "user", "assistant", "system"
    pub content: String,
    pub timestamp: u64,
}
```

### Input Field

A text input for user messages:

```rust
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
}
```

### Submit Button

A clickable button to send messages:

```rust
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

    pub fn is_clickable(&self) -> bool {
        self.enabled
    }
}
```

## Event Handling

### Input Events

Panels receive keyboard and mouse events:

```rust
pub enum PanelEvent {
    KeyPressed { key: String, modifiers: u32 },
    MouseClicked { x: f32, y: f32, button: u32 },
    MouseMoved { x: f32, y: f32 },
    Focused,
    Blurred,
}
```

### Event Processing

```rust
impl ChatPanel {
    pub fn handle_event(&mut self, event: PanelEvent) {
        match event {
            PanelEvent::KeyPressed { key, modifiers } => {
                if key == "Enter" && modifiers & CTRL == CTRL {
                    // Handle submit
                }
            }
            PanelEvent::MouseClicked { x, y, .. } => {
                // Check if click is on submit button
            }
            PanelEvent::Focused => {
                self.set_focused(true);
            }
            PanelEvent::Blurred => {
                self.set_focused(false);
            }
            _ => {}
        }
    }
}
```

## Integration with Extension

### Registration

In the extension's main module (`src/lib.rs`):

```rust
pub struct ZedCopilot {
    chat_panel: Option<ChatPanel>,
}

impl ZedCopilot {
    pub fn new() -> Self {
        ZedCopilot {
            chat_panel: None,
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
            log::info!("Chat panel closed");
        }
    }
}

impl zed::Extension for ZedCopilot {
    fn new() -> Self {
        <Self as std::default::Default>::default()
    }
}
```

### Commands

Extensions expose commands that users can invoke:

```rust
// In extension.toml
[commands]
open-chat = "Open Copilot Chat"
close-chat = "Close Copilot Chat"

// Handler would be triggered by Zed when user executes command
// Implementation depends on zed_extension_api version
```

## Styling & Layout

### Zed UI Primitives

Zed provides a set of built-in UI primitives for consistent styling:

- **Container**: Basic layout box
- **Text**: Text rendering
- **Button**: Clickable button
- **TextInput**: Text input field
- **Scrollable**: Scrollable container
- **Flex**: Flexible layout (row/column)

### Basic Panel Layout

```
┌─────────────────────────────────────────┐
│ Chat Panel                           [×] │ ← Header
├─────────────────────────────────────────┤
│                                         │
│  [Assistant]                            │
│  > Hello! How can I help?               │
│                                         │
│  [You]                                  │
│  > What is Rust?                        │
│                                         │
│                                         │ ← Message List (scrollable)
│  [Assistant]                            │
│  > Rust is a systems language...        │
│                                         │
├─────────────────────────────────────────┤
│ [Type message...                      ] │ ← Input Field
│                           [Send Button] │
└─────────────────────────────────────────┘
```

### CSS-like Styling

Zed uses a flexible styling system:

```rust
pub struct PanelStyle {
    pub background_color: (u8, u8, u8, u8),  // RGBA
    pub text_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub padding: u32,
    pub margin: u32,
}

impl Default for PanelStyle {
    fn default() -> Self {
        PanelStyle {
            background_color: (240, 240, 240, 255),
            text_color: (0, 0, 0, 255),
            border_color: (200, 200, 200, 255),
            padding: 12,
            margin: 8,
        }
    }
}
```

## Error Handling

### Graceful Degradation

Panels should handle errors gracefully:

```rust
pub struct ContextExtractionError {
    pub message: String,
    pub recoverable: bool,
}

impl ChatPanel {
    pub fn load_context(&mut self) -> Result<(), ContextExtractionError> {
        // Attempt to extract context
        // On error, log and continue with empty/partial context
        Ok(())
    }
}
```

## Testing Strategy

### Unit Tests

Test panel lifecycle and state transitions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_opens_and_closes() {
        let mut panel = ChatPanel::new();
        assert!(!panel.is_visible());

        panel.open();
        assert!(panel.is_visible());

        panel.close();
        assert!(!panel.is_visible());
    }

    #[test]
    fn panel_focus_state() {
        let mut panel = ChatPanel::new();
        assert!(!panel.is_focused());

        panel.set_focused(true);
        assert!(panel.is_focused());

        panel.set_focused(false);
        assert!(!panel.is_focused());
    }
}
```

### Integration Tests

Test panel with extension context:

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn extension_opens_chat_panel() {
        let mut extension = ZedCopilot::new();
        extension.open_chat_panel();
        
        assert!(extension.chat_panel.is_some());
        assert!(extension.chat_panel.as_ref().unwrap().is_visible());
    }
}
```

## Best Practices

### 1. **Lifecycle Management**
   - Always initialize panels in a closed state
   - Properly clean up resources on close
   - Handle edge cases (multiple opens, close while closing)

### 2. **Error Handling**
   - No unwrap() calls; use Result<T, E>
   - Log errors at appropriate levels (debug, warn, error)
   - Provide fallback UI state on error

### 3. **Performance**
   - Limit message history in memory (cap at 1000 messages)
   - Debounce rapid open/close commands
   - Use lazy initialization for expensive resources

### 4. **Accessibility**
   - Support keyboard navigation (Tab, Shift+Tab, Enter)
   - Provide ARIA-like hints in logging
   - Support high-contrast themes

### 5. **Testing**
   - Aim for 90%+ test coverage
   - Test happy paths and edge cases
   - Test rapid state transitions (open → close → open)

## References

- Zed Extension API: `zed_extension_api` crate (v0.1)
- Zed Repository: https://github.com/zed-industries/zed
- Example Extensions: https://github.com/zed-industries/extensions

## Revision History

- **v1.0** (Initial): Research and documentation completed
- **Status**: Ready for ChatPanel implementation
