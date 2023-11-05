use luro_model::message::{LuroMessage, LuroMessageSource};
use sqlx::types::Json;

use time::OffsetDateTime;

use twilight_model::{
    channel::{
        message::{
            sticker::MessageSticker, Component, Embed, Mention, MessageActivity, MessageApplication, MessageFlags, MessageInteraction,
            MessageReference, MessageType, Reaction, RoleSubscriptionData,
        },
        Attachment, Channel, ChannelMention, Message,
    },
    gateway::payload::incoming::{MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate},
    guild::PartialMember,
    id::Id,
    user::User,
    util::Timestamp,
};

mod count_messages;
mod count_messages_by_user;
mod count_total_words;
mod count_user_words;
mod get_message;
mod get_messages;
#[cfg(feature = "cache")]
mod handle_cached_message;
mod handle_luro_message;
mod handle_message;
mod handle_message_create;
mod handle_message_delete;
mod handle_message_delete_bulk;
mod handle_message_update;
mod update_message;
mod fetch_user_messages;

#[derive(Default, Debug, ::sqlx::Type)]
#[sqlx(type_name = "message_source", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DatabaseMessageSource {
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
    None,
}

pub enum DatabaseMessageType {
    /// Created from an existing message
    Message(Message),
    /// Added / crafted manually
    LuroMessage(LuroMessage),
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

#[derive(Debug)]
pub struct DatabaseMessage {
    pub activity: Option<Json<MessageActivity>>,
    pub application_id: Option<i64>,
    pub application: Option<Json<MessageApplication>>,
    pub attachments: Option<Json<Vec<Attachment>>>,
    pub author: Json<User>,
    pub author_id: i64,
    pub channel_id: i64,
    pub components: Option<Json<Vec<Component>>>,
    pub content: Option<String>,
    pub deleted: Option<bool>,
    pub edited_timestamp: Option<OffsetDateTime>,
    pub embeds: Option<Json<Vec<Embed>>>,
    pub flags: Option<Json<MessageFlags>>,
    pub guild_id: Option<i64>,
    pub message_id: i64,
    pub interaction: Option<Json<MessageInteraction>>,
    pub kind: Json<MessageType>,
    pub mention_channels: Option<Json<Vec<ChannelMention>>>,
    pub mention_everyone: Option<bool>,
    pub mention_roles: Option<Vec<i64>>,
    pub mentions: Option<Json<Vec<Mention>>>,
    pub pinned: Option<bool>,
    pub reactions: Option<Json<Vec<Reaction>>>,
    pub reference: Option<Json<MessageReference>>,
    pub referenced_message: Option<Json<Box<Message>>>,
    pub role_subscription_data: Option<Json<RoleSubscriptionData>>,
    pub source: DatabaseMessageSource,
    pub sticker_items: Option<Json<Vec<MessageSticker>>>,
    pub thread: Option<Json<Channel>>,
    pub timestamp: time::OffsetDateTime,
    pub tts: Option<bool>,
    pub webhook_id: Option<i64>,
    pub message_updates: Option<Json<Vec<MessageUpdate>>>,
    pub member: Option<Json<PartialMember>>,
}

pub struct DbWordcount {
    pub message_id: i64,
    pub total_words: Option<i64>,
    pub total_unique_words: Option<i64>,
    pub message_content: Option<String>,
}

impl From<LuroMessage> for DatabaseMessage {
    fn from(message: LuroMessage) -> Self {
        Self {
            author_id: message.author.id.get() as i64,
            member: message.member.map(Json),
            activity: message.activity.map(|x| x.into()),
            application_id: message.application_id.map(|x| x.get() as i64),
            application: message.application.map(|x| x.into()),
            attachments: Some(message.attachments.into()),
            author: Json(message.author),
            channel_id: message.channel_id.get() as i64,
            components: Some(message.components.into()),
            content: message.content.into(),
            deleted: message.deleted.into(),
            edited_timestamp: message
                .edited_timestamp
                .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
            embeds: Some(message.embeds.into()),
            flags: message.flags.map(|x| x.into()),
            guild_id: message.guild_id.map(|x| x.get() as i64),
            message_id: message.id.get() as i64,
            interaction: message.interaction.map(|x| x.into()),
            kind: Json(message.kind),
            mention_channels: Some(message.mention_channels.into()),
            mention_everyone: message.mention_everyone.into(),
            mention_roles: message.mention_roles.into_iter().map(|x| x.get() as i64).collect::<Vec<_>>().into(),
            mentions: match !message.mentions.is_empty() {
                true => None,
                false => Some(Json(message.mentions)),
            },
            pinned: message.pinned.into(),
            reactions: Some(Json(message.reactions)),
            reference: message.reference.map(|x| x.into()),
            referenced_message: message.referenced_message.map(|x| x.into()),
            role_subscription_data: message.role_subscription_data.map(|x| x.into()),
            source: match message.source {
                LuroMessageSource::Message => DatabaseMessageSource::Message,
                LuroMessageSource::Custom => DatabaseMessageSource::Custom,
                LuroMessageSource::CachedMessage => DatabaseMessageSource::CachedMessage,
                LuroMessageSource::MessageUpdate => DatabaseMessageSource::MessageUpdate,
                LuroMessageSource::MessageDelete => DatabaseMessageSource::MessageDelete,
                LuroMessageSource::MessageCreate => DatabaseMessageSource::MessageCreate,
                LuroMessageSource::None => DatabaseMessageSource::None,
            },
            sticker_items: Some(Json(message.sticker_items)),
            thread: message.thread.map(Json),
            timestamp: OffsetDateTime::from_unix_timestamp(message.timestamp.as_secs()).unwrap(),
            tts: Some(message.tts),
            message_updates: None,
            webhook_id: message.webhook_id.map(|x| x.get() as i64),
        }
    }
}

impl From<DatabaseMessage> for LuroMessage {
    fn from(message: DatabaseMessage) -> Self {
        Self {
            member: message.member.map(|x| x.0),
            activity: message.activity.map(|x| x.0),
            application_id: message.application_id.map(|x| Id::new(x as u64)),
            application: message.application.map(|x| x.0),
            attachments: message.attachments.map(|x| x.0).unwrap_or_default(),
            author: message.author.0,
            channel_id: Id::new(message.channel_id as u64),
            components: message.components.map(|x| x.0).unwrap_or_default(),
            content: message.content.unwrap_or_default(),
            deleted: message.deleted.unwrap_or_default(),
            edited_timestamp: message.edited_timestamp.map(|x| Timestamp::from_secs(x.unix_timestamp()).unwrap()),
            embeds: message.embeds.map(|x| x.0).unwrap_or_default(),
            flags: message.flags.map(|x| x.0),
            guild_id: message.guild_id.map(|x| Id::new(x as u64)),
            id: Id::new(message.message_id as u64),
            interaction: message.interaction.map(|x| x.0),
            kind: message.kind.0,
            mention_channels: message.mention_channels.map(|x| x.0).unwrap_or_default(),
            mention_everyone: message.mention_everyone.unwrap_or_default(),
            mention_roles: message
                .mention_roles
                .map(|x| x.into_iter().map(|x| Id::new(x as u64)).collect())
                .unwrap_or_default(),
            mentions: message.mentions.map(|x| x.0).unwrap_or_default(),
            pinned: message.pinned.unwrap_or_default(),
            reactions: message.reactions.map(|x| x.0).unwrap_or_default(),
            reference: message.reference.map(|x| x.0),
            referenced_message: message.referenced_message.map(|x| x.0),
            role_subscription_data: message.role_subscription_data.map(|x| x.0),
            source: match message.source {
                DatabaseMessageSource::Message => LuroMessageSource::Message,
                DatabaseMessageSource::Custom => LuroMessageSource::Custom,
                DatabaseMessageSource::CachedMessage => LuroMessageSource::CachedMessage,
                DatabaseMessageSource::MessageUpdate => LuroMessageSource::MessageUpdate,
                DatabaseMessageSource::MessageDelete => LuroMessageSource::MessageDelete,
                DatabaseMessageSource::MessageCreate => LuroMessageSource::MessageCreate,
                DatabaseMessageSource::None => LuroMessageSource::None,
            },
            sticker_items: message.sticker_items.map(|x| x.0).unwrap_or_default(),
            thread: message.thread.map(|x| x.0),
            timestamp: Timestamp::from_secs(message.timestamp.unix_timestamp()).unwrap(),
            tts: message.tts.unwrap_or_default(),
            updated_content: None,
            webhook_id: message.webhook_id.map(|x| Id::new(x as u64)),
        }
    }
}
