use twilight_model::gateway::payload::incoming::{MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate};

#[derive(Debug)]
pub enum MessageSync<'a> {
    /// Created from an existing message
    Message(&'a twilight_model::channel::Message),
    /// Added / crafted manually
    Custom(&'a crate::Message),
    /// Created from a cached message
    #[cfg(feature = "twilight-cache")]
    CachedMessage(&'a CachedMessage),
    /// Created from a message update event
    MessageUpdate(&'a MessageUpdate),
    /// Created from a message delete event
    MessageDelete(&'a MessageDelete),
    /// Created from a message delete bulk event
    MessageDeleteBulk(&'a MessageDeleteBulk),
    /// Created from a message create event
    MessageCreate(&'a MessageCreate),
}
