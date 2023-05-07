use std::{collections::HashMap, env, net::SocketAddr, str::FromStr, sync::Arc};

use anyhow::Error;
use hyper::client::HttpConnector;

use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{stream, ConfigBuilder, Intents, Shard};
use twilight_http::client::InteractionClient;
use twilight_lavalink::Lavalink;
use twilight_model::oauth::Application;
use twilight_standby::Standby;

use crate::{
    commands::commands,
    data::{hecks::Hecks, LuroData},
    HECK_FILE_PATH,
};

pub struct Luro {
    pub application: Application,
    pub twilight_client: twilight_http::Client,
    pub twilight_cache: InMemoryCache,
    pub twilight_standby: Standby,
    pub lavalink: Lavalink,
    pub hyper_client: hyper::Client<HttpConnector>,
    pub data: LuroData,
    pub redis_connection: redis::aio::Connection,
}

impl Luro {
    pub fn interaction(&self) -> InteractionClient<'_> {
        self.twilight_client.interaction(self.application.id)
    }

    // fn command_id(command_name: &str, commands: &[Command]) -> Option<Id<CommandMarker>> {
    //     commands
    //         .iter()
    //         .find_map(|command| (command.name == command_name).then_some(command.id?))
    // }

    pub async fn default() -> Result<(Arc<Self>, Vec<Shard>), Error> {
        let (token, lavalink_host, lavalink_auth, intents) = (
            env::var("DISCORD_TOKEN").expect("No DISCORD_TOKEN defined"),
            env::var("LAVALINK_HOST").expect("No LAVALINK_HOST defined"),
            env::var("LAVALINK_AUTHORISATION").expect("No LAVALINK_AUTHORISATION defined"),
            Intents::GUILD_MESSAGES
                | Intents::GUILD_VOICE_STATES
                | Intents::MESSAGE_CONTENT
                | Intents::GUILD_INVITES,
        );

        let (twilight_client, config) = (
            twilight_http::Client::new(token.clone()),
            ConfigBuilder::new(token.clone(), intents).build(),
        );

        let shard_builder =
            stream::create_recommended(&twilight_client, config, |_, c| c.build()).await;

        let (shards, current_user, application) = (
            shard_builder?.collect::<Vec<Shard>>(),
            twilight_client.current_user().await?.model().await?,
            twilight_client
                .current_user_application()
                .await?
                .model()
                .await?,
        );

        let lavalink = {
            let socket = SocketAddr::from_str(&lavalink_host)?;
            let lavalink = Lavalink::new(current_user.id, shards.len().try_into()?);
            lavalink.add(socket, lavalink_auth).await?;
            lavalink
        };

        let twilight_cache = InMemoryCache::new();
        let twilight_standby = Standby::new();
        let hyper_client = hyper::Client::new();
        let global_commands = commands();
        let data = LuroData {
            global_commands,
            guild_settings: HashMap::new().into(),
            boop: 0.into(),
            hecks: Hecks::get(HECK_FILE_PATH).await?.into(),
        };

        let redis_connection = Luro::get_db().await?.get_async_connection().await?;

        Ok((
            Self {
                application,
                twilight_client,
                twilight_cache,
                twilight_standby,
                lavalink,
                hyper_client,
                data,
                redis_connection,
            }
            .into(),
            shards,
        ))
    }
}
