#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]

use anyhow::anyhow;
use luro_database::LuroDatabase;
use luro_model::{builders::EmbedBuilder, guild::LuroGuild, response::LuroResponse, user::LuroUser, role::LuroRole};
use slash_command::LuroCommand;
use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, Mutex},
};
use tracing::{info, warn};
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::{client::InteractionClient, Client};
use twilight_interactions::command::ResolvedUser;
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    guild::Role,
    id::marker::{RoleMarker, UserMarker},
    oauth::Application,
};
use twilight_util::permission_calculator::PermissionCalculator;

use twilight_model::{
    application::interaction::{
        application_command::CommandData, message_component::MessageComponentInteractionData, modal::ModalInteractionData,
    },
    channel::Message,
    id::{marker::GuildMarker, Id},
    user::User,
};

pub mod command;
pub mod context;
mod framework;
pub mod interactions;
#[cfg(feature = "responses")]
pub mod responses;
pub mod slash_command;

type LuroCommandType = HashMap<String, LuroCommand>;
type LuroMutex<T> = Arc<Mutex<T>>;

#[derive(Clone)]
pub enum InteractionContext {
    CommandInteraction(CommandInteraction),
    CommandAutocompleteInteraction(CommandInteraction),
    ComponentInteraction(ComponentInteraction),
    ModalInteraction(ModalInteraction),
}

/// A context spawned from a command interaction
#[derive(Clone)]
pub struct CommandInteraction {
    pub app_permissions: Option<twilight_model::guild::Permissions>,
    pub application_id: Id<twilight_model::id::marker::ApplicationMarker>,
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    pub channel: twilight_model::channel::Channel,
    pub data: Box<CommandData>,
    pub database: Arc<LuroDatabase>,
    pub global_commands: LuroMutex<LuroCommandType>,
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    pub id: Id<twilight_model::id::marker::InteractionMarker>,
    pub kind: twilight_model::application::interaction::InteractionType,
    pub latency: twilight_gateway::Latency,
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    pub locale: Option<String>,
    pub member: Option<twilight_model::guild::PartialMember>,
    pub message: Option<twilight_model::channel::Message>,
    // pub original: twilight_model::application::interaction::Interaction,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub tracing_subscriber: tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
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
    pub database: Arc<LuroDatabase>,
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
    pub tracing_subscriber: tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
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
    /// Luro's database driver
    pub database: Arc<LuroDatabase>,
    /// HTTP client used for making outbound API requests
    #[cfg(feature = "http-client-hyper")]
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    /// Lavalink client, for playing music
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    /// Twilight's client for interacting with the Discord API
    pub twilight_client: Arc<twilight_http::Client>,
    /// The global tracing subscriber, for allowing manipulation within commands
    pub tracing_subscriber: tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    /// A mutable list of global commands, keyed by [String] (command name) and containing a [ApplicationCommandData]
    pub global_commands: LuroMutex<LuroCommandType>,
    /// A mutable list of guild commands, keyed by [GuildMarker] and containing [LuroCommand]s
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
}

pub trait LuroInteraction {
    fn original_interaction(&self) -> &Interaction;
    fn accent_colour(&self, framework: &Framework) -> impl Future<Output = u32> + Send;
    fn acknowledge_interaction(
        &self,
        framework: &Framework,
        ephemeral: bool,
    ) -> impl Future<Output = anyhow::Result<LuroResponse>> + Send;
    fn default_embed(&self, framework: &Framework) -> impl Future<Output = EmbedBuilder> + Send;
    fn get_interaction_author(&self, framework: &Framework) -> impl Future<Output = anyhow::Result<LuroUser>> + Send;
    fn get_specified_user_or_author(
        &self,
        framework: &Framework,
        specified_user: Option<&ResolvedUser>,
    ) -> impl Future<Output = anyhow::Result<LuroUser>> + Send;
    fn respond_message<F>(&self, framework: &Framework, response: F) -> impl Future<Output = anyhow::Result<Option<Message>>> + Send
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse;
    fn respond<F>(&self, framework: &Framework, response: F) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse;
    fn response_create(
        &self,
        framework: &Framework,
        response: &LuroResponse,
    ) -> impl Future<Output = anyhow::Result<Option<Message>>> + Send;
    fn response_update(&self, framework: &Framework, response: &LuroResponse) -> impl Future<Output = anyhow::Result<Message>> + Send;
    fn send_response(
        &self,
        framework: &Framework,
        response: LuroResponse,
    ) -> impl Future<Output = anyhow::Result<Option<Message>>> + Send;
    fn author_id(&self) -> Id<UserMarker>;
    fn author(&self) -> &User;
    fn guild_id(&self) -> Option<Id<GuildMarker>>;
    fn command_name(&self) -> &str;
}

