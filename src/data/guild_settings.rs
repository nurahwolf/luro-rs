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
