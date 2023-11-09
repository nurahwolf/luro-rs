use luro_model::message::Message;

use twilight_model::gateway::payload::incoming::{MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate};

mod count_messages;
mod count_messages_by_user;
mod count_total_words;
mod count_user_words;
mod fetch_user_messages;
mod get_message;
mod get_messages;
#[cfg(feature = "cache")]
mod handle_cached_message;
mod update_message;

pub enum DatabaseMessageType {
    /// Created from an existing message
    Message(Message),
    /// Added / crafted manually
    LuroMessage(Message),
    /// Created from a cached message
    #[cfg(feature = "cache")]
    CachedMessage(twilight_cache_inmemory::model::CachedMessage),
    /// Created from a message update event
    MessageUpdate(MessageUpdate),
    /// Created from a message delete event
    MessageDelete(MessageDelete),
    /// Created from a message delete bulk event
    MessageDeleteBulk(MessageDeleteBulk),
    /// Created from a message create event
    MessageCreate(MessageCreate),
}

// REMOVE: This should be safe to remove
pub struct DbWordcount {
    pub message_id: i64,
    pub total_words: Option<i64>,
    pub total_unique_words: Option<i64>,
    pub message_content: Option<String>,
}
