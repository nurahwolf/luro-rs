use ::sqlx::types::Json;
use time::OffsetDateTime;
use twilight_model::{channel::{
    message::{
        sticker::MessageSticker, Component, Embed, MessageActivity, MessageApplication, MessageFlags, MessageInteraction,
        MessageReference, MessageType, Reaction, RoleSubscriptionData, Mention,
    },
    Attachment, ChannelMention, Message, Channel,
}, user::User, gateway::payload::incoming::MessageUpdate, guild::PartialMember};

#[cfg(feature = "sqlx-driver")]
mod sqlx;
#[cfg(feature = "toml-driver")]
mod toml;

/// Luro's database, using the toml driver
#[cfg(feature = "toml-driver")]
#[derive(Clone, Debug)]
pub struct LuroDatabase {}

/// Luro's database, using the sqlx driver
#[cfg(feature = "sqlx-driver")]
#[derive(Clone, Debug)]
pub struct LuroDatabase(::sqlx::Pool<::sqlx::Postgres>);

#[derive(Clone)]
pub struct DatabaseGuild {
    pub guild_id: i64,
    pub owner_id: i64,
}

pub struct DatabaseInteraction {
    pub application_id: i64,
    pub interaction_id: i64,
    pub message_id: Option<i64>,
    pub data: Vec<u8>,
    pub kind: Vec<u8>,
    pub token: String,
}

pub struct DatabaseRole {
    pub role_id: i64,
}

#[cfg(feature = "sqlx-driver")]
#[derive(Default, ::sqlx::Type)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuroUserPermissions {
    #[default]
    User,
    Owner,
    Administrator,
}
pub struct DatabaseUser {
    pub name: String,
    pub user_id: i64,
    pub user_permissions: LuroUserPermissions,
}

#[cfg(feature = "sqlx-driver")]
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

#[derive(Debug)]
pub struct DatabaseMessage {
    pub activity: Option<Json<MessageActivity>>,
    pub application_id: Option<i64>,
    pub application: Option<Json<MessageApplication>>,
    pub attachments: Option<Json<Vec<Attachment>>>,
    pub author: Json<User>,
    pub channel_id: i64,
    pub components: Option<Json<Vec<Component>>>,
    pub content: Option<String>,
    pub deleted: Option<bool>,
    pub edited_timestamp: Option<OffsetDateTime>,
    pub embeds: Option<Json<Vec<Embed>>>,
    pub flags: Option<Json<MessageFlags>>,
    pub guild_id: Option<i64>,
    pub id: i64,
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
    pub member: Option<Json<PartialMember>>
}