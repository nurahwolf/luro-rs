use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum LuroMessageSource {
    /// Created from an existing message
    Message,
    /// Added / crafted manually
    Custom,
    /// Created from a cached message
    CachedMessage,
    /// Created from a message update event
    MessageUpdate,
    /// Created from a message delete event
    MessageDelete,
    /// Created from a message create event
    MessageCreate,
    /// No message :(
    #[default]
    None
}
