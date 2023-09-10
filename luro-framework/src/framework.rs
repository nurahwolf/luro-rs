use std::sync::Arc;

use luro_model::{
    configuration::Configuration,
    database::{drivers::LuroDatabaseDriver, LuroDatabase},
};
use tracing_subscriber::{filter::LevelFilter, reload::Handle, Registry};
use twilight_gateway::{stream, Config, ConfigBuilder, Intents, Shard};
use twilight_http::Client;
use twilight_model::{
    gateway::{
        payload::outgoing::update_presence::UpdatePresencePayload,
        presence::{ActivityType, MinimalActivity, Status},
    },
    id::{marker::UserMarker, Id},
};

use crate::Framework;

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

impl<D: LuroDatabaseDriver,> Framework<D,> {
    pub async fn new(
        config: Configuration,
        database_driver: D,
        tracing_subscriber: Handle<LevelFilter, Registry,>,
    ) -> anyhow::Result<(Framework<D,>, Vec<Shard,>,),> {
        let (twilight_client, shard_config,) = create_twilight_client(config.intents, config.token,)?;
        let (database, current_user_id,) = initialise_database(database_driver, twilight_client.clone(),).await?;
        let shards = stream::create_recommended(&twilight_client, shard_config, |_, c| c.build(),)
            .await?
            .collect::<Vec<_,>>();

        #[cfg(feature = "lavalink")]
        let lavalink = {
            let socket = <std::net::SocketAddr as std::str::FromStr>::from_str(&config.lavalink_host,)?;
            let lavalink = twilight_lavalink::Lavalink::new(current_user_id, shards.len().try_into()?,);
            lavalink.add(socket, config.lavalink_auth,).await?;
            lavalink.into()
        };

        #[cfg(feature = "cache-memory")]
        let cache = twilight_cache_inmemory::InMemoryCache::new().into();

        #[cfg(feature = "http-client-hyper")]
        let http_client = hyper::Client::new();

        let framework = Self {
            #[cfg(feature = "cache-memory")]
            cache,
            #[cfg(feature = "http-client-hyper")]
            http_client,
            database,
            global_commands: Default::default(),
            guild_commands: Default::default(),
            #[cfg(feature = "lavalink")]
            lavalink,
            tracing_subscriber,
            twilight_client,
        };

        Ok((framework, shards,),)
    }
}

fn create_twilight_client(intents: Intents, token: String,) -> anyhow::Result<(Arc<Client,>, Config,),> {
    Ok((
        twilight_http::Client::new(token.clone(),).into(),
        ConfigBuilder::new(token, intents,)
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
            )?,)
            .build(),
    ),)
}

async fn initialise_database<D: LuroDatabaseDriver,>(
    driver: D,
    twilight_client: Arc<Client,>,
) -> anyhow::Result<(Arc<LuroDatabase<D,>,>, Id<UserMarker,>,),> {
    let application = twilight_client.current_user_application().await?.model().await?;
    let current_user = twilight_client.current_user().await?.model().await?;
    let current_user_id = current_user.id;
    Ok((
        LuroDatabase::build(application, current_user, twilight_client, driver,).into(),
        current_user_id,
    ),)
}
