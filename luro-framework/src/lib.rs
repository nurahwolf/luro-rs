#![feature(async_fn_in_trait)]
use luro_builder::embed::EmbedBuilder;
use luro_model::{response::LuroResponse, user::LuroUser};
use slash_command::LuroCommand;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};
use twilight_interactions::command::ResolvedUser;
use twilight_model::{
    application::interaction::{Interaction, InteractionData, InteractionType},
    id::marker::UserMarker
};

use luro_model::database::{drivers::LuroDatabaseDriver, LuroDatabase};
use twilight_model::{
    application::interaction::{
        application_command::CommandData, message_component::MessageComponentInteractionData, modal::ModalInteractionData
    },
    channel::{Channel, Message},
    guild::{PartialMember, Permissions},
    id::{
        marker::{ApplicationMarker, GuildMarker, InteractionMarker},
        Id
    },
    user::User
};

pub mod command;
pub mod context;
mod framework;
mod interactions;
#[cfg(feature = "responses")]
pub mod responses;
pub mod slash_command;

type LuroCommandType<D> = HashMap<String, LuroCommand<D>>;
type LuroMutex<T> = Arc<Mutex<T>>;

/// The core framework. Should be available from ALL tasks and holds key data.
/// Context classes generally take a reference to this to perform their actions.
#[derive(Clone)]
pub struct Framework<D: LuroDatabaseDriver> {
    /// The caching layer of the framework
    #[cfg(feature = "cache-memory")]
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    /// Luro's database, which accepts a driver that implements [LuroDatabaseDriver]
    pub database: Arc<LuroDatabase<D>>,
    /// HTTP client used for making outbound API requests
    #[cfg(feature = "http-client-hyper")]
    pub http_client: hyper::Client<hyper::client::HttpConnector>,
    /// Lavalink client, for playing music
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    /// Twilight's client for interacting with the Discord API
    pub twilight_client: Arc<twilight_http::Client>,
    /// The global tracing subscriber, for allowing manipulation within commands
    pub tracing_subscriber:
        tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    /// A mutable list of global commands, keyed by [String] (command name) and containing a [ApplicationCommandData]
    pub global_commands: LuroMutex<LuroCommandType<D>>,
    /// A mutable list of guild commands, keyed by [GuildMarker] and containing [LuroCommand]s
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType<D>>>
}

/// Luro's primary context, which is instanced per event.
/// More specific contexts are spawned where possible, such as [InteractionContext]
pub struct Context {
    /// [Latency] information about the connection to the gateway
    pub latency: twilight_gateway::Latency,
    /// A [MessageSender] used for communicating with the shard
    pub shard: twilight_gateway::MessageSender
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
    pub shard: twilight_gateway::MessageSender
}

impl InteractionContext {
    pub fn new(interaction: Interaction, ctx: Context) -> Self {
        Self {
            app_permissions: interaction.app_permissions,
            application_id: interaction.application_id,
            channel: interaction.channel.clone(),
            data: interaction.data.clone(),
            guild_id: interaction.guild_id,
            guild_locale: interaction.guild_locale.clone(),
            id: interaction.id,
            kind: interaction.kind,
            latency: ctx.latency,
            locale: interaction.locale.clone(),
            member: interaction.member.clone(),
            message: interaction.message.clone(),
            original: interaction.clone(),
            shard: ctx.shard,
            token: interaction.token,
            user: interaction.user
        }
    }
}

#[derive(Clone, Debug)]
pub struct InteractionCommand {
    pub application_id: Id<ApplicationMarker>,
    pub channel: Option<Channel>,
    pub data: Box<CommandData>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<InteractionMarker>,
    pub latency: twilight_gateway::Latency,
    pub member: Option<PartialMember>,
    pub permissions: Option<Permissions>,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub user: Option<User>
}

#[derive(Clone)]
pub struct InteractionComponent {
    pub original: Interaction,
    pub application_id: Id<ApplicationMarker>,
    pub channel: Option<Channel>,
    pub data: Box<MessageComponentInteractionData>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<InteractionMarker>,
    pub latency: twilight_gateway::Latency,
    pub member: Option<PartialMember>,
    pub message: Message,
    pub permissions: Option<Permissions>,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub user: Option<User>
}

#[derive(Clone)]
pub struct InteractionModal {
    pub application_id: Id<ApplicationMarker>,
    pub channel: Option<Channel>,
    pub data: ModalInteractionData,
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<InteractionMarker>,
    pub latency: twilight_gateway::Latency,
    pub member: Option<PartialMember>,
    pub message: Option<Message>,
    pub permissions: Option<Permissions>,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub user: Option<User>
}
pub trait LuroInteraction {
    async fn accent_colour<D: LuroDatabaseDriver>(&self, framework: &Framework<D>) -> u32;
    async fn acknowledge_interaction<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework<D>,
        ephemeral: bool
    ) -> anyhow::Result<LuroResponse>;
    async fn default_embed<D: LuroDatabaseDriver>(&self, framework: &Framework<D>) -> EmbedBuilder;
    async fn get_interaction_author<D: LuroDatabaseDriver>(&self, framework: &Framework<D>) -> anyhow::Result<LuroUser>;
    async fn get_specified_user_or_author<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework<D>,
        specified_user: Option<&ResolvedUser>
    ) -> anyhow::Result<LuroUser>;
    async fn respond_message<D, F>(&self, framework: &Framework<D>, response: F) -> anyhow::Result<Option<Message>>
    where
        D: LuroDatabaseDriver,
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse;
    async fn respond<D, F>(&self, framework: &Framework<D>, response: F) -> anyhow::Result<()>
    where
        D: LuroDatabaseDriver,
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse;
    async fn response_create<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework<D>,
        response: &LuroResponse
    ) -> anyhow::Result<Option<Message>>;
    async fn response_update<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework<D>,
        response: &LuroResponse
    ) -> anyhow::Result<Message>;
    async fn send_response<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework<D>,
        response: LuroResponse
    ) -> anyhow::Result<Option<Message>>;
    fn author_id(&self) -> Id<UserMarker>;
    fn author(&self) -> &User;
    fn guild_id(&self) -> Option<Id<GuildMarker>>;
    fn command_name(&self) -> &str;
}
