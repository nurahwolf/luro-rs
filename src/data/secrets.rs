use anyhow::Error;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};
use tracing::info;

/// Structure for `secrets.toml`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Secrets {
    pub discord_token: Option<String>,
    pub e621_token: Option<String>,
    pub twitter_api: Option<String>,
    pub furaffinity_cookies: [String; 2],
    pub saucenao_token: String,
    pub owners: Option<Vec<u64>>,
}

impl Secrets {
    /// Get a new structure filled with data from a toml file.
    pub async fn get(path: &str) -> Result<Secrets, Error> {
        let mut file_opened = match File::open(path).await {
            Ok(file_opened) => file_opened,
            Err(why) => return Err(why.into()),
        };

        let mut contents = String::new();
        match file_opened.read_to_string(&mut contents).await {
            Ok(size) => {
                info!("Read file {path} of length {size}");
            }
            Err(why) => return Err(why.into()),
        }

        match toml::from_str::<Secrets>(&contents) {
            Ok(secrets) => Ok(secrets),
            Err(why) => Err(why.into()),
        }
    }

    // pub fn write(new_data: &Secrets, path: &str) {
    //     let struct_to_toml_string = match toml::to_string_pretty(&new_data) {
    //         Ok(string) => string,
    //         Err(err) => panic!("Error serialising struct to toml string: {err}")
    //     };

    //     match write(path, struct_to_toml_string).await {
    //         Ok(a) => a, // TODO: No clue what this is doing but it works soooo....
    //         Err(err) => panic!("Error writing toml file: {err}")
    //     }
    // }

    // pub fn reload(&mut self, new_data: &Secrets) {
    //     self.discord_token = new_data.discord_token.clone();
    //     self.e621_token = new_data.e621_token.clone();
    //     self.twitter_api = new_data.twitter_api.clone();
    // }
}
