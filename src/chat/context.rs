use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileContext {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub language: String,
    pub modified: bool,
}

#[derive(Debug, Clone)]
pub struct SelectionContext {
    pub text: String,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

#[derive(Debug, Clone)]
pub struct CursorContext {
    pub line: u32,
    pub column: u32,
    pub character_under_cursor: Option<char>,
}

#[derive(Debug)]
pub enum ContextError {
    EditorNotAvailable,
    FileNotFound,
    IoError(String),
    EncodingError(String),
}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextError::EditorNotAvailable => write!(f, "Editor context not available"),
            ContextError::FileNotFound => write!(f, "File not found"),
            ContextError::IoError(msg) => write!(f, "IO error: {}", msg),
            ContextError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
        }
    }
}

impl std::error::Error for ContextError {}

pub type ContextResult<T> = Result<Option<T>, ContextError>;

pub trait ContextProvider: Send + Sync {
    fn get_file_context(&self) -> ContextResult<FileContext>;

    fn get_selection_context(&self) -> ContextResult<SelectionContext>;

    fn get_cursor_context(&self) -> ContextResult<CursorContext>;
}

pub struct ZedContextProvider {
    file_path: Option<PathBuf>,
    file_size: u64,
    language: String,
    modified: bool,
}

impl ZedContextProvider {
    pub fn new() -> Self {
        ZedContextProvider {
            file_path: None,
            file_size: 0,
            language: String::from("unknown"),
            modified: false,
        }
    }

    pub fn with_file_path(mut self, path: PathBuf) -> Self {
        self.file_path = Some(path);
        self
    }

    pub fn with_file_size(mut self, size: u64) -> Self {
        self.file_size = size;
        self
    }

    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    pub fn set_modified(&mut self, modified: bool) {
        self.modified = modified;
    }
}

impl Default for ZedContextProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextProvider for ZedContextProvider {
    fn get_file_context(&self) -> ContextResult<FileContext> {
        match &self.file_path {
            Some(path) => {
                if !path.exists() {
                    log::warn!("File context requested but file does not exist: {:?}", path);
                    return Ok(None);
                }

                Ok(Some(FileContext {
                    path: path.clone(),
                    size_bytes: self.file_size,
                    language: self.language.clone(),
                    modified: self.modified,
                }))
            }
            None => {
                log::debug!("No file context available: no file is currently open");
                Ok(None)
            }
        }
    }

    fn get_selection_context(&self) -> ContextResult<SelectionContext> {
        log::debug!("Selection context extraction not yet implemented");
        Ok(None)
    }

    fn get_cursor_context(&self) -> ContextResult<CursorContext> {
        log::debug!("Cursor context extraction not yet implemented");
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_context_creation() {
        let path = PathBuf::from("/tmp/test.rs");
        let context = FileContext {
            path: path.clone(),
            size_bytes: 1024,
            language: "rust".to_string(),
            modified: false,
        };

        assert_eq!(context.path, path);
        assert_eq!(context.size_bytes, 1024);
        assert_eq!(context.language, "rust");
        assert!(!context.modified);
    }

    #[test]
    fn test_selection_context_creation() {
        let context = SelectionContext {
            text: "let x = 5;".to_string(),
            start_line: 1,
            start_column: 0,
            end_line: 1,
            end_column: 10,
        };

        assert_eq!(context.text, "let x = 5;");
        assert_eq!(context.start_line, 1);
        assert_eq!(context.start_column, 0);
        assert_eq!(context.end_line, 1);
        assert_eq!(context.end_column, 10);
    }

    #[test]
    fn test_cursor_context_creation() {
        let context = CursorContext {
            line: 5,
            column: 10,
            character_under_cursor: Some('x'),
        };

        assert_eq!(context.line, 5);
        assert_eq!(context.column, 10);
        assert_eq!(context.character_under_cursor, Some('x'));
    }

    #[test]
    fn test_cursor_context_no_character() {
        let context = CursorContext {
            line: 0,
            column: 0,
            character_under_cursor: None,
        };

        assert_eq!(context.line, 0);
        assert_eq!(context.column, 0);
        assert_eq!(context.character_under_cursor, None);
    }

    #[test]
    fn test_zed_context_provider_new() {
        let provider = ZedContextProvider::new();
        assert!(provider.file_path.is_none());
        assert_eq!(provider.file_size, 0);
        assert_eq!(provider.language, "unknown");
        assert!(!provider.modified);
    }

    #[test]
    fn test_zed_context_provider_builder() {
        let path = PathBuf::from("/tmp/test.rs");
        let provider = ZedContextProvider::new()
            .with_file_path(path.clone())
            .with_file_size(2048)
            .with_language("rust");

        assert_eq!(provider.file_path, Some(path));
        assert_eq!(provider.file_size, 2048);
        assert_eq!(provider.language, "rust");
    }

    #[test]
    fn test_zed_context_provider_get_file_context_no_file() {
        let provider = ZedContextProvider::new();
        let result = provider.get_file_context();

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_zed_context_provider_get_file_context_with_nonexistent_file() {
        let path = PathBuf::from("/tmp/nonexistent_file_xyz_12345.rs");
        let provider = ZedContextProvider::new()
            .with_file_path(path)
            .with_file_size(1024)
            .with_language("rust");

        let result = provider.get_file_context();
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_zed_context_provider_get_selection_context() {
        let provider = ZedContextProvider::new();
        let result = provider.get_selection_context();

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_zed_context_provider_get_cursor_context() {
        let provider = ZedContextProvider::new();
        let result = provider.get_cursor_context();

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_context_provider_trait_implemented() {
        let provider = ZedContextProvider::new();
        let _: &dyn ContextProvider = &provider;
    }

    #[test]
    fn test_context_error_display() {
        let error = ContextError::EditorNotAvailable;
        assert_eq!(error.to_string(), "Editor context not available");

        let error = ContextError::FileNotFound;
        assert_eq!(error.to_string(), "File not found");

        let error = ContextError::IoError("test error".to_string());
        assert_eq!(error.to_string(), "IO error: test error");
    }

    #[test]
    fn test_file_context_modified_flag() {
        let context = FileContext {
            path: PathBuf::from("/tmp/test.rs"),
            size_bytes: 512,
            language: "rust".to_string(),
            modified: true,
        };

        assert!(context.modified);
    }

    #[test]
    fn test_selection_context_multiline() {
        let context = SelectionContext {
            text: "fn main() {\n    println!(\"hello\");\n}".to_string(),
            start_line: 1,
            start_column: 0,
            end_line: 3,
            end_column: 1,
        };

        assert_eq!(context.start_line, 1);
        assert_eq!(context.end_line, 3);
    }

    #[test]
    fn test_zed_context_provider_set_modified() {
        let mut provider = ZedContextProvider::new();
        assert!(!provider.modified);

        provider.set_modified(true);
        assert!(provider.modified);

        provider.set_modified(false);
        assert!(!provider.modified);
    }

    #[test]
    fn test_zed_context_provider_default() {
        let provider = ZedContextProvider::default();
        assert!(provider.file_path.is_none());
        assert_eq!(provider.file_size, 0);
    }
}
