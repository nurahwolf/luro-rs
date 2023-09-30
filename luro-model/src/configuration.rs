use std::sync::Arc;

use twilight_gateway::{Config, ConfigBuilder, Intents};
use twilight_model::gateway::{
    payload::outgoing::update_presence::{UpdatePresenceError, UpdatePresencePayload},
    presence::{ActivityType, MinimalActivity, Status},
};

#[derive(Debug, Clone)]
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
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    pub twilight_client: Arc<twilight_http::Client>,
    pub shard_config: Config,
    pub connection_string: String,
}

impl Configuration {
    /// Create a new configuration
    pub fn new(
        intents: Intents,
        #[cfg(feature = "lavalink")] lavalink_auth: String,
        #[cfg(feature = "lavalink")] lavalink_host: String,
        token: String,
    ) -> anyhow::Result<Self> {
        #[cfg(feature = "cache-memory")]
        let cache = twilight_cache_inmemory::InMemoryCache::new().into();
        let twilight_client = twilight_http::Client::new(token.clone()).into();
        let shard_config = shard_config_builder(intents, token.clone())?;
        let connection_string = "postgres://localhost/luro".to_owned();

        Ok(Self {
            cache,
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
        })
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
