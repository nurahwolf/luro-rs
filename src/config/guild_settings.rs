use std::path::Path;

use tokio::{fs::File, io::{AsyncWriteExt, AsyncReadExt}};
use tracing::{warn, info};
use tokio::fs::write;

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
            Err(why) => return Err(format!("Error serialising toml file - {why}")),
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