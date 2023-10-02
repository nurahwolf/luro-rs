use luro_model::message::LuroMessage;
use twilight_model::{
    channel::Message,
    gateway::payload::incoming::{MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate},
};

use crate::DatabaseMessageType;

impl From<Message> for DatabaseMessageType {
    fn from(message: Message) -> Self {
        Self::Message(message)
    }
}

impl From<LuroMessage> for DatabaseMessageType {
    fn from(message: LuroMessage) -> Self {
        Self::LuroMessage(message)
    }
}

impl From<MessageUpdate> for DatabaseMessageType {
    fn from(message: MessageUpdate) -> Self {
        Self::MessageUpdate(message)
    }
}

impl From<MessageDelete> for DatabaseMessageType {
    fn from(message: MessageDelete) -> Self {
        Self::MessageDelete(message)
    }
}

impl From<MessageDeleteBulk> for DatabaseMessageType {
    fn from(message: MessageDeleteBulk) -> Self {
        Self::MessageDeleteBulk(message)
    }
}

impl From<MessageCreate> for DatabaseMessageType {
    fn from(message: MessageCreate) -> Self {
        Self::MessageCreate(message)
    }
}

#[cfg(feature = "cache")]
impl From<twilight_cache_inmemory::model::CachedMessage> for DatabaseMessageType {
    fn from(message: twilight_cache_inmemory::model::CachedMessage) -> Self {
        Self::CachedMessage(message)
    }
}
