use std::{convert::TryInto, net::SocketAddr, str::FromStr, sync::Arc};

use anyhow::Error;

use tracing::metadata::LevelFilter;
use tracing_subscriber::{reload::Handle, Registry};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{stream, ConfigBuilder, Intents, Shard};
use twilight_http::client::InteractionClient;
use twilight_lavalink::Lavalink;
use twilight_model::{
    gateway::{
        payload::outgoing::update_presence::UpdatePresencePayload,
        presence::{ActivityType, MinimalActivity, Status}
    },
    id::Id
};

use crate::{
    models::{GlobalData, GuildSettings, Hecks, Settings},
    LuroFramework
};

use crate::HECK_FILE_PATH;

impl LuroFramework {
    /// Creates a new framework builder, this is a shortcut to FrameworkBuilder.
    /// [new](crate::builder::FrameworkBuilder::new)
    pub async fn builder(
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
        let guild_data = GuildSettings::get().await?.guilds.into();
        let hecks = Hecks::get(HECK_FILE_PATH).await?;
        let owners = vec![Id::new(97003404601094144)];
        let global_data = GlobalData {
            count: 0,
            hecks,
            owners,
            application: application.clone(),
            current_user
        }
        .into();
        let settings = Settings {
            application_id: application.id
        }
        .into();

        Ok((
            Self {
                twilight_client,
                twilight_cache,
                hyper_client,
                lavalink,
                guild_data,
                global_data,
                tracing_subscriber,
                settings
            }
            .into(),
            shards
        ))
    }

    /// Gets the [interaction client](InteractionClient) using this framework's
    /// [http client](Client) and [application id](ApplicationMarker)
    pub fn interaction_client(&self) -> InteractionClient {
        self.twilight_client.interaction(self.settings.read().application_id)
    }
}
