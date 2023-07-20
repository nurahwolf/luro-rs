use std::{collections::HashMap, convert::TryInto, net::SocketAddr, str::FromStr, sync::Arc};

use anyhow::Error;
use hyper::client::HttpConnector;
use parking_lot::RwLock;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{reload::Handle, Registry};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{stream, ConfigBuilder, Intents, Shard};
use twilight_http::{client::InteractionClient, Client};
use twilight_lavalink::Lavalink;
use twilight_model::{
    application::command::Command,
    gateway::{
        payload::outgoing::update_presence::UpdatePresencePayload,
        presence::{ActivityType, MinimalActivity, Status}
    },
    id::{
        marker::{GuildMarker, UserMarker},
        Id
    },
    oauth::Application
};

use crate::{
    commands::Commands,
    guild::{LuroGuild, LuroGuilds},
    hecks::Hecks,
    HECK_FILE_PATH
};

pub struct GlobalData {
    /// Simply used as a test to make sure that data is shared across threads
    pub count: usize,
    pub hecks: Hecks,
    pub owners: Vec<Id<UserMarker>>
}

/// The framework used to dispatch slash commands.
pub struct LuroFramework {
    /// Application data returned by Discord on initial bot startup
    pub application: Application,
    /// HTTP client used for making outbound API requests
    pub hyper_client: hyper::Client<HttpConnector>,
    /// Lavalink client, for playing music
    pub lavalink: Lavalink,
    /// Twilight's client for interacting with the Discord API
    pub twilight_client: Client,
    /// Twilight's cache
    pub twilight_cache: InMemoryCache,
    /// A map of simple commands.
    pub commands: Commands,
    /// Guild specific stuff
    pub guilds: RwLock<HashMap<Id<GuildMarker>, LuroGuild>>,
    /// Mutable data used throughout Luro
    pub global_data: RwLock<GlobalData>,
    pub tracing_subscriber: Handle<LevelFilter, Registry>
}

impl LuroFramework {
    /// Creates a new framework builder, this is a shortcut to FrameworkBuilder.
    /// [new](crate::builder::FrameworkBuilder::new)
    pub async fn builder(
        commands: Commands,
        intents: Intents,
        lavalink_auth: String,
        lavalink_host: String,
        token: String,
        tracing_subscriber: Handle<LevelFilter, Registry>
    ) -> Result<(Arc<Self>, Vec<Shard>), Error> {
        let (twilight_client, twilight_cache, shard_config) = (
            twilight_http::Client::new(token.clone()),
            InMemoryCache::new(),
            ConfigBuilder::new(token, intents)
                .presence(UpdatePresencePayload::new(
                    vec![MinimalActivity {
                        kind: ActivityType::Playing,
                        name: "/about | Hello World!".to_owned(),
                        url: None
                    }
                    .into()],
                    false,
                    None,
                    Status::Online
                )?)
                .build()
        );

        let shards = stream::create_recommended(&twilight_client, shard_config, |_, c| c.build())
            .await?
            .collect::<Vec<_>>();

        let current_user = twilight_client.current_user().await?.model().await?;
        let application = twilight_client.current_user_application().await?.model().await?;

        let lavalink = {
            let socket = SocketAddr::from_str(&lavalink_host)?;
            let lavalink = Lavalink::new(current_user.id, shards.len().try_into()?);
            lavalink.add(socket, lavalink_auth).await?;
            lavalink
        };

        let hyper_client = hyper::Client::new();
        let guilds = LuroGuilds::get().await?.guilds.into();
        let hecks = Hecks::get(HECK_FILE_PATH).await?;
        let owners = vec![Id::new(97003404601094144)];
        let global_data = GlobalData { count: 0, hecks, owners }.into();

        Ok((
            Self {
                application,
                twilight_client,
                twilight_cache,
                hyper_client,
                lavalink,
                commands,
                guilds,
                global_data,
                tracing_subscriber
            }
            .into(),
            shards
        ))
    }

    /// Registers all commands in `commands.global_commands`, returning the commands that were registered.
    pub async fn register_global_commands(&self) -> Result<Vec<Command>, Error> {
        let mut commands = Vec::new();

        for cmd in self.commands.global_commands.values() {
            // Interaction client for registering the commands with
            let interaction_client = self.interaction_client();

            // Create and register a command
            let mut command = interaction_client
                .create_global_command()
                .chat_input(cmd.name.as_str(), cmd.description.as_str())?
                .command_options(&cmd.options)?;

            // Affix default permissions if set
            if let Some(permissions) = &cmd.default_member_permissions {
                command = command.default_member_permissions(*permissions);
            }

            // Push our created command to a vector that should be returned
            commands.push(command.await?.model().await?);
        }

        Ok(commands)
    }

    /// Gets the [interaction client](InteractionClient) using this framework's
    /// [http client](Client) and [application id](ApplicationMarker)
    pub fn interaction_client(&self) -> InteractionClient {
        self.twilight_client.interaction(self.application.id)
    }
}