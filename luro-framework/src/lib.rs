#![feature(async_fn_in_trait)]
use luro_model::{
    builders::EmbedBuilder,
    database_driver::{LuroDatabase, LuroDatabaseDriver},
    driver_toml::TomlDatabaseDriver,
    response::LuroResponse,
    user::LuroUser,
};
use slash_command::LuroCommand;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::info;
use twilight_http::client::InteractionClient;
use twilight_interactions::command::ResolvedUser;
use twilight_model::{
    application::{
        command::Command,
        interaction::{Interaction, InteractionData, InteractionType},
    },
    id::marker::UserMarker,
};

use twilight_model::{
    application::interaction::{
        application_command::CommandData, message_component::MessageComponentInteractionData, modal::ModalInteractionData,
    },
    channel::{Channel, Message},
    guild::{PartialMember, Permissions},
    id::{
        marker::{ApplicationMarker, GuildMarker, InteractionMarker},
        Id,
    },
    user::User,
};

pub mod command;
pub mod context;
mod framework;
pub mod interactions;
#[cfg(feature = "responses")]
pub mod responses;
pub mod slash_command;

#[cfg(feature = "database-toml")]
pub type DatabaseEngine = TomlDatabaseDriver;
type LuroCommandType = HashMap<String, LuroCommand<()>>;
type LuroMutex<T> = Arc<Mutex<T>>;

/// A context spawned from a command interaction
#[derive(Clone)]
pub struct CommandInteraction<T> {
    pub command: T,
    pub app_permissions: Option<twilight_model::guild::Permissions>,
    pub application_id: Id<twilight_model::id::marker::ApplicationMarker>,
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    pub channel: twilight_model::channel::Channel,
    pub data: Box<CommandData>,
    pub database: Arc<LuroDatabase<DatabaseEngine>>,
    pub global_commands: LuroMutex<LuroCommandType>,
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    pub id: Id<twilight_model::id::marker::InteractionMarker>,
    pub kind: twilight_model::application::interaction::InteractionType,
    pub latency: twilight_gateway::Latency,
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    pub locale: Option<String>,
    pub member: Option<twilight_model::guild::PartialMember>,
    pub message: Option<twilight_model::channel::Message>,
    pub original: twilight_model::application::interaction::Interaction,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub tracing_subscriber:
        tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    pub twilight_client: Arc<twilight_http::Client>,
    pub user: Option<twilight_model::user::User>,
}

/// Luro's primary context, which is instanced per event.
///
/// Contains [Framework] and houses data containing the [Event], [Latency] and [MessageSender].
#[derive(Clone)]
pub struct Context {
    /// The caching layer of the framework
    #[cfg(feature = "cache-memory")]
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    /// Luro's database, which accepts a driver that implements [LuroDatabaseDriver]
    pub database: Arc<LuroDatabase<DatabaseEngine>>,
    /// The raw event in which this [Context] was created from
    pub event: twilight_gateway::Event,
    /// A [Mutex] holding a bunch of [LuroCommandType] for global commands
    pub global_commands: LuroMutex<LuroCommandType>,
    /// A [Mutex] holding a bunch of [LuroCommandType] for guild commands, indexed by [GuildMarker]
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
    /// A HTTP client used for making web requests. Uses Hyper.
    #[cfg(feature = "http-client-hyper")]
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    /// Latency information from the shard that this event was spawned from
    pub latency: twilight_gateway::Latency,
    /// A lavalink instance for playing music
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    /// A [MessageSender] for interacting with the shard
    pub shard: twilight_gateway::MessageSender,
    /// Tracing subscriber information
    pub tracing_subscriber:
        tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    /// Twilight client for interacting with the Discord API
    pub twilight_client: Arc<twilight_http::Client>,
}

/// The core framework. Should be available from ALL tasks and holds key data.
/// Context classes generally take a reference to this to perform their actions.
#[derive(Clone)]
pub struct Framework {
    /// The caching layer of the framework
    #[cfg(feature = "cache-memory")]
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    /// Luro's database, which accepts a driver that implements [LuroDatabaseDriver]
    pub database: Arc<LuroDatabase<DatabaseEngine>>,
    /// HTTP client used for making outbound API requests
    #[cfg(feature = "http-client-hyper")]
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    /// Lavalink client, for playing music
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    /// Twilight's client for interacting with the Discord API
    pub twilight_client: Arc<twilight_http::Client>,
    /// The global tracing subscriber, for allowing manipulation within commands
    pub tracing_subscriber:
        tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    /// A mutable list of global commands, keyed by [String] (command name) and containing a [ApplicationCommandData]
    pub global_commands: LuroMutex<LuroCommandType>,
    /// A mutable list of guild commands, keyed by [GuildMarker] and containing [LuroCommand]s
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
}

