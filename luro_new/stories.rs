use anyhow::Error;
use serde::{Deserialize, Serialize};
use tokio::{fs::write, fs::File, io::AsyncReadExt};
use tracing::info;

/// Structure for `stories.toml`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stories {
    pub stories: Vec<[String; 2]>,
}

impl Stories {
    /// Get a new structure filled with data from a toml file.
    pub async fn get(path: &str) -> Result<Stories, Error> {
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

        match toml::from_str::<Stories>(&contents) {
            Ok(secrets) => Ok(secrets),
            Err(why) => Err(why.into()),
        }
    }

    /// Write the struct to a toml file
    pub async fn write(new_data: &Stories, path: &str) -> Result<(), Error> {
        let struct_to_toml_string = match toml::to_string(&new_data) {
            Ok(string) => string,
            Err(why) => return Err(why.into()),
        };

        match write(path, struct_to_toml_string).await {
            Ok(a) => Ok(a),
            Err(why) => Err(why.into()),
        }
    }

    /// Mutate the struct
    pub fn reload(&mut self, new_data: &Stories) {
        self.stories = new_data.stories.clone();
    }
}
