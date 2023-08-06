use std::collections::HashMap;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use tokio::{fs::write, fs::File, io::AsyncReadExt};
use tracing::info;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Favorite {
    pub channel_id: u64,
    pub message_id: u64,
}

/// Structure for `user_favs.toml`
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Favs {
    /// A hashset of user IDs, containing a hashset of 'categories', which contains a vec of messages
    pub favs: HashMap<String, HashMap<String, Vec<Favorite>>>,
}

impl Favs {
    /// Remove the favorite!
    pub async fn remove_favourite(
        &mut self,
        author_id: &String,
        category: &String,
        id: usize,
    ) -> Result<(), String> {
        let user_favs = match self.favs.get_mut(author_id) {
            Some(ok) => ok,
            None => return Err("Failed to find user's favourites!".into()),
        };

        let category_favs = match user_favs.get_mut(category) {
            Some(ok) => ok,
            None => return Err(format!("Failed to find category {category}")),
        };

        category_favs.remove(id);

        Ok(())
    }

    /// Get a new structure filled with data from a toml file.
    pub async fn get(path: &str) -> Result<Favs, Error> {
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

        match toml::from_str::<Favs>(&contents) {
            Ok(secrets) => Ok(secrets),
            Err(why) => Err(why.into()),
        }
    }

    /// Write the struct to a toml file
    pub async fn write(new_data: &Favs, path: &str) -> Result<(), Error> {
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
    pub fn reload(&mut self, new_data: &Favs) {
        self.favs = new_data.favs.clone();
    }
}
