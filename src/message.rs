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
