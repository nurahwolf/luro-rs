use std::path::Path;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::AsyncReadExt;
use tracing::{debug, info, warn};

#[async_trait]
pub trait LuroTOML: Default + Serialize + for<'a> Deserialize<'a> {
    /// Get a new structure filled with data from a toml file. If it does not exist, create it.
    async fn get(path: &Path) -> anyhow::Result<Self> {
        // Check to make sure our path exists, if not then create a new heck file
        if !path.exists() {
            warn!("Path {} does not exist, attempting to create it", path.to_string_lossy());
            let new_data = Self::default();
            let formatted_data = toml::to_string_pretty(&new_data)?;

            // Make sure the directory exists, then attempt to make the file
            fs::create_dir_all(path.parent().unwrap()).await?;
            fs::write(path, formatted_data).await?;
            // We can short curcuit here and return directly, saving a few operations.
            return Ok(new_data);
        }

        // Attempt to open the file. It should now exist due to the previous check
        let mut file_opened = fs::File::open(path).await?;
        let mut contents = String::new();

        // If we could read it, then let standard output know
        if let Ok(size) = file_opened.read_to_string(&mut contents).await {
            info!("Read file {} of length {size}", path.to_string_lossy());
        }

        // Serialise into a Heck type
        Ok(toml::from_str::<Self>(&contents)?)
    }

    /// Write the struct to a toml file
    async fn write(&self, path: &Path) -> anyhow::Result<()> {
        let struct_to_toml_string = toml::to_string_pretty(self)?;

        if !path.exists() {
            warn!("Path {} does not exist, attempting to create it", path.to_string_lossy());
            fs::create_dir_all(path.parent().unwrap()).await?
        }

        debug!("Path {} has bee updated with new data", path.to_string_lossy());
        Ok(fs::write(path, struct_to_toml_string).await?)
    }
}
