use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, RwLock},
};

use anyhow::{Context, Error};
use hyper::client::HttpConnector;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{
    stream::{self},
    ConfigBuilder, Intents, Shard,
};
use twilight_http::client::InteractionClient;
use twilight_lavalink::Lavalink;
use twilight_model::{
    application::command::Command,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    oauth::Application,
};

use crate::{commands::commands, HECK_FILE_PATH};

use super::{hecks::Hecks, GlobalCommands, GuildSettings, LuroError, UserSettings};

/// Luro's core, containing all the data needed to make things work!
pub struct Luro {
    /// Information about the bot direct from Discord, loaded on firt initialisation
    pub application: RwLock<Application>,
    /// Twilight client, for interacting with the Discord API
    pub twilight_client: twilight_http::Client,
    /// Twilight cache, for useful cached things!
    pub twilight_cache: InMemoryCache,
    /// Lavalink client, for playing music
    pub lavalink: Lavalink,
    /// Luro's direct HTTP client, for making API requests and interacting with lavalink
    pub hyper_client: hyper::Client<HttpConnector>,
    /// A hashmap containing the number of commands run. The entry `total` includes the amount of all commands run.
    pub command_usage: RwLock<HashMap<String, usize>>,
    /// A hashmap containing ettings per guild
    pub guild_settings: RwLock<HashMap<Id<GuildMarker>, GuildSettings>>,
    /// A hashmap containing settings per user
    pub user_settings: RwLock<HashMap<Id<UserMarker>, UserSettings>>,
    /// Data for commands that are spread cross-server
    pub global_command_data: GlobalCommands,
    /// A vector of commands that are loaded globally
    pub global_commands: RwLock<Vec<Command>>,
}

/// Key Luro implementation
impl Luro {
    /// Create a new instance of Luro.
    pub async fn new() -> Result<(Arc<Self>, Vec<Shard>), Error> {
        let (token, lavalink_host, lavalink_auth, intents) = (
            env::var("DISCORD_TOKEN").context("Failed to get the variable DISCORD_TOKEN")?,
            env::var("LAVALINK_HOST").context("Failed to get the variable LAVALINK_HOST")?,
            env::var("LAVALINK_AUTHORISATION")
                .context("Failed to get the variable LAVALINK_AUTHORISATION")?,
            Intents::GUILD_MESSAGES
                | Intents::GUILD_VOICE_STATES
                | Intents::MESSAGE_CONTENT
                | Intents::GUILD_INVITES,
        );

        let (twilight_client, config) = (
            twilight_http::Client::new(token.clone()),
            ConfigBuilder::new(token.clone(), intents).build(),
        );

        let shards = stream::create_recommended(&twilight_client, config, |_, c| c.build())
            .await?
            .collect::<Vec<_>>();

        let (current_user, application) = (
            twilight_client.current_user().await?.model().await?,
            twilight_client
                .current_user_application()
                .await?
                .model()
                .await?
                .into(),
        );

        let lavalink = {
            let socket = SocketAddr::from_str(&lavalink_host)?;
            let lavalink = Lavalink::new(current_user.id, shards.len().try_into()?);
            lavalink.add(socket, lavalink_auth).await?;
            lavalink
        };

        let hyper_client = hyper::Client::new();
        let global_commands = commands().into();
        let global_command_data = GlobalCommands {
            global_hecks: Hecks::get(HECK_FILE_PATH).await?.into(),
        };
        let command_usage = HashMap::new().into();
        let guild_settings = HashMap::new().into();
        let user_settings = HashMap::new().into();
        let twilight_cache = InMemoryCache::new();

        Ok((
            Self {
                application,
                twilight_client,
                lavalink,
                hyper_client,
                command_usage,
                guild_settings,
                user_settings,
                global_command_data,
                global_commands,
                twilight_cache,
            }
            .into(),
            shards,
        ))
    }

    /// Get an interaction client
    pub async fn create_interaction_client<'a>(
        twilight_client: &'a twilight_http::Client,
        application_data: &'a RwLock<Application>,
    ) -> Result<InteractionClient<'a>, LuroError> {
        match application_data.read() {
            Ok(application_data) => Ok(twilight_client.interaction(application_data.id)),
            Err(_) => Err(LuroError::NoApplicationData),
        }
    }
}