/// A context sapwned only in which the event is an interaction
#[derive(Debug)]
pub struct InteractionContext {
    pub original: Interaction,
    /// App's permissions in the channel the interaction was sent from.
    ///
    /// Present when the interaction is invoked in a guild.
    pub app_permissions: Option<Permissions>,
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// The channel the interaction was invoked in.
    ///
    /// Present on all interactions types, except [`Ping`].
    ///
    /// [`Ping`]: InteractionType::Ping
    pub channel: Option<Channel>,
    /// Data from the interaction.
    ///
    /// This field present on [`ApplicationCommand`], [`MessageComponent`],
    /// [`ApplicationCommandAutocomplete`] and [`ModalSubmit`] interactions.
    /// The inner enum variant matches the interaction type.
    ///
    /// [`ApplicationCommand`]: InteractionType::ApplicationCommand
    /// [`MessageComponent`]: InteractionType::MessageComponent
    /// [`ApplicationCommandAutocomplete`]: InteractionType::ApplicationCommandAutocomplete
    /// [`ModalSubmit`]: InteractionType::ModalSubmit
    pub data: Option<InteractionData>,
    /// ID of the guild the interaction was invoked in.
    pub guild_id: Option<Id<GuildMarker>>,
    /// Guild’s preferred locale.
    ///
    /// Present when the interaction is invoked in a guild.
    pub guild_locale: Option<String>,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Type of interaction.
    pub kind: InteractionType,
    /// Selected language of the user who invoked the interaction.
    ///
    /// Present on all interactions types, except [`Ping`].
    ///
    /// [`Ping`]: InteractionType::Ping
    pub locale: Option<String>,
    /// Member that invoked the interaction.
    ///
    /// Present when the interaction is invoked in a guild.
    pub member: Option<PartialMember>,
    /// Message attached to the interaction.
    ///
    /// Present on [`MessageComponent`] interactions.
    ///
    /// [`MessageComponent`]: InteractionType::MessageComponent
    pub message: Option<Message>,
    /// Token for responding to the interaction.
    pub token: String,
    /// User that invoked the interaction.
    ///
    /// Present when the interaction is invoked in a direct message.
    pub user: Option<User>,
    /// [Latency] information about the connection to the gateway
    pub latency: twilight_gateway::Latency,
    /// A [MessageSender] used for communicating with the shard
    pub shard: twilight_gateway::MessageSender,
}

pub trait LuroInteraction {
    fn original_interaction<D: LuroDatabaseDriver>(&self) -> &Interaction;
    async fn accent_colour<D: LuroDatabaseDriver>(&self, framework: &Framework) -> u32;
    async fn acknowledge_interaction<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework,
        ephemeral: bool,
    ) -> anyhow::Result<LuroResponse>;
    async fn default_embed<D: LuroDatabaseDriver>(&self, framework: &Framework) -> EmbedBuilder;
    async fn get_interaction_author<D: LuroDatabaseDriver>(&self, framework: &Framework) -> anyhow::Result<LuroUser>;
    async fn get_specified_user_or_author<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework,
        specified_user: Option<&ResolvedUser>,
    ) -> anyhow::Result<LuroUser>;
    async fn respond_message<D, F>(&self, framework: &Framework, response: F) -> anyhow::Result<Option<Message>>
    where
        D: LuroDatabaseDriver,
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse;
    async fn respond<D, F>(&self, framework: &Framework, response: F) -> anyhow::Result<()>
    where
        D: LuroDatabaseDriver,
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse;
    async fn response_create<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework,
        response: &LuroResponse,
    ) -> anyhow::Result<Option<Message>>;
    async fn response_update<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework,
        response: &LuroResponse,
    ) -> anyhow::Result<Message>;
    async fn send_response<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework,
        response: LuroResponse,
    ) -> anyhow::Result<Option<Message>>;
    fn author_id(&self) -> Id<UserMarker>;
    fn author(&self) -> &User;
    fn guild_id(&self) -> Option<Id<GuildMarker>>;
    fn command_name(&self) -> &str;
}

/// A context spawned from a modal interaction
pub struct ModalInteraction<T> {
    pub command: T,
    pub app_permissions: Option<twilight_model::guild::Permissions>,
    pub application_id: Id<twilight_model::id::marker::ApplicationMarker>,
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    pub channel: twilight_model::channel::Channel,
    pub data: ModalInteractionData,
    pub database: Arc<LuroDatabase<DatabaseEngine>>,
    pub global_commands: LuroMutex<LuroCommandType>,
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    pub id: Id<twilight_model::id::marker::InteractionMarker>,
    pub kind: twilight_model::application::interaction::InteractionType,
    pub latency: twilight_gateway::Latency,
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    pub locale: Option<String>,
    pub member: Option<twilight_model::guild::PartialMember>,
    pub message: Option<twilight_model::channel::Message>,
    pub original: twilight_model::application::interaction::Interaction,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub tracing_subscriber:
        tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    pub twilight_client: Arc<twilight_http::Client>,
    pub user: Option<twilight_model::user::User>,
}

pub struct ComponentInteraction<T> {
    pub command: T,
    pub app_permissions: Option<twilight_model::guild::Permissions>,
    pub application_id: Id<twilight_model::id::marker::ApplicationMarker>,
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    pub channel: twilight_model::channel::Channel,
    pub data: Box<MessageComponentInteractionData>,
    pub database: Arc<LuroDatabase<DatabaseEngine>>,
    pub global_commands: LuroMutex<LuroCommandType>,
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    pub id: Id<twilight_model::id::marker::InteractionMarker>,
    pub kind: twilight_model::application::interaction::InteractionType,
    pub latency: twilight_gateway::Latency,
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    pub locale: Option<String>,
    pub member: Option<twilight_model::guild::PartialMember>,
    pub message: twilight_model::channel::Message,
    pub original: twilight_model::application::interaction::Interaction,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub tracing_subscriber:
        tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    pub twilight_client: Arc<twilight_http::Client>,
    pub user: Option<twilight_model::user::User>,
}

/// A trait that enforces the things you can access in ANY context
pub trait Luro {
    /// Gets the [interaction client](InteractionClient) using this framework's
    /// [http client](Client) and [application id](ApplicationMarker)
    async fn interaction_client(&self) -> anyhow::Result<InteractionClient>;

    /// Register commands to the Discord API.
    async fn register_commands(&self, commands: &[Command]) -> anyhow::Result<()> {
        let client = self.interaction_client().await?;

        match client.set_global_commands(commands).await {
            Ok(command_result) => Ok(info!(
                "Successfully registered {} global commands!",
                command_result.model().await?.len()
            )),
            Err(why) => Err(why.into()),
        }
    }
}
