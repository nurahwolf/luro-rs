use std::collections::HashMap;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use tokio::{fs::write, fs::File, io::AsyncReadExt};
use tracing::info;
use twilight_model::id::{
    marker::{ChannelMarker, RoleMarker},
    Id,
};

use crate::GUILDSETTINGS_FILE_PATH;

/// A struct holding specific guild settings
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LuroGuildSettings {
    /// Discord events are logged here, if defined
    pub discord_events_log_channel: Option<Id<ChannelMarker>>,
    /// Moderator actions are pushed here such as bans, if defined
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>,
    /// An override allowing users of these role to use 'Moderator' commands like Ban and Kick, without needing the perms themselves
    pub moderator_role_override: Option<Vec<Id<RoleMarker>>>,
}

/// Structure for `guild_settings.toml`
/// This file is checked for some commands, and allows some overrides such as a channel to report bans or who can execute commands
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LuroGuilds {
    /// A hashmap containing all the guilds, and their settings. Key is GuildId
    pub guilds: HashMap<String, LuroGuildSettings>,
}

impl LuroGuilds {
    /// Get a new structure filled with data from a toml file.
    pub async fn get(path: &str) -> Result<LuroGuildSettings, Error> {
        let mut file_opened = File::open(path).await?;

        let mut contents = String::new();
        match file_opened.read_to_string(&mut contents).await {
            Ok(size) => {
                info!("Read file {path} of length {size}");
            }
            Err(why) => return Err(why.into()),
        }

        match toml::from_str::<LuroGuildSettings>(&contents) {
            Ok(secrets) => Ok(secrets),
            Err(why) => Err(why.into()),
        }
    }

    /// Write the struct to a toml file
    pub async fn write(new_data: &LuroGuilds, path: &str) -> Result<(), Error> {
        let struct_to_toml_string = toml::to_string(&new_data)?;

        match write(path, struct_to_toml_string).await {
            Ok(a) => Ok(a),
            Err(why) => Err(why.into()),
        }
    }

    /// Reload just one guild, and return it's settings. Errors if it cannot find the guild.
    pub async fn reload_guild(
        &mut self,
        guild_id: Id<ChannelMarker>,
        new_data: LuroGuildSettings,
    ) -> Result<LuroGuildSettings, Error> {
        self.guilds.insert(guild_id.to_string(), new_data.clone());
        LuroGuilds::write(self, GUILDSETTINGS_FILE_PATH).await?;
        Ok(new_data)
    }

    /// Reload ALL guild settings
    pub fn reload_all(&mut self, new_data: &LuroGuilds) -> &mut Self {
        self.guilds = new_data.guilds.clone();
        self
    }
}
use std::path::Path;

use tokio::fs::write;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::{info, warn};

use crate::GUILDSETTINGS_FILE_PATH;

use super::LuroGuilds;


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
            Err(why) => Err(format!("Error serialising toml file - {why}")),
        }
    }

    /// Write the struct to a toml file
    pub async fn write(&self) -> Result<(), String> {
        let struct_to_toml_string = match toml::to_string(&self.clone()) {
            Ok(string) => string,
            Err(why) => return Err(format!("Error serialising struct to toml string: {why}")),
        };

        match write(GUILDSETTINGS_FILE_PATH, struct_to_toml_string).await {
            Ok(_) => Ok(()),
            Err(why) => Err(format!("Error writing toml file: {why}")),
        }
    }
}

