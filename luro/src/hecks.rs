use anyhow::Error;

use tokio::{fs::write, fs::File, io::AsyncReadExt};
use tracing::info;

use crate::{models::Hecks, LuroContext, HECK_FILE_PATH};

impl Hecks {
    /// Get a new structure filled with data from a toml file.
    pub async fn get(path: &str) -> Result<Hecks, Error> {
        let mut file_opened = match File::open(path).await {
            Ok(file_opened) => file_opened,
            Err(why) => return Err(why.into())
        };

        let mut contents = String::new();
        match file_opened.read_to_string(&mut contents).await {
            Ok(size) => {
                info!("Read file {path} of length {size}");
            }
            Err(why) => return Err(why.into())
        }

        match toml::from_str::<Hecks>(&contents) {
            Ok(secrets) => Ok(secrets),
            Err(why) => Err(why.into())
        }
    }

    /// Write the struct to a toml file
    pub async fn write(ctx: &LuroContext) -> Result<(), Error> {
        let struct_to_toml_string = match toml::to_string(&ctx.global_data.read().hecks) {
            Ok(string) => string,
            Err(why) => return Err(why.into())
        };

        match write(HECK_FILE_PATH, struct_to_toml_string).await {
            Ok(a) => Ok(a),
            Err(why) => Err(why.into())
        }
    }

    /// Reload sfw heck IDs
    pub fn reload_sfw_heck_ids(&mut self) {
        let mut heck_ids = vec![];

        for num in 0..self.sfw_hecks.len() {
            heck_ids.push(num)
        }

        self.sfw_heck_ids = heck_ids;
    }

    /// Reload nsfw heck IDs
    pub fn reload_nsfw_heck_ids(&mut self) {
        let mut heck_ids = vec![];

        for num in 0..self.nsfw_hecks.len() {
            heck_ids.push(num)
        }

        self.nsfw_heck_ids = heck_ids;
    }
}
