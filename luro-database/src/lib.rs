
#[cfg(feature = "sql-driver")]
use ::sqlx::types::Json;
#[cfg(feature = "diesel-driver")]
use ::diesel::sql_types::Jsonb as Json;

use luro_model::{guild::LuroGuild, message::LuroMessage, user::LuroUser};
use time::OffsetDateTime;
use twilight_model::{
    channel::{
        message::{
            sticker::MessageSticker, Component, Embed, Mention, MessageActivity, MessageApplication, MessageFlags,
            MessageInteraction, MessageReference, MessageType, Reaction, RoleSubscriptionData,
        },
        Attachment, Channel, ChannelMention, Message,
    },
    gateway::payload::incoming::{GuildUpdate, MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate, UserUpdate},
    guild::{Guild, PartialMember, RoleTags},
    user::{User, UserFlags, PremiumType},
    util::ImageHash,
};

mod data; // Added functionality around the types defined in this crate
#[cfg(feature = "sqlx-driver")]
mod sqlx;
#[cfg(feature = "toml-driver")]
mod toml;
#[cfg(feature = "diesel-driver")]
mod diesel;
#[cfg(feature = "diesel-driver")]
mod schema;

#[cfg(feature = "toml-driver")]
pub use crate::diesel::{LuroDatabase, DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser};
#[cfg(feature = "sqlx-driver")]
pub use crate::diesel::{LuroDatabase, DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser};
#[cfg(feature = "diesel-driver")]
pub use crate::diesel::{LuroDatabase, DatabaseGuild, DatabaseInteraction, DatabaseInteractionKind, DatabaseUser};

#[cfg(feature = "sqlx-driver")]
#[derive(Debug, Default, ::sqlx::Type)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuroUserPermissions {
    #[default]
    User,
    Owner,
    Administrator,
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

#[cfg(feature = "diesel-driver")]
#[derive(Debug, Default)]
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

pub enum DatabaseGuildType {
    Guild(Guild),
    GuildUpdate(Box<GuildUpdate>),
    LuroGuild(LuroGuild),
}

pub enum DatabaseUserType {
    User(User),
    UserUpdate(UserUpdate),
    LuroUser(LuroUser),
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

#[cfg(feature = "sqlx-driver")]
#[derive(Debug, ::sqlx::Type)]
#[sqlx(type_name = "interaction_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DatabaseInteractionKind {
    ApplicationCommand,
    ApplicationCommandAutocomplete,
    MessageComponent,
    ModalSubmit,
    Ping,
    Unknown,
}
