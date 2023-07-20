use std::collections::hash_map::Entry;
use std::path::Path;

use anyhow::Error;

use tokio::fs::write;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt}
};
use tracing::{info, warn};
use twilight_model::id::{marker::GuildMarker, Id};

use crate::models::{GuildSetting, GuildSettings};
use crate::{LuroContext, GUILDSETTINGS_FILE_PATH};

impl GuildSettings {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub async fn get() -> anyhow::Result<Self> {
        let mut file;
        let mut contents = String::new();
        // Create a file if it does not exist
        if !Path::new(GUILDSETTINGS_FILE_PATH).exists() {
            warn!("guild_settings.toml does not exist, creating it...");
            contents = r"guilds = {}".to_string();

            file = match File::create(GUILDSETTINGS_FILE_PATH).await {
                Ok(ok) => ok,
                Err(why) => return Err(Error::msg(format!("Error creating guild_settings.toml - {why}")))
            };

            if let Err(why) = file.write_all(contents.as_bytes()).await {
                warn!("Error writing toml file - {why}");
            };
            info!("guild_settings.toml successfully created!");
        } else {
            file = match File::open(GUILDSETTINGS_FILE_PATH).await {
                Ok(file_opened) => file_opened,
                Err(why) => return Err(Error::msg(format!("Error opening toml file - {why}")))
            };

            match file.read_to_string(&mut contents).await {
                Ok(size) => info!("Read file {GUILDSETTINGS_FILE_PATH} of length {size}"),
                Err(why) => return Err(Error::msg(format!("Error reading toml file - {why}")))
            };
        };

        match toml::from_str::<Self>(&contents) {
            Ok(guild_settings) => Ok(guild_settings),
            Err(why) => Err(Error::msg(format!("Error serialising toml file - {why}")))
        }
    }

    /// Write the struct to a toml file
    pub async fn write(ctx: &LuroContext) -> anyhow::Result<()> {
        let guilds = Self {
            guilds: ctx.guild_data.read().clone()
        };

        let struct_to_toml_string = match toml::to_string(&guilds) {
            Ok(string) => string,
            Err(why) => return Err(Error::msg(format!("Error serialising struct to toml string: {why}")))
        };

        match write(GUILDSETTINGS_FILE_PATH, struct_to_toml_string).await {
            Ok(_) => Ok(()),
            Err(why) => Err(Error::msg(format!("Error writing toml file: {why}")))
        }
    }

    /// Create guild settings for a guild, if it is not present.
    pub fn check_guild_is_present(ctx: LuroContext, guild_id: Id<GuildMarker>) -> anyhow::Result<()> {
        let mut guild_db = ctx.guild_data.write();

        match guild_db.entry(guild_id) {
            Entry::Occupied(_) => (),
            Entry::Vacant(vacant) => {
                vacant.insert(GuildSetting {
                    commands: Default::default(),
                    hecks: Default::default(),
                    accent_colour: Default::default(),
                    accent_colour_custom: Default::default(),
                    discord_events_log_channel: Default::default(),
                    moderator_actions_log_channel: Default::default()
                });
            }
        };
        Ok(())
    }
}
