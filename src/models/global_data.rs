use std::path::Path;

use tokio::{fs, io::AsyncReadExt};
use tracing::{info, warn};

use super::{GlobalData, Stories, Story};

impl GlobalData {
    /// Get a new structure filled with data from a toml file. If it does not exist, create it.
    pub async fn get_stories(path: &Path) -> anyhow::Result<Stories> {
        // Check to make sure our path exists, if not then create a new heck file
        if !path.exists() {
            warn!("Path {} does not exist, attempting to create it", path.to_string_lossy());
            let new_data = Default::default();
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
        Ok(toml::from_str::<Stories>(&contents)?)
    }

    /// Reload global stories
    pub fn reload_stories(&mut self, new_data: Vec<Story>) -> Vec<Story> {
        self.stories = new_data;
        self.stories.clone()
    }
}
