use std::{fs, path::PathBuf, sync::Arc};

use luro_database::LuroDatabase;
use luro_model::configuration::Configuration;
use tracing_subscriber::{filter::LevelFilter, reload::Handle, Registry};

use twilight_gateway::{stream, Shard};

use twilight_model::id::{marker::UserMarker, Id};

use crate::{Context, DatabaseEngine, Framework};

#[cfg(feature = "luro-builder")]
mod default_embed;
mod guild_accent_colour;
mod interaction_client;
mod register_commands;
#[cfg(feature = "luro-builder")]
mod send_log_channel;
#[cfg(feature = "luro-builder")]
mod send_message;
mod webhook;

impl Framework {
    pub async fn new(
        config: Arc<Configuration<DatabaseEngine>>,
        tracing_subscriber: Handle<LevelFilter, Registry>,
    ) -> anyhow::Result<(Framework, Vec<Shard>)> {
        // Ensure data directory exists on disk
        ensure_data_directory_exists();

        let (database, current_user_id) = initialise_database(config.clone()).await?;
        let shards = stream::create_recommended(&config.twilight_client, config.shard_config.clone(), |_, c| c.build())
            .await?
            .collect::<Vec<_>>();

        #[cfg(feature = "lavalink")]
        let lavalink = {
            let socket = <std::net::SocketAddr as std::str::FromStr>::from_str(&config.lavalink_host)?;
            let lavalink = twilight_lavalink::Lavalink::new(current_user_id, shards.len().try_into()?);
            lavalink.add(socket, &config.lavalink_auth).await?;
            lavalink.into()
        };

        #[cfg(feature = "http-client-hyper")]
        let http_client = hyper::Client::new().into();

        let framework = Self {
            #[cfg(feature = "cache-memory")]
            cache: config.cache.clone(),
            #[cfg(feature = "http-client-hyper")]
            http_client,
            database,
            global_commands: Default::default(),
            guild_commands: Default::default(),
            #[cfg(feature = "lavalink")]
            lavalink,
            tracing_subscriber,
            twilight_client: config.twilight_client.clone(),
        };

        Ok((framework, shards))
    }
}

impl From<Context> for Framework {
    fn from(framework: Context) -> Self {
        Self {
            cache: framework.cache,
            database: framework.database,
            global_commands: framework.global_commands,
            guild_commands: framework.guild_commands,
            http_client: framework.http_client,
            lavalink: framework.lavalink,
            tracing_subscriber: framework.tracing_subscriber,
            twilight_client: framework.twilight_client,
        }
    }
}

async fn initialise_database(
    config: Arc<Configuration<DatabaseEngine>>,
) -> anyhow::Result<(Arc<LuroDatabase<DatabaseEngine>>, Id<UserMarker>)> {
    let current_user = config.twilight_client.current_user().await?.model().await?;
    let current_user_id = current_user.id;
    Ok((LuroDatabase::build(config).into(), current_user_id))
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
