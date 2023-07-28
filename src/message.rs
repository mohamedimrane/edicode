pub struct Message<'a> {
    kind: MessageType,
    message: &'a str,
}

pub enum MessageType {
    Normal,
    Error,
}

impl<'a> Message<'a> {
    pub fn new(kind: MessageType, message: &'a str) -> Self {
        Self { kind, message }
    }
}

impl<'a> std::fmt::Display for Message<'_> {
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
