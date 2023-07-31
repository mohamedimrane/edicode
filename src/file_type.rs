use crate::highlighting::HighlightingOptions;

#[derive(Clone)]
pub enum FileType {
    PlainText,
    Rust,
    Golang,
    Javascript,
    Typescript,
}

impl From<&str> for FileType {
    fn from(value: &str) -> Self {
        FileType::from(value.to_string())
    }
}

impl From<String> for FileType {
    fn from(value: String) -> Self {
        let extension = value.split('.').last().unwrap_or_default();
        if extension.is_empty() {
            return Self::PlainText;
        }

        match extension {
            "rs" => Self::Rust,
            "go" => Self::Golang,
            "js" => Self::Javascript,
            "ts" => Self::Typescript,
            _ => Self::PlainText,
        }
    }
}

impl Into<HighlightingOptions> for FileType {
    fn into(self) -> HighlightingOptions {
        use FileType::*;

        match self {
            PlainText => HighlightingOptions {
                highlight_numbers: false,
                highlight_strings: None,
            },
            Rust | Golang | Javascript | Typescript => HighlightingOptions {
                highlight_numbers: true,
                highlight_strings: Some(vec!['\'', '"']),
            },
        }
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::PlainText
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FileType::*;

        let file_type_name = match self {
            PlainText => "Plain Text",
            Rust => "Rust",
            Golang => "Go",
            Javascript => "Javascript",
            Typescript => "Typescript",
        };

        write!(f, "{}", file_type_name)
    }
}
