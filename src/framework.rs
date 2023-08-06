use std::{
    convert::TryInto,
    fs,
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc
};

use dashmap::DashMap;
use hyper::client::HttpConnector;
use parking_lot::RwLock;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{reload::Handle, Registry};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{stream, Config, ConfigBuilder, Intents, Shard};
use twilight_http::Client;
use twilight_lavalink::Lavalink;
use twilight_model::{
    gateway::{
        payload::outgoing::update_presence::UpdatePresencePayload,
        presence::{ActivityType, MinimalActivity, Status}
    },
    id::{
        marker::{GuildMarker, MessageMarker, UserMarker},
        Id
    }
};

use crate::{
    models::{GlobalData, GuildSetting, Hecks, LuroCommandCache, UserData},
    traits::toml::LuroTOML,
    BOT_OWNERS, HECK_FILE_PATH, STORIES_FILE_PATH
};

mod accent_colour;
mod default_embed;
mod deferred;
mod get_guild_id;
mod get_interaction_author;
mod get_specified_user_or_author;
mod handle_interaction;
mod interaction_channel;
mod interaction_client;
mod parse_modal_field;
mod register_commands;
mod response;
mod send_log_channel;
mod send_message;

/// A rewored data structure for Luro
#[derive(Debug)]

pub struct LuroFramework {
    /// HTTP client used for making outbound API requests
    pub hyper_client: hyper::Client<HttpConnector>,
    /// Twilight's client for interacting with the Discord API
    pub twilight_client: Client,
    /// Twilight's cache
    pub twilight_cache: InMemoryCache,
    /// Lavalink client, for playing music
    pub lavalink: Lavalink,
    /// The global tracing subscriber, for allowing manipulation of logged data
    pub tracing_subscriber: Handle<LevelFilter, Registry>,
    /// Data that may be accessed globally, including DMs
    pub data_global: RwLock<GlobalData>,
    /// Cached data for handling commands
    pub data_command: DashMap<Id<MessageMarker>, LuroCommandCache>,
    /// Data that is specific to a guild
    pub data_guild: DashMap<Id<GuildMarker>, GuildSetting>,
    /// Data that is specific to a user
    pub data_user: DashMap<Id<UserMarker>, UserData>
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

async fn data_global(twilight_client: &Client) -> anyhow::Result<GlobalData> {
    let application = twilight_client.current_user_application().await?.model().await?;
    let mut owners = vec![];
    if let Some(application_owner) = &application.owner {
        owners.push(application_owner.clone());
        for owner in BOT_OWNERS {
            if owner == application_owner.id {
                continue;
            }
            owners.push(twilight_client.user(owner).await?.model().await?)
        }
    } else {
        for owner in BOT_OWNERS {
            owners.push(twilight_client.user(owner).await?.model().await?)
        }
    }

    Ok(GlobalData {
        count: 0,
        hecks: Hecks::get(Path::new(HECK_FILE_PATH)).await?,
        owners,
        application,
        current_user: twilight_client.current_user().await?.model().await?,
        stories: GlobalData::get_stories(Path::new(STORIES_FILE_PATH)).await?.stories
    })
}

impl LuroFramework {
    pub async fn run(
        intents: Intents,
        lavalink_auth: String,
        lavalink_host: String,
        token: String,
        tracing_subscriber: Handle<LevelFilter, Registry>
    ) -> anyhow::Result<(Arc<Self>, Vec<Shard>)> {
        ensure_data_directory_exists();
        let (twilight_client, twilight_cache, shard_config) = create_twilight_client(intents, token)?;
        let data_global = data_global(&twilight_client).await?;
        let shards = stream::create_recommended(&twilight_client, shard_config, |_, c| c.build())
            .await?
            .collect::<Vec<_>>();

        let lavalink = {
            let socket = SocketAddr::from_str(&lavalink_host)?;
            let lavalink = Lavalink::new(data_global.current_user.id, shards.len().try_into()?);
            lavalink.add(socket, lavalink_auth).await?;
            lavalink
        };

        Ok((
            Self {
                hyper_client: hyper::Client::new(),
                twilight_cache,
                lavalink,
                tracing_subscriber,
                data_global: data_global.into(),
                twilight_client,
                data_command: Default::default(),
                data_guild: Default::default(),
                data_user: Default::default()
            }
            .into(),
            shards
        ))
    }
}
