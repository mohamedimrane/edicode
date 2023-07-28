pub struct Message {
    kind: MessageType,
    message: String,
}

pub enum MessageType {
    Normal,
    Error,
}
