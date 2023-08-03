use twilight_model::gateway::payload::incoming::{MessageCreate, MessageDelete, MessageUpdate};

use crate::models::LuroMessage;

impl From<MessageUpdate> for LuroMessage {
    fn from(message: MessageUpdate) -> Self {
        Self {
            author: message.author,
            content: message.content,
            guild_id: message.guild_id,
            source: super::LuroMessageSource::MessageUpdate,
            channel_id: message.channel_id,
            id: message.id,
            embeds: message.embeds
        }
    }
}

impl From<MessageDelete> for LuroMessage {
    fn from(message: MessageDelete) -> Self {
        Self {
            author: None,
            content: None,
            guild_id: message.guild_id,
            source: super::LuroMessageSource::MessageDelete,
            channel_id: message.channel_id,
            id: message.id,
            embeds: None
        }
    }
}

impl From<MessageCreate> for LuroMessage {
    fn from(message: MessageCreate) -> Self {
        Self {
            author: Some(message.author.clone()),
            content: Some(message.content.clone()),
            guild_id: message.guild_id,
            source: super::LuroMessageSource::MessageDelete,
            channel_id: message.channel_id,
            id: message.id,
            embeds: Some(message.embeds.clone())
        }
    }
}
