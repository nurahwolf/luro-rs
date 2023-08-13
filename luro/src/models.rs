use std::collections::HashMap;

use luro_model::luro_message_source::LuroMessageSource;
use serde::{Deserialize, Serialize};

use twilight_http::Client;
use twilight_model::{
    application::command::Command,
    channel::message::Embed,
    guild::{Guild, Role},
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, MessageMarker, RoleMarker, UserMarker},
        Id
    },
    user::User,
    util::ImageHash
};

use crate::LuroFramework;

mod custom_id;
mod guild_permissions;
mod luro_framework;
mod luro_message;
mod luro_permissions;
mod luro_webhook;
mod member_roles;
mod role_ordering;

/// A simple structure containing our commands
/// TODO: Change this to a hashmap?
#[derive(Default)]
pub struct Commands {
    /// Commands that are available to be registered within guilds
    pub guild_commands: HashMap<&'static str, Command>,
    /// Commands that are available to be registered globally
    pub global_commands: HashMap<&'static str, Command>
}

/// Component custom id.
///
/// This type is used to hold component identifiers, used in buttons or modals.
/// Each custom id must have a `name` which correspond to the component type,
/// and optionally an `id` used to store component state.
pub struct CustomId {
    /// Name of the component.
    pub name: String,
    /// ID of the component.
    pub id: Option<String>
}

/// Calculate the permissions for a given guild.
pub struct GuildPermissions<'a> {
    twilight_client: &'a Client,
    guild: Guild
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LuroCommandCache {
    pub author: Id<UserMarker>,
    pub user_in_command: Id<UserMarker>,
    pub reason: String
}

/// Calculate the permissions of a member with information from the cache.
pub struct LuroPermissions<'a> {
    twilight_client: &'a Client,
    guild_id: Id<GuildMarker>,
    member_id: Id<UserMarker>,
    member_roles: MemberRoles,
    is_owner: bool
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

/// List of resolved roles of a member.
struct MemberRoles {
    /// Everyone role
    pub everyone: Role,
    /// List of roles of the user
    pub roles: Vec<Role>
}

/// Compares the position of two roles.
///
/// This type is used to compare positions of two different roles, using the
/// [`Ord`] trait.
///
/// According to [twilight-model documentation]:
///
/// > Roles are primarily ordered by their position in descending order.
/// > For example, a role with a position of 17 is considered a higher role than
/// > one with a position of 12.
/// >
/// > Discord does not guarantee that role positions are positive, unique, or
/// > contiguous. When two or more roles have the same position then the order
/// > is based on the roles’ IDs in ascending order. For example, given two roles
/// > with positions of 10 then a role with an ID of 1 would be considered a
/// > higher role than one with an ID of 20.
///
/// [twilight-model documentation]: https://docs.rs/twilight-model/0.10.2/twilight_model/guild/struct.Role.html#impl-Ord
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleOrdering {
    pub id: Id<RoleMarker>,
    pub position: i64,
    pub colour: u32
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
