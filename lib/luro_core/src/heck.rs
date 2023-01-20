use serde::{Deserialize, Serialize};
use tokio::{fs::write, fs::File, io::AsyncReadExt};
use tracing::log::info;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Heck {
    pub heck_message: String,
    pub author_id: u64
}

/// Structure for `heck.toml`
/// We have two hecks, one that is slowly drained (so we only get a heck once) and another used to get explicit hecks.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hecks {
    /// A vector containing all our available hecks
    pub hecks: Vec<Heck>,
    /// A vector of [usize] that contains availalbe random hecks to get. The hecks are reloaded when this reaches zero.
    pub available_heck_ids: Vec<usize>
}

impl Hecks {
    /// Get a new structure filled with data from a toml file. Note, this panics if it cannot find the toml file!
    pub async fn get(path: &str) -> Hecks {
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

        return match toml_edit::easy::from_str::<Hecks>(&contents) {
            Ok(secrets) => secrets,
            Err(err) => panic!("Error serialising toml file: {err}")
        };
    }

    /// Write the struct to a toml file
    pub async fn write(new_data: &Hecks, path: &str) {
        let struct_to_toml_string = match toml_edit::easy::to_string(&new_data) {
            Ok(string) => string,
            Err(err) => panic!("Error serialising struct to toml string: {err}")
        };

        match write(path, struct_to_toml_string).await {
            Ok(a) => a, // TODO: No clue what this is doing but it works soooo....
            Err(err) => panic!("Error writing toml file: {err}")
        }
    }

    /// Reload ALL hecks
    pub fn reload(&mut self, new_data: &Hecks) -> &mut Self {
        self.hecks = new_data.hecks.clone();
        // Calculate the heck_ids
        let mut heck_ids = vec![];
        for num in 0..self.hecks.len() {
            heck_ids.push(num)
        }
        self.available_heck_ids = heck_ids;
        self
    }

    /// Reload just the heck IDs
    pub fn reload_ids(&mut self) {
        let mut heck_ids = vec![];
        for num in 0..self.hecks.len() {
            heck_ids.push(num)
        }
        self.available_heck_ids = heck_ids;
    }
}
