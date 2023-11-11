use twilight_model::gateway::payload::incoming::{MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate};

#[derive(Debug)]
pub enum MessageSync {
    /// Created from an existing message
    Message(twilight_model::channel::Message),
    /// Added / crafted manually
    Custom(crate::Message),
    /// Created from a cached message
    #[cfg(feature = "twilight-cache")]
    CachedMessage(CachedMessage),
    /// Created from a message update event
    MessageUpdate(MessageUpdate),
    /// Created from a message delete event
    MessageDelete(MessageDelete),
    /// Created from a message delete bulk event
    MessageDeleteBulk(MessageDeleteBulk),
    /// Created from a message create event
    MessageCreate(MessageCreate),
}
