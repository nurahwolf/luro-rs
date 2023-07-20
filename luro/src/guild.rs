use std::collections::hash_map::Entry;
use std::{collections::HashMap, path::Path};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use tokio::fs::write;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt}
};
use tracing::{info, warn};
use twilight_model::{
    application::command::Command,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id
    }
};

use crate::framework::LuroFramework;
use crate::LuroContext;
use crate::{hecks::Hecks, GUILDSETTINGS_FILE_PATH};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LuroGuilds {
    /// Guild Settings
    pub guilds: HashMap<Id<GuildMarker>, LuroGuild>
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LuroGuild {
    /// Commands registered to a guild
    pub commands: Vec<Command>,
    /// Private hecks for this specific guild
    pub hecks: Hecks,
    /// Guild Accent Colour, which is the first colour role within a guild
    pub accent_colour: u32,
    /// An administrator may wish to override the colour in which case this is set.
    pub accent_colour_custom: Option<u32>,
    /// Discord events are logged here, if defined
    pub discord_events_log_channel: Option<Id<ChannelMarker>>,
    /// Moderator actions are pushed here such as bans, if defined
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>
}

impl LuroGuilds {
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
    pub async fn write(ctx: &LuroFramework) -> anyhow::Result<()> {
        let guilds = LuroGuilds {
            guilds: ctx.guilds.read().clone()
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
        let mut guild_db = ctx.guilds.write();

        match guild_db.entry(guild_id) {
            Entry::Occupied(_) => (),
            Entry::Vacant(vacant) => {
                vacant.insert(LuroGuild {
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
