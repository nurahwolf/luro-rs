use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::message::Embed,
    gateway::payload::incoming::{MessageCreate, MessageDelete, MessageUpdate},
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker},
        Id
    },
    user::User
};

use crate::luro_message_source::LuroMessageSource;

/// Effectively a wrapper around different type of messages, for more streamlined responses
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LuroMessage {
    pub author: Option<User>,
    pub content: Option<String>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub channel_id: Id<ChannelMarker>,
    pub id: Id<MessageMarker>,
    pub source: LuroMessageSource,
    pub embeds: Option<Vec<Embed>>
}

impl From<MessageUpdate> for LuroMessage {
    fn from(message: MessageUpdate) -> Self {
        Self {
            author: message.author,
            content: message.content,
            guild_id: message.guild_id,
            source: LuroMessageSource::MessageUpdate,
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
            source: LuroMessageSource::MessageDelete,
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
            source: LuroMessageSource::MessageCreate,
            channel_id: message.channel_id,
            id: message.id,
            embeds: Some(message.embeds.clone())
        }
    }
}
