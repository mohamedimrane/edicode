use crate::terminal_utils::color_fg;
use termion::color::{Reset, Rgb};

#[derive(Clone)]
pub struct Message {
    pub kind: MessageType,
    pub message: String,
}

#[derive(Clone)]
pub enum MessageType {
    Normal,
    Error,
}

impl Message {
    pub fn new(kind: MessageType, message: String) -> Self {
        Self { kind, message }
    }

    pub fn new_normal(message: String) -> Self {
        Self {
            kind: MessageType::Normal,
            message,
        }
    }

    pub fn new_error(message: String) -> Self {
        Self {
            kind: MessageType::Error,
            message,
        }
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            kind: MessageType::Normal,
            message: String::new(),
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MessageType::*;

        match self.kind {
            Normal => write!(f, "{}", color_fg(&self.message, Reset)),
            Error => write!(f, "{}", color_fg(&self.message, Rgb(255, 0, 0))),
        }
    }
}
