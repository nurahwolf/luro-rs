use anyhow::Error;
use serde::{Deserialize, Serialize};
use tokio::{fs::write, fs::File, io::AsyncReadExt};
use tracing::info;

/// Structure for `quotes.toml`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Quotes {
    pub quotes: Vec<Quote>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Quote {
    pub user_id: u64,
    pub quote: String,
}

impl Quotes {
    /// Get a new structure filled with data from a toml file.
    pub async fn get(path: &str) -> Result<Quotes, Error> {
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

        match toml::from_str::<Quotes>(&contents) {
            Ok(secrets) => Ok(secrets),
            Err(why) => Err(why.into()),
        }
    }

    /// Write the struct to a toml file
    pub async fn write(new_data: &Quotes, path: &str) -> Result<(), Error> {
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
    pub fn reload(&mut self, new_data: &Quotes) {
        self.quotes = new_data.quotes.clone();
    }
}
