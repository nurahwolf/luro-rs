#![feature(let_chains)]

use anyhow::Error;
use commands::LuroCommands;
use config::{Hecks, LuroGuilds};
use dotenv::dotenv;
use events::{
    interaction_create::interaction_create_handler, message_create::message_create_handler,
    ready::ready_handler,
};
use futures::StreamExt;
use hyper::client::{Client as HyperClient, HttpConnector};

use tracing::{info, warn};
use twilight_cache_inmemory::InMemoryCache;

use core::fmt;
use std::{
    env,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, RwLock},
};
use twilight_gateway::{
    stream::{self, ShardEventStream},
    ConfigBuilder, Event, Intents, Shard,
};
use twilight_http::Client;
use twilight_lavalink::Lavalink;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    oauth::Application,
    user::CurrentUser,
};
use twilight_standby::Standby;

type State = Arc<Luro>;

pub mod commands;
pub mod config;
mod events;
pub mod functions;
pub mod interactions;

#[derive(Debug)]
pub struct Luro {
    /// The twilight client, used for interacting with the Discord PAI
    pub twilight_client: Client,
    /// Used for deferring tasks
    pub twilight_standby: Standby,
    /// Music!
    pub lavalink: Lavalink,
    /// Used for HTTP requests
    pub hyper_client: HyperClient<HttpConnector>,
    /// used to cache permissions and messages
    pub cache: InMemoryCache,
    /// Luro's data
    pub data: Data,
}

#[derive(Debug)]
pub struct Data {
    /// Information from Discord's API about the context of the application used to run the bot
    pub application_info: Application,
    /// The current bot user, initialised at bot login
    pub current_user: CurrentUser,
    /// Guild specific settings
    pub guild_settings: RwLock<LuroGuilds>,
    pub hecks: RwLock<Hecks>,
    pub interaction_count: RwLock<usize>,
    pub commands: RwLock<LuroCommands>,
}

// pub struct LuroData {
//     /// A structure holding all of Luro's register commands, both globally and per guild.
//     commands: RwLock<LuroCommands>,
//     /// Settings for Luro, primarily per guild.
//     guild_settings: RwLock<LuroGuilds>,
//     /// Hecks, a silly command with a bunch of silly
//     hecks: RwLock<Hecks>,
//     interaction_count: RwLock<usize>,
// }

impl Luro {
    pub async fn default() -> Result<(Self, Vec<Shard>), Error> {
        // Loads dotenv. This allows std::env to view the variables in the file
        dotenv().ok();

        let (token, lavalink_host, lavalink_auth, intents) = (
            env::var("DISCORD_TOKEN").expect("No DISCORD_TOKEN defined"),
            env::var("LAVALINK_HOST").expect("No LAVALINK_HOST defined"),
            env::var("LAVALINK_AUTHORISATION").expect("No LAVALINK_AUTHORISATION defined"),
            Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES | Intents::MESSAGE_CONTENT,
        );

        let (twilight_client, config) = (
            Client::new(token.clone()),
            ConfigBuilder::new(token.clone(), intents).build(),
        );

        let shard_builder =
            stream::create_recommended(&twilight_client, config, |_, c| c.build()).await;

        let (shards, current_user, application_info) = (
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

        let cache = InMemoryCache::new();
        let twilight_standby = Standby::new();
        let guild_settings = match LuroGuilds::get().await {
            Ok(ok) => ok,
            Err(_) => todo!(),
        }
        .into();
        let hyper_client = HyperClient::new();
        let hecks = match Hecks::get().await {
            Ok(ok) => ok,
            Err(_) => todo!(),
        }
        .into();
        let interaction_count = RwLock::new(0);
        let commands = LuroCommands::set_default_commands().into();
        let data = Data {
            application_info,
            current_user,
            guild_settings,
            hecks,
            interaction_count,
            commands,
        };

        Ok((
            Self {
                twilight_client,
                twilight_standby,
                lavalink,
                hyper_client,
                cache,
                data,
            },
            shards,
        ))
    }

    pub async fn accent_colour(&self, guild: Option<Id<GuildMarker>>) -> u32 {
        match guild {
            Some(guild) => {
                let all_guild_settings = match self.data.guild_settings.read() {
                    Ok(ok) => ok,
                    Err(_) => todo!(),
                };

                let guild_settings = match all_guild_settings.guilds.get(&guild.to_string()) {
                    Some(ok) => ok,
                    None => {
                        warn!("No guild settings defined for guild");
                        return ACCENT_COLOUR;
                    }
                };

                if let Some(custom_accent_colour_defined) = guild_settings.accent_colour_custom {
                    custom_accent_colour_defined
                } else {
                    guild_settings.accent_colour
                }
            }
            None => ACCENT_COLOUR,
        }
    }

    pub async fn register_global_commands(self) -> anyhow::Result<()> {
        let global_commands = match self.data.commands.read() {
            Ok(ok) => ok.clone().global_commands,
            Err(why) => panic!("Command Mutex is poisoned: {why}"),
        };

        self.twilight_client
            .interaction(self.data.application_info.id)
            .set_global_commands(&global_commands)
            .await?
            .model()
            .await?;

        Ok(())
    }
}

impl Data {
    // /// Return a default context, usually on bot startup.
    // pub async fn init() -> Result<Self, Error> {
    //     let path_to_data = PathBuf::from("./data"); //env::current_dir().expect("Invaild executing directory").join("/data");

