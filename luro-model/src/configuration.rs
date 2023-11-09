use std::sync::Arc;

use anyhow::Context;
use tracing::debug;
use tracing_subscriber::{
    filter::LevelFilter,
    reload::{Handle, Layer},
    Registry,
};
use twilight_gateway::{Config, ConfigBuilder, Intents};
use twilight_model::gateway::{
    payload::outgoing::update_presence::{UpdatePresenceError, UpdatePresencePayload},
    presence::{ActivityType, MinimalActivity, Status},
};

#[derive(Debug)]
pub struct Configuration {
    /// The token used for interacting with the Discord API
    pub token: String,
    /// The intents we want to listen for
    pub intents: Intents,
    /// The host for where Lavalink is running
    #[cfg(feature = "lavalink")]
    pub lavalink_host: String,
    /// The auth header for being able to interact with lavalink
    #[cfg(feature = "lavalink")]
    pub lavalink_auth: String,
    #[cfg(feature = "cache-memory")]
    pub twilight_client: Arc<twilight_http::Client>,
    pub shard_config: Config,
    pub connection_string: String,
    pub filter: Layer<tracing_subscriber::filter::LevelFilter, Registry>,
    pub tracing_subscriber: Handle<LevelFilter, Registry>,
}

impl Configuration {
    /// Create a new configuration, fetching most information from environment variables
    pub fn new(intents: Intents, filter: LevelFilter) -> anyhow::Result<Self> {
        #[cfg(feature = "dotenvy")]
        dotenvy::dotenv()?;

        let token = std::env::var("DISCORD_TOKEN").context("Failed to get the variable DISCORD_TOKEN")?;
        let twilight_client = twilight_http::Client::new(token.clone()).into();
        let shard_config = shard_config_builder(intents, token.clone())?;
        let (filter, tracing_subscriber) = tracing_subscriber::reload::Layer::new(filter);

        #[cfg(feature = "cache-memory")]
        let connection_string = std::env::var("DATABASE_URL").unwrap_or("".to_owned());

        #[cfg(feature = "lavalink")]
        let lavalink_auth = std::env::var("LAVALINK_AUTHORISATION").context("Failed to get the variable LAVALINK_AUTHORISATION")?;
        #[cfg(feature = "lavalink")]
        let lavalink_host = std::env::var("LAVALINK_HOST").context("Failed to get the variable LAVALINK_HOST")?;

        let config = Self {
            token,
            intents,
            #[cfg(feature = "lavalink")]
            lavalink_host,
            #[cfg(feature = "lavalink")]
            lavalink_auth,
            #[cfg(feature = "cache-memory")]
            twilight_client,
            shard_config,
            connection_string,
            filter,
            tracing_subscriber,
        };

        debug!("New config created: {:#?}", config);
        Ok(config)
    }
}

fn shard_config_builder(intents: Intents, token: String) -> Result<Config, UpdatePresenceError> {
    Ok(ConfigBuilder::new(token, intents)
        .presence(UpdatePresencePayload::new(
            vec![MinimalActivity {
                kind: ActivityType::Playing,
                name: "/about | Hello World!".to_owned(),
                url: None,
            }
            .into()],
            false,
            None,
            Status::Online,
        )?)
        .build())
}
