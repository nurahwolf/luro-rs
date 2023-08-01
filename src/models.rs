use std::collections::{BTreeMap, HashMap};

use dashmap::DashMap;
use hyper::client::HttpConnector;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{reload::Handle, Registry};
use twilight_cache_inmemory::InMemoryCache;

use twilight_gateway::MessageSender;
use twilight_http::Client;
use twilight_lavalink::Lavalink;
use twilight_model::{
    application::{
        command::{Command, CommandOptionChoice},
        interaction::Interaction
    },
    channel::message::{AllowedMentions, Component, Embed, MessageFlags},
    guild::{Guild, Role},
    http::{attachment::Attachment, interaction::InteractionResponseType},
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id
    },
    oauth::Application,
    user::{CurrentUser, User}
};

use crate::LuroContext;

mod custom_id;
mod global_data;
mod guild_permissions;
mod guild_settings;
mod hecks;
mod luro_framework;
mod luro_permissions;
mod luro_slash;
mod luro_webhook;
mod member_roles;
mod role_ordering;
mod user_data;

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

/// Data that may be accessed globally, including DMs. Generally not modified by the end user
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalData {
    pub application: Application,
    pub count: usize,
    pub current_user: CurrentUser,
    pub hecks: Hecks,
    pub stories: Vec<Story>,
    pub owners: Vec<User>
}

/// Calculate the permissions for a given guild.
pub struct GuildPermissions<'a> {
    twilight_client: &'a Client,
    guild: Guild
}

/// Settings that are specific to a guild
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GuildSetting {
    /// The Guild's name
    pub guild_name: String,
    /// Commands registered to a guild
    pub commands: Vec<Command>,
    /// Private hecks for this specific guild
    pub hecks: Hecks,
    /// Guild Accent Colour, which is the first colour role within a guild
    pub accent_colour: u32,
    /// An administrator may wish to override the colour in which case this is set.
    pub accent_colour_custom: Option<u32>,
    /// Discord events are logged here, if defined
    pub discord_events_log_channel: Option<Id<ChannelMarker>>,
    /// Moderator actions are pushed here such as bans, if defined
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>
}

/// A specific heck, used in [Hecks]. This contains the message itself, and the user ID of the author.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Heck {
    pub heck_message: String,
    pub author_id: u64
}

/// Structure for `heck.toml`
/// We have two hecks, one that is slowly drained (so we only get a heck once) and another used to get explicit hecks.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Hecks {
    /// A vector containing all SFW hecks
    pub sfw_hecks: Vec<Heck>,
    /// A vector containing all NSFW hecks
    pub nsfw_hecks: Vec<Heck>,
    /// A vector of [usize] that contains availalbe random hecks to get. The hecks are reloaded when this reaches zero.
    pub sfw_heck_ids: Vec<usize>,
    /// A vector of [usize] that contains availalbe random hecks to get. The hecks are reloaded when this reaches zero.
    pub nsfw_heck_ids: Vec<usize>
}

/// The core of Luro. Used to handle our global state and generally wrapped in an [Arc].
#[derive(Debug)]
pub struct LuroFramework {
    /// HTTP client used for making outbound API requests
    pub hyper_client: hyper::Client<HttpConnector>,
    /// Lavalink client, for playing music
    pub lavalink: Lavalink,
    /// Twilight's client for interacting with the Discord API
    pub twilight_client: Client,
    /// Twilight's cache
    pub twilight_cache: InMemoryCache,
    /// The global tracing subscriber, for allowing manipulation within commands
    pub tracing_subscriber: Handle<LevelFilter, Registry>,
    /// Settings that are stored on disk and meant to be modified by the user
    pub settings: RwLock<Settings>,
    /// Data that may be accessed globally, including DMs
    pub global_data: RwLock<GlobalData>,
    /// Data that is specific to a guild
    pub guild_data: DashMap<Id<GuildMarker>, GuildSetting>,
    /// Data that is specific to a user
    pub user_data: DashMap<Id<UserMarker>, UserData>,
    /// Guild ID that can be set for some operations,
    pub guild_id: Option<Id<GuildMarker>>
}

/// Calculate the permissions of a member with information from the cache.
pub struct LuroPermissions<'a> {
    twilight_client: &'a Client,
    guild_id: Id<GuildMarker>,
    member_id: Id<UserMarker>,
    member_roles: MemberRoles,
    is_owner: bool
}

/// Some nice stuff about formatting a response, ready to send via twilight's client
#[derive(Clone, Debug)]
pub struct LuroSlash {
    // /// Luro's context, used for utility such as setting the embed accent colour and for sending our response near the end.
    pub luro: LuroContext,
    /// Interaction we are handling
    pub interaction: Interaction,
    pub shard: MessageSender,
    /// The interaction response type for our response. Defaults to [`InteractionResponseType::ChannelMessageWithSource`].
    pub interaction_response_type: InteractionResponseType,
    /// Allowed mentions of the response.
    pub allowed_mentions: Option<AllowedMentions>,
    /// List of attachments on the response.
    pub attachments: Option<Vec<Attachment>>,
    /// List of autocomplete alternatives.
    ///
    /// Can only be used with
    /// [`InteractionResponseType::ApplicationCommandAutocompleteResult`].
    pub choices: Option<Vec<CommandOptionChoice>>,
    /// List of components on the response.
    pub components: Option<Vec<Component>>,
    /// Content of the response.
    pub content: Option<String>,
    /// For [`InteractionResponseType::Modal`], user defined identifier.
    pub custom_id: Option<String>,
    /// Embeds of the response.
    pub embeds: Option<Vec<Embed>>,
    /// Interaction response data flags.
    ///
    /// The supported flags are [`MessageFlags::SUPPRESS_EMBEDS`] and
    /// [`MessageFlags::EPHEMERAL`].
    pub flags: Option<MessageFlags>,
    /// For [`InteractionResponseType::Modal`], title of the modal.
    pub title: Option<String>,
    /// Whether the response is TTS.
    pub tts: Option<bool>
}

/// Used for handling webhooks
pub struct LuroWebhook {
    luro: LuroContext
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
    pub position: i64
}

/// Settings that are stored on disk and meant to be modified by the user
#[derive(Debug)]
pub struct Settings {
    /// The application ID
    pub application_id: Id<ApplicationMarker>
}

/// TODO: This data structure is not needed, it's only so it can be serialised to disk
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Stories {
    pub stories: Vec<Story>
}

/// A story, which is simply a title and content both as strings
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Story {
    pub title: String,
    pub description: String
}

/// Data that is specific to a user
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserData {
    pub wordcount: usize,
    pub averagesize: usize,
    #[serde(
        deserialize_with = "user_data::deserialize_wordsize",
        serialize_with = "user_data::serialize_wordsize"
    )]
    pub wordsize: BTreeMap<usize, usize>,
    pub words: BTreeMap<String, usize>
}
