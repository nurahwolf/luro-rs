use std::{
    convert::TryInto,
    fs,
    net::SocketAddr,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc
};

use anyhow::Error;

use tracing::{info, metadata::LevelFilter};
use tracing_subscriber::{reload::Handle, Registry};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{stream, ConfigBuilder, Intents, Shard};
use twilight_http::client::InteractionClient;
use twilight_lavalink::Lavalink;
use twilight_model::{
    application::command::Command,
    gateway::{
        payload::outgoing::update_presence::UpdatePresencePayload,
        presence::{ActivityType, MinimalActivity, Status}
    },
    id::{marker::ApplicationMarker, Id}
};

use crate::{
    models::{GlobalData, Hecks, Settings},
    LuroFramework, BOT_OWNERS, STORIES_FILE_PATH
};

use crate::HECK_FILE_PATH;

use crate::traits::toml::LuroTOML;

use super::Commands;

impl LuroFramework {
    /// Creates a new framework builder, this is a shortcut to FrameworkBuilder.
    pub async fn builder(
        intents: Intents,
        lavalink_auth: String,
        lavalink_host: String,
        token: String,
        tracing_subscriber: Handle<LevelFilter, Registry>
    ) -> Result<(Arc<Self>, Vec<Shard>), Error> {
        check_data();
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
        let hecks = Hecks::get(Path::new(HECK_FILE_PATH)).await?;
        let application_owner = match &application.owner {
            Some(owner) => owner.clone(),
            None => panic!("No bot owner present in application")
        };
        let mut owners = vec![application_owner.clone()];
        for owner in BOT_OWNERS {
            // If we already have the owner in the list (which is likely) then don't add them again.
            if owner == application_owner.id {
                continue;
            }

            owners.push(twilight_client.user(owner).await?.model().await?)
        }
        let settings = Settings {
            application_id: application.id
        }
        .into();
        let global_data = GlobalData {
            count: 0,
            hecks,
            owners,
            application,
            current_user,
            stories: GlobalData::get_stories(Path::new(STORIES_FILE_PATH)).await?.stories
        }
        .into();

        Ok((
            Self {
                twilight_client,
                twilight_cache,
                hyper_client,
                lavalink,
                user_data: Default::default(),
                guild_data: Default::default(),
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
    /// Register commands to the Discord API.
    pub async fn register_commands(&self, application_id: Id<ApplicationMarker>) -> anyhow::Result<()> {
        let client = self.twilight_client.interaction(application_id);

        match client
            .set_global_commands(
                &Commands::default_commands()
                    .global_commands
                    .into_values()
                    .collect::<Vec<Command>>()
            )
            .await
        {
            Ok(command_result) => Ok(info!(
                "Successfully registered {} global commands!",
                command_result.model().await?.len()
            )),
            Err(why) => Err(why.into())
        }
    }
}

// A simple function used to make sure our data path and other needed files exist
fn check_data() {
    let path_to_data = PathBuf::from("./data"); //env::current_dir().expect("Invaild executing directory").join("/data");

    // Initialise /data folder for toml. Otherwise it panics.
    if !path_to_data.exists() {
        tracing::warn!("/data folder does not exist, creating it...");
        fs::create_dir(path_to_data).expect("Failed to make data subfolder");
        tracing::info!("/data folder successfully created!");
    }
}