    //     // Initialise /data folder for toml. Otherwise it panics.
    //     if !path_to_data.exists() {
    //         tracing::warn!("/data folder does not exist, creating it...");
    //         fs::create_dir(path_to_data).expect("Failed to make data subfolder");
    //         tracing::info!("/data folder successfully created!");
    //     }

    //     // Initialise our guild settings
    //     let guild_settings = match LuroGuilds::get().await {
    //         Ok(ok) => RwLock::new(ok),
    //         Err(why) => panic!("Failed to initialise guild settings - {why}"),
    //     };

    //     // Initialise our hecks settings
    //     let hecks = match Hecks::get().await {
    //         Ok(ok) => RwLock::new(ok),
    //         Err(why) => panic!("Failed to initialise guild settings - {why}"),
    //     };

    //     let commands = RwLock::new(LuroCommands::set_default_commands());
    //     let interaction_count = RwLock::new(0);

    //     Ok(
    //         Self{
    //             commands,
    //             guild_settings,
    //             hecks,
    //             interaction_count
    //     })
    // }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise the tracing subscriber.
    tracing_subscriber::fmt::init();

    // Initialise Luro
    let (luro, mut shards) = match Luro::default().await {
        Ok(luro) => luro,
        Err(why) => panic!("Failed to initialise Luro! {why}"),
    };

    // Create our state
    let state: State = Arc::new(luro);

    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((_, event)) = stream.next().await {
        let context = context.clone();
        tokio::spawn(bot::handle_event(event, context));
    }

    while let Some((shard, event)) = stream.next().await {
        let event = match event {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        Luro::event_handler(state.clone(), shard, event).await;
    }

    Ok(())
}

pub const ACCENT_COLOUR: u32 = 0xDABEEF;

#[derive(Debug)]
pub enum LuroError {
    NoInteractionData,
    NoApplicationCommand,
    NoMessageInteractionData,
}

impl std::error::Error for LuroError {}

impl fmt::Display for LuroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuroError::NoMessageInteractionData => write!(f, "No Message Interaction Data"),
            LuroError::NoInteractionData => write!(f, "No data was found in the interaction"),
            LuroError::NoApplicationCommand => write!(
                f,
                "No ApplicationCommand was found in the interaction's data"
            ),
        }
    }
}

/// Used for setting what environment variable Luro listens for. Defaults to "LURO_TOKEN".
pub const BOT_TOKEN: &str = "LURO_TOKEN";
/// The core data directory for Luro. By default this is at the "data" folder within Luro.
/// Consider setting this to XDG_DATA_HOME on a production system.
pub const DATA_PATH: &str = "data/";
/// Where the config toml file lives. Can be overriden elsewhere if desired.
pub const CONFIG_FILE_PATH: &str = "data/config.toml";
/// Where the database folder lives. Can be overriden elsewhere if desired.
pub const DATABASE_FILE_PATH: &str = "data/database";
/// Where the heck toml file lives. Can be overriden elsewhere if desired.
pub const HECK_FILE_PATH: &str = "data/hecks.toml";
/// Where the quotes toml file lives. Can be overriden elsewhere if desired.
pub const QUOTES_FILE_PATH: &str = "data/quotes.toml";
/// Where the user_favs toml file lives. Can be overriden elsewhere if desired.
pub const FAVOURITES_FILE_PATH: &str = "data/user_favs.toml";
/// Where the secrets toml file lives. Make sure this is in a safe space and with strong permissions!
pub const SECRETS_FILE_PATH: &str = "data/secrets.toml";
/// Where the stories toml file lives. Can be overriden elsewhere if desired.
pub const STORIES_FILE_PATH: &str = "data/stories.toml";
/// Where the guild_settings toml file lives. Can be overriden elsewhere if desired.
pub const GUILDSETTINGS_FILE_PATH: &str = "data/guild_settings.toml";
/// Where the fursona folder lives. Can be overriden elsewhere if desired.
pub const FURSONA_FILE_PATH: &str = "data/fursona";
/// The regex used to match furaffinity posts.
pub const FURAFFINITY_REGEX: &str = r"(?:https://)?(?:www\.)?furaffinity\.net/(?:view|full)/(?P<submission_id>\d+)/?|https://d\.(?:facdn|furaffinity).net/art/(?P<author>[\w\-.~:?#\[\]@!$&'()*+,;=%]+)/(?P<cdn_id>\d+)/(?P<original_cdn_id>\d*).\S*(?:gif|jpe?g|tiff?|png|webp|bmp)";
/// Regex to pull out links from a message, which is then passed to the source finder commands.
pub const SOURCE_FINDER_REGEX: &str = r"(?P<url>http[^\s>]+)";
/// The timeout duriation for command buttons, in seconds.
pub const TIMEOUT_DURIATION: u64 = 12 * 60;
