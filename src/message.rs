pub struct Message {
    kind: MessageType,
    message: String,
}

pub enum MessageType {
    Normal,
    Error,
}

impl Message {
    pub fn new(kind: MessageType, message: String) -> Self {
        Self { kind, message }
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
        use termion::color::{Fg, Reset, Rgb};
        use MessageType::*;

        // let color: dyn color::Color = match self.kind {
        //     Normal => Reset,
        //     Error => Rgb(255, 0, 0),
        // };
        // write!(
        //     f,
        //     "{}{}{}",
        //     Fg(Red),
        //     self.message,
        //     Fg(Reset)
        // )

        match self.kind {
            Normal => write!(f, "{}{}", Fg(Reset), self.message),
            Error => write!(f, "{}{}{}", Fg(Rgb(255, 0, 0)), self.message, Fg(Reset)),
        }
    }
}
