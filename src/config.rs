use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};
use tokio::{
    fs::write,
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::{info, warn};
use twilight_model::id::{marker::ChannelMarker, Id};

use crate::GUILDSETTINGS_FILE_PATH;

/// A struct holding specific guild settings
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LuroGuildSettings {
    /// Guild Accent Colour
    pub accent_colour: u32,
    /// User specified accent colour
    pub accent_colour_custom: Option<u32>,
    /// Discord events are logged here, if defined
    pub discord_events_log_channel: Option<Id<ChannelMarker>>,
    /// Moderator actions are pushed here such as bans, if defined
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>,
}

/// Structure for `guild_settings.toml`
/// This file is checked for some commands, and allows some overrides such as a channel to report bans or who can execute commands
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LuroGuilds {
    /// A hashmap containing all the guilds, and their settings. Key is GuildId
    pub guilds: HashMap<String, LuroGuildSettings>,
}

impl LuroGuilds {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub async fn get() -> Result<Self, String> {
        let mut file;
        let mut contents = String::new();
        // Create a file if it does not exist
        if !Path::new(GUILDSETTINGS_FILE_PATH).exists() {
            warn!("guild_settings.toml does not exist, creating it...");
            contents = r"guilds = {}".to_string();

            file = match File::create(GUILDSETTINGS_FILE_PATH).await {
                Ok(ok) => ok,
                Err(why) => return Err(format!("Error creating guild_settings.toml - {why}")),
            };

            if let Err(why) = file.write_all(contents.as_bytes()).await {
                warn!("Error writing toml file - {why}");
            };
            info!("guild_settings.toml successfully created!");
        } else {
            file = match File::open(GUILDSETTINGS_FILE_PATH).await {
                Ok(file_opened) => file_opened,
                Err(why) => return Err(format!("Error opening toml file - {why}")),
            };

            match file.read_to_string(&mut contents).await {
                Ok(size) => info!("Read file {GUILDSETTINGS_FILE_PATH} of length {size}"),
                Err(why) => return Err(format!("Error reading toml file - {why}")),
            };
        };

        match toml::from_str::<LuroGuilds>(&contents) {
            Ok(guild_settings) => Ok(guild_settings),
            Err(why) => return Err(format!("Error serialising toml file - {why}")),
        }
    }

    /// Write the struct to a toml file
    pub async fn write(&self) -> Result<(), String> {
        let struct_to_toml_string = match toml::to_string(&self.clone()) {
            Ok(string) => string,
            Err(err) => return Err(format!("Error serialising struct to toml string: {err}")),
        };

        match write(GUILDSETTINGS_FILE_PATH, struct_to_toml_string).await {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Error writing toml file: {err}")),
        }
    }
}
