use std::{fs, net::SocketAddr, path::PathBuf, str::FromStr, sync::Arc};

use hyper::client::HttpConnector;
use tracing_subscriber::{filter::LevelFilter, reload::Handle, Registry};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{stream, Config, ConfigBuilder, Intents, Shard};
use twilight_http::Client;
use twilight_lavalink::Lavalink;
use twilight_model::{
    gateway::{
        payload::outgoing::update_presence::UpdatePresencePayload,
        presence::{ActivityType, MinimalActivity, Status}
    },
    id::{marker::UserMarker, Id}
};

use crate::LuroDatabaseDriver;
use luro_model::luro_database::LuroDatabase;

/// The core of Luro. Used to handle our global state and generally wrapped in an [Arc].
#[derive(Debug)]
pub struct Framework<D: LuroDatabaseDriver> {
    pub database: LuroDatabase<D>,
    /// HTTP client used for making outbound API requests
    pub hyper_client: hyper::Client<HttpConnector>,
    /// Lavalink client, for playing music
    pub lavalink: Lavalink,
    /// Twilight's client for interacting with the Discord API
    pub twilight_client: Client,
    /// Twilight's cache
    pub twilight_cache: InMemoryCache,
    /// The global tracing subscriber, for allowing manipulation within commands
    pub tracing_subscriber: Handle<LevelFilter, Registry>
}

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn builder(
        driver: D,
        intents: Intents,
        lavalink_auth: String,
        lavalink_host: String,
        token: String,
        tracing_subscriber: Handle<LevelFilter, Registry>
    ) -> anyhow::Result<(Arc<Self>, Vec<Shard>)> {
        ensure_data_directory_exists();
        let (twilight_client, twilight_cache, shard_config) = create_twilight_client(intents, token)?;
        let (database, current_user_id) = initialise_database(driver, &twilight_client).await?;
        let shards = stream::create_recommended(&twilight_client, shard_config, |_, c| c.build())
            .await?
            .collect::<Vec<_>>();

        let lavalink = {
            let socket = SocketAddr::from_str(&lavalink_host)?;
            let lavalink = Lavalink::new(current_user_id, shards.len().try_into()?);
            lavalink.add(socket, lavalink_auth).await?;
            lavalink
        };

        Ok((
            Self {
                database,
                hyper_client: hyper::Client::new(),
                lavalink,
                twilight_client,
                twilight_cache,
                tracing_subscriber
            }
            .into(),
            shards
        ))
    }
}

fn ensure_data_directory_exists() {
    let path_to_data = PathBuf::from("./data"); //env::current_dir().expect("Invaild executing directory").join("/data");

    // Initialise /data folder for toml. Otherwise it panics.
    if !path_to_data.exists() {
        tracing::warn!("/data folder does not exist, creating it...");
        fs::create_dir(path_to_data).expect("Failed to make data subfolder");
        tracing::info!("/data folder successfully created!");
    }
}

fn create_twilight_client(intents: Intents, token: String) -> anyhow::Result<(Client, InMemoryCache, Config)> {
    Ok((
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
    ))
}

async fn initialise_database<D: LuroDatabaseDriver>(
    driver: D,
    twilight_client: &Client
) -> anyhow::Result<(LuroDatabase<D>, Id<UserMarker>)> {
    let application = twilight_client.current_user_application().await?.model().await?;
    let current_user = twilight_client.current_user().await?.model().await?;
    let current_user_id = current_user.id;
    Ok((LuroDatabase::build(application, current_user, driver).await, current_user_id))
}