/// A context spawned from a modal interaction
#[derive(Debug, Clone)]
pub struct ModalInteraction {
    pub app_permissions: Option<twilight_model::guild::Permissions>,
    pub application_id: Id<twilight_model::id::marker::ApplicationMarker>,
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    pub channel: twilight_model::channel::Channel,
    pub data: ModalInteractionData,
    pub database: Arc<LuroDatabase>,
    pub global_commands: LuroMutex<LuroCommandType>,
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    pub id: Id<twilight_model::id::marker::InteractionMarker>,
    pub kind: twilight_model::application::interaction::InteractionType,
    pub latency: twilight_gateway::Latency,
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    pub locale: Option<String>,
    pub member: Option<twilight_model::guild::PartialMember>,
    pub message: Option<twilight_model::channel::Message>,
    pub original: twilight_model::application::interaction::Interaction,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub tracing_subscriber: tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    pub twilight_client: Arc<twilight_http::Client>,
    pub user: Option<twilight_model::user::User>,
}

#[derive(Debug, Clone)]
pub struct ComponentInteraction {
    pub app_permissions: Option<twilight_model::guild::Permissions>,
    pub application_id: Id<twilight_model::id::marker::ApplicationMarker>,
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    pub channel: twilight_model::channel::Channel,
    pub data: Box<MessageComponentInteractionData>,
    pub database: Arc<LuroDatabase>,
    pub global_commands: LuroMutex<LuroCommandType>,
    pub guild_commands: LuroMutex<HashMap<Id<GuildMarker>, LuroCommandType>>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    pub id: Id<twilight_model::id::marker::InteractionMarker>,
    pub kind: twilight_model::application::interaction::InteractionType,
    pub latency: twilight_gateway::Latency,
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    pub locale: Option<String>,
    pub member: Option<twilight_model::guild::PartialMember>,
    pub message: twilight_model::channel::Message,
    pub original: twilight_model::application::interaction::Interaction,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub tracing_subscriber: tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    pub twilight_client: Arc<twilight_http::Client>,
    pub user: Option<twilight_model::user::User>,
}

/// A trait that enforces the things you can access in ANY context
pub trait Luro {
    /// Gets the [interaction client](InteractionClient) using this framework's
    /// [http client](Client) and [application id](ApplicationMarker)
    fn interaction_client(&self) -> impl Future<Output = anyhow::Result<InteractionClient>> + Send;

    fn application(&self) -> impl Future<Output = anyhow::Result<Application>> + Send
    where
        Self: Sync,
    {
        async { Ok(self.twilight_client().current_user_application().await?.model().await?) }
    }

