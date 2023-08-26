#![feature(async_fn_in_trait)]
use interaction_context::LuroInteraction;
use luro_model::ACCENT_COLOUR;
use twilight_interactions::command::ApplicationCommandData;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};
use tracing::warn;
use luro_builder::embed::EmbedBuilder;
use twilight_model::{http::interaction::InteractionResponseType, application::interaction::{InteractionData, InteractionType, Interaction}};
use luro_model::response::LuroResponse;

use luro_model::database::{drivers::LuroDatabaseDriver, LuroDatabase};
use twilight_model::{
    application::interaction::{
            application_command::CommandData, message_component::MessageComponentInteractionData, modal::ModalInteractionData
        },
    guild::{PartialMember, Permissions},
    id::{
        marker::{GuildMarker, InteractionMarker, ApplicationMarker},
        Id
    },
    user::User, channel::{Channel, Message}
};

mod command;
mod context;
mod framework;
pub mod interaction_context;
#[cfg(feature = "responses")]
pub mod responses;

type LuroCommand = HashMap<String, ApplicationCommandData>;


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
    pub global_commands: Arc<Mutex<LuroCommand>>,
    /// A mutable list of guild commands, keyed by [GuildMarker] and containing [LuroCommand]s
    pub guild_commands: Arc<Mutex<HashMap<Id<GuildMarker>, LuroCommand>>>,
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
            channel: interaction.channel,
            data: interaction.data,
            guild_id: interaction.guild_id,
            guild_locale: interaction.guild_locale,
            id: interaction.id,
            kind: interaction.kind,
            locale: interaction.locale,
            member: interaction.member,
            message: interaction.message,
            token: interaction.token,
            user: interaction.user,
            latency: ctx.latency,
            shard: ctx.shard,
        }
    }
}

#[derive(Clone)]
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

macro_rules! interaction_methods {
    ($($ty:ty,)*) => {
        $(
            impl LuroInteraction for $ty {
    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    async fn default_embed<D: LuroDatabaseDriver>(&self, ctx: &Framework<D>) -> EmbedBuilder {
        ctx.default_embed(self.guild_id.as_ref()).await
    }

    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    async fn accent_colour<D: LuroDatabaseDriver>(&self, ctx: &Framework<D>) -> u32 {
        match self.guild_id {
            Some(guild_id) => ctx
                .guild_accent_colour(&guild_id)
                .await
                .map(|x| x.unwrap_or(ACCENT_COLOUR)) // Guild has no accent colour
                .unwrap_or(ACCENT_COLOUR), // We had an error getting the guild's accent colour
            None => ACCENT_COLOUR // There is no guild for this interaction
        }
    }

        /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    async fn respond<D, F>(&self, ctx: &Framework<D>, response: F) -> anyhow::Result<Option<Message>>
    where
        D: LuroDatabaseDriver,
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        match r.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource
            || r.interaction_response_type == InteractionResponseType::DeferredUpdateMessage
        {
            true => Ok(Some(self.response_update(ctx, &r).await?)),
            false => self.response_create(ctx, &r).await
        }
    }

    /// Create a response. This is used for sending a response to an interaction, as well as to defer interactions.
    /// This CANNOT be used to update a response! Use `response_update` for that!
    async fn response_create<D: LuroDatabaseDriver>(
        &self,
        ctx: &Framework<D>,
        response: &LuroResponse
    ) -> anyhow::Result<Option<Message>> {
        let client = ctx.interaction_client(self.application_id);
        let request = response.interaction_response();

        match client.create_response(self.id, &self.token, &request).await {
            Ok(_) => Ok(None),
            Err(why) => {
                warn!(why = ?why, "Failed to send a response to an interaction, attempting to send as an update");
                Ok(Some(self.response_update(ctx, response).await?))
            }
        }
    }

    /// Update an existing response
    async fn response_update<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework<D>,
        response: &LuroResponse
    ) -> anyhow::Result<Message> {
        Ok(framework
            .interaction_client(self.application_id)
            .update_response(&self.token)
            .allowed_mentions(response.allowed_mentions.as_ref())
            .components(response.components.as_deref())
            .content(response.content.as_deref())
            .embeds(response.embeds.as_deref())
            .await?
            .model()
            .await?)
    }

    /// Send an existing response builder a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    async fn send_response<D: LuroDatabaseDriver>(
        &self,
        ctx: &Framework<D>,
        response: LuroResponse
    ) -> anyhow::Result<Option<Message>> {
        self.respond(ctx, |r| {
            *r = response;
            r
        })
        .await
    }

            }
        )*
    };
}

interaction_methods! {
    InteractionContext,
    InteractionCommand,
    InteractionComponent,
    InteractionModal,
}