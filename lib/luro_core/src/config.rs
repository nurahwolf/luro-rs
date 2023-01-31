use serde::{Deserialize, Serialize};
use tokio::{fs::write, fs::File, io::AsyncReadExt};
use tracing::log::info;

/// Structure for `config.toml`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub accent_colour: [u8; 3],
    pub e621_blacklist: String,
    pub e621_useragent: String,
    pub webhook_name: String,
    pub git_url: Option<String>
}

impl Config {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub async fn get(path: &str) -> Config {
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

        return match toml::from_str::<Config>(&contents) {
            Ok(secrets) => secrets,
            Err(err) => panic!("Error serialising toml file: {err}")
        };
    }

    /// Write the struct to a toml file
    pub async fn write(new_data: &Config, path: &str) {
        let struct_to_toml_string = match toml::to_string(&new_data) {
            Ok(string) => string,
            Err(err) => panic!("Error serialising struct to toml string: {err}")
        };

        match write(path, struct_to_toml_string).await {
            Ok(a) => a, // TODO: No clue what this is doing but it works soooo....
            Err(err) => panic!("Error writing toml file: {err}")
        }
    }

    /// Mutate the struct
    pub fn reload(&mut self, new_data: &Config) {
        self.accent_colour = new_data.accent_colour;
        self.e621_blacklist = new_data.e621_blacklist.clone();
        self.e621_useragent = new_data.e621_useragent.clone();
    }
}
