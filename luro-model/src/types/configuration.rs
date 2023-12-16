use std::sync::Arc;

use anyhow::Context;
use tracing::debug;
use twilight_gateway::{Config, ConfigBuilder, Intents};
use twilight_model::{
    gateway::{
        payload::outgoing::update_presence::{UpdatePresenceError, UpdatePresencePayload},
        presence::{ActivityType, MinimalActivity, Status},
    },
    user::CurrentUser,
};

#[cfg(feature = "toml-configuration")]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
/// A structure representing an on-disk configuration file.
/// All parameters are optional, and if required, fall back to environment variables.
pub struct ConfigurationFile {
    pub token: Option<String>
}

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
    pub logging: Arc<luro_logging::Logging>,
    pub current_user: CurrentUser,
}

impl Configuration {
    /// Create a new configuration, fetching most information from environment variables
    pub async fn new(intents: Intents) -> anyhow::Result<Self> {
        #[cfg(feature = "dotenvy")]
        dotenvy::dotenv()?;

        let token = std::env::var("DISCORD_TOKEN").context("Failed to get the variable DISCORD_TOKEN")?;
        let twilight_client = twilight_http::Client::new(token.clone());
        let current_user = twilight_client.current_user().await?.model().await?;
        let logging = luro_logging::init(&current_user.name);
        let shard_config = shard_config_builder(intents, token.clone())?;

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
            twilight_client: twilight_client.into(),
            shard_config,
            connection_string,
            logging: logging.into(),
            current_user,
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
