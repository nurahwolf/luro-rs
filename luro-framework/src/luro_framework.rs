use std::{fs, path::PathBuf, sync::Arc};

use luro_database::Database;
use luro_model::configuration::Configuration;
use twilight_gateway::{stream, Shard};
use twilight_model::id::{marker::{GuildMarker, UserMarker}, Id};

use crate::{Luro, LuroContext};

#[cfg(feature = "luro-builder")]
mod default_embed;
mod register_commands;
#[cfg(feature = "luro-builder")]
mod send_log_channel;
#[cfg(feature = "luro-builder")]
mod send_message;
mod webhook;

/// The core framework. Should be available from ALL tasks and holds key data.
/// Context classes generally take a reference to this to perform their actions.
#[derive(Clone)]
pub struct Framework {
    /// Luro's database driver
    pub database: Arc<Database>,
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
}

impl Framework {
    pub async fn new(config: &Configuration) -> anyhow::Result<(Framework, Vec<Shard>)> {
        // Ensure data directory exists on disk
        ensure_data_directory_exists();

        let current_user = config.twilight_client.current_user().await?.model().await?;
        let database = initialise_database(config, current_user.id).await?.into();
        let shards = stream::create_recommended(&config.twilight_client, config.shard_config.clone(), |_, c| c.build())
            .await?
            .collect::<Vec<_>>();

        #[cfg(feature = "lavalink")]
        let lavalink = {
            let socket = <std::net::SocketAddr as std::str::FromStr>::from_str(&config.lavalink_host)?;
            let lavalink = twilight_lavalink::Lavalink::new(current_user.id, shards.len().try_into()?);

            tracing::info!(
                "Connecting to lavalink with Socket + Auth: '{}' - '{}'",
                socket,
                config.lavalink_auth
            );
            lavalink.add(socket, &config.lavalink_auth).await?;
            lavalink.into()
        };

        #[cfg(feature = "http-client-hyper")]
        let http_client = hyper::Client::new().into();

        let framework = Self {
            #[cfg(feature = "http-client-hyper")]
            http_client,
            database,
            #[cfg(feature = "lavalink")]
            lavalink,
            tracing_subscriber: config.tracing_subscriber.clone(),
            twilight_client: config.twilight_client.clone(),
        };

        Ok((framework, shards))
    }
}

impl Luro for Framework {
    fn guild_id(&self) -> Option<Id<GuildMarker>> {
        None
    }

    async fn interaction_client(&self) -> anyhow::Result<twilight_http::client::InteractionClient> {
        Ok(self
            .twilight_client
            .interaction(self.twilight_client.current_user_application().await?.model().await?.id))
    }

    fn database(&self) -> std::sync::Arc<Database> {
        self.database.clone()
    }

    fn twilight_client(&self) -> std::sync::Arc<twilight_http::Client> {
        self.twilight_client.clone()
    }

}

impl From<LuroContext> for Framework {
    fn from(framework: LuroContext) -> Self {
        Self {
            database: framework.database,
            http_client: framework.http_client,
            #[cfg(feature = "lavalink")]
            lavalink: framework.lavalink,
            tracing_subscriber: framework.tracing_subscriber,
            twilight_client: framework.twilight_client,
        }
    }
}

async fn initialise_database(config: &Configuration, current_user: Id<UserMarker>) -> anyhow::Result<Database> {
    Database::new(config, current_user).await
}

fn ensure_data_directory_exists() {
    // TODO: Variable
    let path_to_data = PathBuf::from("./data");

    // Initialise /data folder for toml. Otherwise it panics.
    if !path_to_data.exists() {
        tracing::warn!("/data folder does not exist, creating it...");
        fs::create_dir(path_to_data).expect("Failed to make data subfolder");
        tracing::info!("/data folder successfully created!");
    }
}
