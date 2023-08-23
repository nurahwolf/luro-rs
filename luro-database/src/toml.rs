use std::path::{Path, PathBuf};

const GDPR_DELETE: &str = "gdpr_delete = \"THE USER REQUESTED ALL OF THEIR DATA TO BE DELETED\"";
const GUILDSETTINGS_FILE_PATH: &str = "data/guilds";
const INTERACTION_FILE_PATH: &str = "data/interactions.toml";
const NSFW_HECK_FILE_PATH: &str = "data/nsfw_hecks.toml";
const NSFW_STORIES_FILE_PATH: &str = "data/nsfw_stories.toml";
const QUOTES_FILE_PATH: &str = "data/quotes.toml";
const SFW_HECK_FILE_PATH: &str = "data/sfw_hecks.toml";
const SFW_STORIES_FILE_PATH: &str = "data/sfw_stories.toml";
const USERDATA_FILE_PATH: &str = "data/user";

use serde::Deserialize;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{fs, io::AsyncReadExt};
use tracing::{debug, error, warn};

mod driver;

/// Defaults to the toml driver
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct TomlDatabaseDriver {}

impl TomlDatabaseDriver {
    // A simple function used to make sure our data path and other needed files exist
    pub async fn start() -> anyhow::Result<()> {
        let path_to_data = PathBuf::from("./data"); //env::current_dir().expect("Invaild executing directory").join("/data");

        // Initialise /data folder for toml. Otherwise it panics.
        if !path_to_data.exists() {
            tracing::warn!("/data folder does not exist, creating it...");
            fs::create_dir(path_to_data).await?;
            tracing::info!("/data folder successfully created!");
        }

        Ok(())
    }

    /// Gets the specified [Path], which should be a toml file. If it does not exist, it will be created.
    ///
    /// If the type does not exists, uses the passed `new` to create a new one.
    async fn get<'de, T>(path: &Path, new: T) -> anyhow::Result<T>
    where
        T: Serialize + DeserializeOwned + std::fmt::Debug
    {
        // Check to make sure our path exists, if not then create a new heck file
        if !path.exists() {
            warn!("Path {} does not exist, attempting to create it", path.to_string_lossy());
            let formatted_data = toml::to_string_pretty(&new)?;

            // Make sure the directory exists, then attempt to make the file
            fs::create_dir_all(path.parent().unwrap()).await?;
            fs::write(path, formatted_data).await?;
            // We can short curcuit here and return directly, saving a few operations.
            return Ok(new);
        }

        // Attempt to open the file. It should now exist due to the previous check
        let mut file_opened = fs::File::open(path).await?;
        let mut contents = String::new();

        // If we could read it, then let standard output know
        if let Ok(size) = file_opened.read_to_string(&mut contents).await {
            debug!("Read file {} of length {size}", path.to_string_lossy());
        }

        // Serialise into a Heck type
        match toml::from_str::<T>(&contents) {
            Ok(ok) => Ok(ok),
            Err(why) => {
                error!(why = ?why, "Failed to serialised the type {:?}", new);
                Err(why.into())
            }
        }
    }

    async fn gdpr_delete(path: &Path) -> anyhow::Result<()> {
        let new_data = toml::to_string_pretty(GDPR_DELETE)?;

        match path.exists() {
            true => {
                warn!(
                    "Path {} has been deleted at the request of the user (GDPR)",
                    path.to_string_lossy()
                );
                Ok(fs::write(path, new_data).await?)
            }
            false => Ok(warn!(
                "Path {} does not exist and the user requested that their data be deleted",
                path.to_string_lossy()
            ))
        }
    }

    /// Write the passed data to file
    async fn write<T>(data: T, path: &Path) -> anyhow::Result<()>
    where
        T: Serialize
    {
        let struct_to_toml_string = toml::to_string_pretty(&data)?;

        if !path.exists() {
            warn!("Path {} does not exist, attempting to create it", path.to_string_lossy());
            fs::create_dir_all(path.parent().unwrap()).await?
        }

        debug!("Path {} has bee updated with new data", path.to_string_lossy());
        Ok(fs::write(path, struct_to_toml_string).await?)
    }
}
