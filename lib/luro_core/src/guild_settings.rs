use std::collections::HashMap;

use poise::serenity_prelude::{ChannelId, GuildId, RoleId};
use serde::{Deserialize, Serialize};
use tokio::{fs::write, fs::File, io::AsyncReadExt};
use tracing::log::info;

use crate::GUILDSETTINGS_FILE_PATH;

/// A struct holding specific guild settings
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LuroGuildSettings {
    /// Discord events are logged here, if defined
    pub discord_events_log_channel: Option<ChannelId>,
    /// Moderator actions are pushed here such as bans, if defined
    pub moderator_actions_log_channel: Option<ChannelId>,
    /// An override allowing users of these role to use 'Moderator' commands like Ban and Kick, without needing the perms themselves
    pub moderator_role_override: Option<Vec<RoleId>>
}

/// Structure for `guild_settings.toml`
/// This file is checked for some commands, and allows some overrides such as a channel to report bans or who can execute commands
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LuroGuilds {
    /// A hashmap containing all the guilds, and their settings. Key is GuildId
    pub guilds: HashMap<String, LuroGuildSettings>
}

impl LuroGuilds {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub async fn get(path: &str) -> LuroGuilds {
        let mut file_opened = match File::open(path).await {
            Ok(file_opened) => file_opened,
            Err(err) => panic!("Error opening toml file: {err}")
        };

        let mut contents = String::new();
        match file_opened.read_to_string(&mut contents).await {
            Ok(size) => {
                info!("Read file {path} of length {size}");
            }
            Err(err) => panic!("Error reading toml file: {err}")
        }

        return match toml::from_str::<LuroGuilds>(&contents) {
            Ok(ok) => ok,
            Err(err) => panic!("Error serialising toml file: {err}")
        };
    }

    /// Write the struct to a toml file
    pub async fn write(new_data: &LuroGuilds, path: &str) {
        let struct_to_toml_string = match toml::to_string(&new_data) {
            Ok(string) => string,
            Err(err) => panic!("Error serialising struct to toml string: {err}")
        };

        match write(path, struct_to_toml_string).await {
            Ok(a) => a, // TODO: No clue what this is doing but it works soooo....
            Err(err) => panic!("Error writing toml file: {err}")
        }
    }

    /// Reload just one guild, and return it's settings. Errors if it cannot find the guild.
    pub async fn reload_guild(&mut self, guild_id: GuildId, new_data: LuroGuildSettings) -> LuroGuildSettings {
        self.guilds.insert(guild_id.to_string(), new_data.clone());
        LuroGuilds::write(self, GUILDSETTINGS_FILE_PATH).await;
        new_data
    }

    /// Reload ALL guild settings
    pub fn reload_all(&mut self, new_data: &LuroGuilds) -> &mut Self {
        self.guilds = new_data.guilds.clone();
        self
    }
}
