use std::path::Path;

use tokio::{
    fs::write,
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::{info, warn};

use crate::HECK_FILE_PATH;

use super::{Heck, Hecks};

impl Hecks {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub async fn get() -> Result<Self, String> {
        let mut file;
        let mut contents = String::new();
        // Create a file if it does not exist
        if !Path::new(HECK_FILE_PATH).exists() {
            warn!("guild_settings.toml does not exist, creating it...");
            contents = "hecks = {}\nsfw_hecks = []\nnsfw_hecks = []\nsfw_heck_ids = []\nnsfw_heck_ids = []".to_string();

            file = match File::create(HECK_FILE_PATH).await {
                Ok(ok) => ok,
                Err(why) => return Err(format!("Error creating guild_settings.toml - {why}")),
            };

            if let Err(why) = file.write_all(contents.as_bytes()).await {
                warn!("Error writing toml file - {why}");
            };
            info!("guild_settings.toml successfully created!");
        } else {
            file = match File::open(HECK_FILE_PATH).await {
                Ok(file_opened) => file_opened,
                Err(why) => return Err(format!("Error opening toml file - {why}")),
            };

            match file.read_to_string(&mut contents).await {
                Ok(size) => info!("Read file {HECK_FILE_PATH} of length {size}"),
                Err(why) => return Err(format!("Error reading toml file - {why}")),
            };
        };

        match toml::from_str::<Hecks>(&contents) {
            Ok(hecks) => Ok(hecks),
            Err(why) => return Err(format!("Error serialising toml file - {why}")),
        }
    }

    /// Write the struct to a toml file
    pub async fn write(&self) -> Result<(), String> {
        let struct_to_toml_string = match toml::to_string(&self.clone()) {
            Ok(string) => string,
            Err(why) => return Err(format!("Error serialising struct to toml string: {why}")),
        };

        match write(HECK_FILE_PATH, struct_to_toml_string).await {
            Ok(_) => Ok(()),
            Err(why) => Err(format!("Error writing toml file: {why}")),
        }
    }

    /// Reload ALL hecks
    pub fn reload(&mut self, new_hecks: &Hecks) -> &mut Self {
        self.sfw_hecks = new_hecks.sfw_hecks.clone();
        self.nsfw_hecks = new_hecks.nsfw_hecks.clone();
        self.sfw_heck_ids = calculate_heck_ids(self.sfw_hecks.clone());
        self.nsfw_heck_ids = calculate_heck_ids(self.nsfw_hecks.clone());
        self
    }

    /// Reload all heck IDs only
    pub fn reload_all_heck_ids(&mut self) {
        self.sfw_heck_ids = calculate_heck_ids(self.sfw_hecks.clone());
        self.nsfw_heck_ids = calculate_heck_ids(self.nsfw_hecks.clone());
    }

    /// Reload sfw heck IDs
    pub fn reload_sfw_heck_ids(&mut self) {
        self.sfw_heck_ids = calculate_heck_ids(self.sfw_hecks.clone());
    }

    /// Reload nsfw heck IDs
    pub fn reload_nsfw_heck_ids(&mut self) {
        self.nsfw_heck_ids = calculate_heck_ids(self.nsfw_hecks.clone());
    }
}

/// Returns a vector filled with the available heck IDs
fn calculate_heck_ids(hecks: Vec<Heck>) -> Vec<usize> {
    let mut heck_ids = vec![];
    for num in 0..hecks.len() {
        heck_ids.push(num)
    }
    heck_ids
}
