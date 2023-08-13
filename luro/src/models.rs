use std::collections::HashMap;

use luro_model::luro_message_source::LuroMessageSource;
use serde::{Deserialize, Serialize};

use twilight_model::{
    application::command::Command,
    channel::message::Embed,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id
    },
    user::User,
    util::ImageHash
};

use crate::LuroFramework;

mod luro_framework;
mod luro_message;
mod luro_webhook;

/// A simple structure containing our commands
/// TODO: Change this to a hashmap?
#[derive(Default)]
pub struct Commands {
    /// Commands that are available to be registered within guilds
    pub guild_commands: HashMap<&'static str, Command>,
    /// Commands that are available to be registered globally
    pub global_commands: HashMap<&'static str, Command>
}

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

/// Used for handling webhooks
pub struct LuroWebhook {
    framework: LuroFramework
}

/// Settings that are stored on disk and meant to be modified by the user
#[derive(Debug)]
pub struct Settings {
    /// The application ID
    pub application_id: Id<ApplicationMarker>
}

mod slash_user;

/// Some useful formatting around a user, such as their avatar .
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SlashUser {
    pub user_id: Id<UserMarker>,
    pub user_avatar: Option<ImageHash>,
    pub user_discriminator: u16,
    pub user_name: String,
    pub user_global_name: Option<String>,
    pub user_banner: Option<ImageHash>,
    pub member_avatar: Option<ImageHash>,
    pub member_nickname: Option<String>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub avatar: String,
    pub banner: Option<String>,
    pub name: String
}
