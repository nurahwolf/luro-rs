use ::sqlx::types::Json;
use luro_model::{guild::LuroGuild, message::LuroMessage, user::LuroUser};
use time::OffsetDateTime;
use twilight_model::{
    application::interaction::InteractionData,
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
    pub name: String,
    pub guild_id: i64,
    pub owner_id: i64,
}

pub struct DatabaseInteraction {
    pub app_permissions: Option<i64>,
    pub application_id: i64,
    pub channel_id: Option<i64>,
    pub data: Option<Json<InteractionData>>,
    pub guild_id: Option<i64>,
    pub guild_locale: Option<String>,
    pub interaction_id: i64,
    pub kind: DatabaseInteractionKind,
    pub locale: Option<String>,
    pub member_id: Option<i64>,
    pub message_id: Option<i64>,
    pub token: String,
    pub user_id: Option<i64>,
}

pub struct DatabaseRole {
    pub colour: i32,
    pub deleted: bool,
    pub hoist: bool,
    pub icon: Option<Json<ImageHash>>,
    pub role_id: i64,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub permissions: i64,
    pub position: i64,
    pub flags: i64,
    pub tags: Option<Json<RoleTags>>,
    pub unicode_emoji: Option<String>,
}

#[cfg(feature = "sqlx-driver")]
#[derive(Debug, Default, ::sqlx::Type)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuroUserPermissions {
    #[default]
    User,
    Owner,
    Administrator,
}

impl From<LuroUserPermissions> for luro_model::user::LuroUserPermissions {
    fn from(permissions: LuroUserPermissions) -> Self {
        match permissions {
            LuroUserPermissions::User => Self::User,
            LuroUserPermissions::Owner =>  Self::Owner,
            LuroUserPermissions::Administrator =>  Self::Administrator,
        }
    }
}

impl From<luro_model::user::LuroUserPermissions> for  LuroUserPermissions {
    fn from(permissions: luro_model::user::LuroUserPermissions) -> Self {
        match permissions {
            luro_model::user::LuroUserPermissions::User => Self::User,
            luro_model::user::LuroUserPermissions::Owner => Self::Owner,
            luro_model::user::LuroUserPermissions::Administrator => Self::Administrator,
        }
    }
}

#[derive(Debug)]
pub struct DatabaseUser {
    // pub character_prefix: BTreeMap<String, String>,
    // pub guilds: HashMap<Id<GuildMarker>, LuroMember>,
    // pub marriages: BTreeMap<Id<UserMarker>, UserMarriages>,
    // pub moderation_actions_performed: usize,
    // pub moderation_actions: Json<Vec<UserActions>>,
    // pub words: Json<BTreeMap<String, usize>>,
    // pub wordsize: Json<BTreeMap<usize, usize>>,
    pub accent_colour: Option<i32>,
    pub avatar_decoration: Option<Json<ImageHash>>,
    pub avatar: Option<Json<ImageHash>>,
    pub banner: Option<Json<ImageHash>>,
    pub bot: Option<bool>,
    pub characters: Option<Vec<i32>>,
    pub discriminator: i16,
    pub email: Option<String>,
    pub flags: Option<Json<UserFlags>>,
    pub global_name: Option<String>,
    pub locale: Option<String>,
    pub message_edits: Option<i64>,
    pub messages: Option<Vec<i64>>,
    pub mfa_enabled: Option<bool>,
    pub name: String,
    pub premium_type: Option<Json<PremiumType>>,
    pub public_flags: Option<Json<UserFlags>>,
    pub system: Option<bool>,
    pub user_id: i64,
    pub user_permissions: LuroUserPermissions,
    pub verified: Option<bool>,
    pub warnings: Option<Vec<i64>>,
    pub words_average: Option<i64>,
    pub words_count: Option<i64>,
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
    pub member: Option<Json<PartialMember>>,
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