    /// Register commands to the Discord API.
    fn register_commands(&self, commands: &[Command]) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        Self: Sync,
    {
        async {
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

    /// Returns the database used by this context
    fn database(&self) -> Arc<LuroDatabase>;

    /// Returns the twilight_client used by this context
    fn twilight_client(&self) -> Arc<Client>;

    /// Returns the cache used by this context
    fn cache(&self) -> Arc<InMemoryCache>;

    /// Fetch and return a [LuroGuild], updating the database if not present
    /// Luro Database -> Twilight Guild
    fn get_guild(&self, guild_id: &Id<GuildMarker>) -> impl Future<Output = anyhow::Result<LuroGuild>> + Send
    where
        Self: Sync,
    {
        async {
            Ok(match self.database().get_guild(guild_id.get() as i64).await? {
                Some(guild) => guild.into(),
                None => self
                    .database()
                    .update_guild(self.twilight_client().guild(*guild_id).await?.model().await?)
                    .await?
                    .into(),
            })
        }
    }

    fn get_guilds(&self) -> impl Future<Output = anyhow::Result<Vec<LuroGuild>>> + Send
    where
        Self: Sync,
    {
        async {
            Ok(self
                .database()
                .get_all_guilds()
                .await
                .map(|x| x.into_iter().map(|x| x.into()).collect())?)
        }
    }

    /// Fetch and return a [LuroGuild], updating the database if not present
    /// Luro Database -> Twilight Client
    fn get_user(&self, user_id: &Id<UserMarker>) -> impl Future<Output = anyhow::Result<LuroUser>> + Send
    where
        Self: Sync,
    {
        async {
            Ok(match self.database().get_user(user_id.get() as i64).await? {
                Some(user) => user.into(),
                None => {
                    let user = self.twilight_client().user(*user_id).await?.model().await?;
                    match self.database().update_user(user.clone()).await {
                        Ok(db_user) => match db_user {
                            Some(user) => user.into(),
                            None => {
                                warn!("User did not exist in the database, attempted to update the user but the database still did not return the user");
                                (&user).into()
                            }
                        },
                        Err(why) => {
                            warn!(why = ?why, "Failed to get user, falling back to twilight");
                            (&user).into()
                        }
                    }
                }
            })
        }
    }

    /// Fetch and return a [LuroGuild], updating the database if not present
    /// Luro Database -> Twilight Cache
    fn get_role(&self, role_id: Id<RoleMarker>) -> impl Future<Output = anyhow::Result<Role>> + Send
    where
        Self: Sync,
    {
        async move {
            if let Ok(Some(role)) = self.database().get_role(role_id.get() as i64).await {
                return Ok(role.into());
            }

            let cache = self.cache();
            let cached_role = match cache.role(role_id) {
                Some(role) => role,
                None => return Err(anyhow!("No role referance in cache or database: {role_id}")),
            };

            Ok(self
                .database()
                .update_role((cached_role.guild_id(), cached_role.resource().clone()))
                .await?
                .into())
        }
    }

    /// Fetch all guild roles. Set bypass to true to force a flush of all roles, if you want to make sure we have the most up to date roles possible, such as for highly privileged commands.
    async fn get_guild_roles(&self, guild_id: &Id<GuildMarker>, bypass: bool) -> anyhow::Result<Vec<Role>>
    where
        Self: Sync,
    {
        // Get fresh roles at user request
        if bypass {
            let roles = self.twilight_client().roles(*guild_id).await?.model().await?;

            for role in &roles {
                self.database().update_role((*guild_id, role.clone())).await?;
            }

            return Ok(roles);
        }

        // Get from database
        if let Ok(roles) = self.database().get_guild_roles(guild_id.get() as i64).await {
            return Ok(roles.into_iter().map(|x|x.into()).collect::<Vec<Role>>())
        }

        // Database failed, fetch from client.
        info!("Failed to find guild roles for guild {guild_id}, fetching using twilight_client");
        let roles = self.twilight_client().roles(*guild_id).await?.model().await?;

        for role in &roles {
            self.database().update_role((*guild_id, role.clone())).await?;
        }

        Ok(roles)
    }

    // async fn get_guild_member_roles(&self, guild_id: &Id<GuildMarker>, user_id: &Id<UserMarker>, bypass: bool) -> anyhow::Result<Vec<Role>>
    // where
    //     Self: Sync,
    // {
    //     let guild_roles = self.get_guild_roles(guild_id, true).await?;


    // }

    // async fn user_permission_calculator(&self, user_id: Id<UserMarker>, guild_id: Id<GuildMarker>) -> anyhow::Result<PermissionCalculator>
    // where
    //     Self: Sync,
    // {
    //     let roles = self.get_guild_roles(&guild_id, true).await?;
    //     let guild = self.get_guild(&guild_id).await?;
    //     let user = self.get_user(&user_id).await?;

    //     // Temp
    //     let everyone: LuroRole = self.get_role(guild_id.cast()).await?.into();

    //     Ok(PermissionCalculator::new(guild_id, user_id, everyone.permissions, &guild.user_role_permissions(&user)).owner_id(guild.owner_id))
    // }
}
