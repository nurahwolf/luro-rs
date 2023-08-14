use std::path::{Path, PathBuf};

use anyhow::anyhow;
/// Where the heck toml file lives. Can be overriden elsewhere if desired.
const SFW_HECK_FILE_PATH: &str = "data/sfw_hecks.toml";
const NSFW_HECK_FILE_PATH: &str = "data/nsfw_hecks.toml";
/// Where the stories toml file lives. Can be overriden elsewhere if desired.
const SFW_STORIES_FILE_PATH: &str = "data/sfw_stories.toml";
const NSFW_STORIES_FILE_PATH: &str = "data/nsfw_stories.toml";
/// A folder where <guild/guild_id.toml> are stored
const GUILDSETTINGS_FILE_PATH: &str = "data/guilds";
/// A folder where <user/user_id.toml> are stored
const USERDATA_FILE_PATH: &str = "data/user";
const QUOTES_FILE_PATH: &str = "data/quotes.toml";

/// A toml file containing interactions, so that we can respond to them in the future post restart
const INTERACTION_FILE_PATH: &str = "data/interactions.toml";
use luro_model::constants::BOT_OWNERS;
use luro_model::heck::Heck;
use luro_model::luro_database_driver::LuroDatabaseDriver;
use luro_model::luro_message::LuroMessage;
use luro_model::story::Story;
use luro_model::types::{CommandManager, Hecks, LuroUserData, Stories, Quotes};
use luro_model::{guild_setting::GuildSetting, luro_user::LuroUser};
use serde::{de::DeserializeOwned, Serialize};
use tokio::{fs, io::AsyncReadExt};
use tracing::{debug, error, warn};
use twilight_model::application::interaction::Interaction;

use crate::TomlDatabaseDriver;

const GDPR_DELETE: &str = "gdpr_delete = \"THE USER REQUESTED ALL OF THEIR DATA TO BE DELETED\"";

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
    async fn get<'de, T>(path: &Path) -> anyhow::Result<T>
    where
        T: Default + Serialize + DeserializeOwned + std::fmt::Debug
    {
        // Check to make sure our path exists, if not then create a new heck file
        if !path.exists() {
            warn!("Path {} does not exist, attempting to create it", path.to_string_lossy());
            let new_data = T::default();
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
            debug!("Read file {} of length {size}", path.to_string_lossy());
        }

        // Serialise into a Heck type
        match toml::from_str::<T>(&contents) {
            Ok(ok) => Ok(ok),
            Err(why) => {
                error!(why = ?why, "Failed to serialised the type {:?}", T::default());
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

impl LuroDatabaseDriver for TomlDatabaseDriver {
    async fn get_user(&self, id: u64) -> anyhow::Result<LuroUser> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Self::get(Path::new(&path)).await
    }

    async fn save_user(&self, id: u64, user: &LuroUser) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Self::write(user, Path::new(&path)).await
    }

    async fn remove_user(&self, id: u64) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Self::gdpr_delete(Path::new(&path)).await
    }

    async fn modify_user(&self, id: u64, user: &LuroUser) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &id);
        Self::write(user, Path::new(&path)).await
    }

    async fn add_guild(&self, id: u64, user: &GuildSetting) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::write(user, Path::new(&path)).await
    }

    async fn get_guild(&self, id: u64) -> anyhow::Result<GuildSetting> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::get(Path::new(&path)).await
    }

    async fn save_guild(&self, id: u64, user: &GuildSetting) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::write(user, Path::new(&path)).await
    }

    async fn remove_guild(&self, id: u64) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::gdpr_delete(Path::new(&path)).await
    }

    /// Modify the guild settings and flush it to disk. This WILL overwrite all data locally!
    async fn update_guild(&self, id: u64, user: &GuildSetting) -> anyhow::Result<()> {
        let path = format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, &id);
        Self::write(user, Path::new(&path)).await
    }

    async fn add_sfw_heck(&self, heck: &Heck) -> anyhow::Result<()> {
        let data: Hecks = Self::get(Path::new(SFW_HECK_FILE_PATH)).await?;
        let total_hecks = data.len() + 1;
        data.entry(total_hecks.to_string()).insert(heck.clone());
        Self::write(data, Path::new(Path::new(SFW_HECK_FILE_PATH))).await
    }

    async fn get_sfw_hecks(&self) -> anyhow::Result<Hecks> {
        let hecks: Hecks = Self::get(Path::new(SFW_HECK_FILE_PATH)).await?;
        Ok(hecks)
    }

    async fn modify_sfw_heck(&self, id: usize, heck: &Heck) -> anyhow::Result<()> {
        let data: Hecks = Self::get(Path::new(SFW_HECK_FILE_PATH)).await?;
        data.entry(id.to_string()).insert(heck.clone());
        Self::write(data, Path::new(Path::new(SFW_HECK_FILE_PATH))).await
    }

    async fn modify_sfw_hecks(&self, modified_hecks: Vec<(usize, Heck)>) -> anyhow::Result<()> {
        let data: Hecks = Self::get(Path::new(SFW_HECK_FILE_PATH)).await?;
        for (heck_id, modified_heck) in modified_hecks {
            data.entry(heck_id.to_string()).insert(modified_heck);
        }
        Self::write(data, Path::new(Path::new(SFW_HECK_FILE_PATH))).await
    }

    async fn remove_sfw_heck(&self, id: usize) -> anyhow::Result<()> {
        let data: Hecks = Self::get(Path::new(SFW_HECK_FILE_PATH)).await?;
        data.remove(&id.to_string());
        Self::write(data, Path::new(Path::new(SFW_HECK_FILE_PATH))).await
    }

    async fn save_sfw_hecks(&self, hecks: &Hecks) -> anyhow::Result<()> {
        Self::write(hecks, Path::new(Path::new(SFW_HECK_FILE_PATH))).await
    }

    async fn add_nsfw_heck(&self, heck: &Heck) -> anyhow::Result<()> {
        let data: Hecks = Self::get(Path::new(NSFW_HECK_FILE_PATH)).await?;
        let total_hecks = data.len() + 1;
        data.entry(total_hecks.to_string()).insert(heck.clone());
        Self::write(data, Path::new(Path::new(NSFW_HECK_FILE_PATH))).await
    }

    async fn get_nsfw_hecks(&self) -> anyhow::Result<Hecks> {
        let hecks: Hecks = Self::get(Path::new(NSFW_HECK_FILE_PATH)).await?;
        Ok(hecks)
    }

    async fn modify_nsfw_heck(&self, id: usize, heck: &Heck) -> anyhow::Result<()> {
        let data: Hecks = Self::get(Path::new(NSFW_HECK_FILE_PATH)).await?;
        data.entry(id.to_string()).insert(heck.clone());
        Self::write(data, Path::new(Path::new(NSFW_HECK_FILE_PATH))).await
    }

    async fn modify_nsfw_hecks(&self, modified_hecks: Vec<(usize, Heck)>) -> anyhow::Result<()> {
        let data: Hecks = Self::get(Path::new(NSFW_HECK_FILE_PATH)).await?;
        for (heck_id, modified_heck) in modified_hecks {
            data.entry(heck_id.to_string()).insert(modified_heck);
        }
        Self::write(data, Path::new(Path::new(NSFW_HECK_FILE_PATH))).await
    }

    async fn remove_nsfw_heck(&self, id: usize) -> anyhow::Result<()> {
        let data: Hecks = Self::get(Path::new(NSFW_HECK_FILE_PATH)).await?;
        data.remove(&id.to_string());
        Self::write(data, Path::new(Path::new(NSFW_HECK_FILE_PATH))).await
    }

    async fn save_nsfw_hecks(&self, hecks: &Hecks) -> anyhow::Result<()> {
        Self::write(hecks, Path::new(Path::new(NSFW_HECK_FILE_PATH))).await
    }

    // TODO
    async fn get_users(&self) -> luro_model::types::LuroUserData {
        todo!()
    }

    async fn save_users(&self) -> luro_model::types::LuroUserData {
        todo!()
    }

    async fn get_guilds(&self) -> luro_model::types::GuildData {
        todo!()
    }

    async fn save_guilds(&self) -> luro_model::guild_setting::GuildSetting {
        todo!()
    }

    async fn add_sfw_story(&self, story: &Story) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(SFW_STORIES_FILE_PATH)).await?;
        let total_stories = data.len() + 1;
        data.entry(total_stories.to_string()).insert(story.clone());
        Self::write(data, Path::new(Path::new(SFW_STORIES_FILE_PATH))).await
    }

    async fn add_sfw_stories(&self, stories: &[Story]) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(SFW_STORIES_FILE_PATH)).await?;
        let mut total_stories = data.len() + 1;
        for story in stories {
            data.entry(total_stories.to_string()).insert(story.clone());
            total_stories += 1;
        }
        Self::write(data, Path::new(Path::new(SFW_STORIES_FILE_PATH))).await
    }

    async fn get_sfw_stories(&self) -> anyhow::Result<Stories> {
        let data: Stories = Self::get(Path::new(SFW_STORIES_FILE_PATH)).await?;
        Ok(data)
    }

    async fn get_sfw_story(&self, id: &usize) -> anyhow::Result<Story> {
        let data: Stories = Self::get(Path::new(SFW_STORIES_FILE_PATH)).await?;
        let story = match data.get(&id.to_string()) {
            Some(story) => Ok(story.clone()),
            None => Err(anyhow!("Story with ID {id} not present!"))
        };
        story
    }

    async fn modify_sfw_story(&self, id: usize, story: Story) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        data.entry(id.to_string()).insert(story.clone());
        Self::write(data, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn remove_sfw_story(&self, id: usize) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        data.remove(&id.to_string());
        Self::write(data, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn save_sfw_story(&self, story: Story) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        let total_stories = data.len() + 1;
        data.entry(total_stories.to_string()).insert(story.clone());
        Self::write(data, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn save_sfw_stories(&self, stories: Stories) -> anyhow::Result<()> {
        Self::write(stories, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn add_nsfw_story(&self, story: &Story) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        let total_stories = data.len() + 1;
        data.entry(total_stories.to_string()).insert(story.clone());
        Self::write(data, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn add_nsfw_stories(&self, stories: &[Story]) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        let mut total_stories = data.len() + 1;
        for story in stories {
            data.entry(total_stories.to_string()).insert(story.clone());
            total_stories += 1;
        }
        Self::write(data, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn get_nsfw_stories(&self) -> anyhow::Result<Stories> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        Ok(data)
    }

    async fn get_nsfw_story(&self, id: &usize) -> anyhow::Result<Story> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        let story = match data.get(&id.to_string()) {
            Some(story) => Ok(story.clone()),
            None => Err(anyhow!("Story with ID {id} not present!"))
        };
        story
    }

    async fn modify_nsfw_story(&self, id: usize, story: Story) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        data.entry(id.to_string()).insert(story.clone());
        Self::write(data, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn remove_nsfw_story(&self, id: usize) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        data.remove(&id.to_string());
        Self::write(data, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn save_nsfw_story(&self, story: Story) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        let total_stories = data.len() + 1;
        data.entry(total_stories.to_string()).insert(story.clone());
        Self::write(data, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn save_nsfw_stories(&self, stories: Stories) -> anyhow::Result<()> {
        Self::write(stories, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn modify_nsfw_stories(&self, modified_stories: Vec<(usize, Story)>) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(NSFW_STORIES_FILE_PATH)).await?;
        for (id, story) in modified_stories {
            data.entry(id.to_string()).insert(story);
        }
        Self::write(data, Path::new(Path::new(NSFW_STORIES_FILE_PATH))).await
    }

    async fn modify_sfw_stories(&self, modified_stories: Vec<(usize, Story)>) -> anyhow::Result<()> {
        let data: Stories = Self::get(Path::new(SFW_STORIES_FILE_PATH)).await?;
        for (id, story) in modified_stories {
            data.entry(id.to_string()).insert(story);
        }
        Self::write(data, Path::new(Path::new(SFW_STORIES_FILE_PATH))).await
    }

    async fn get_sfw_heck(&self, id: &usize) -> anyhow::Result<Heck> {
        let data: Hecks = Self::get(Path::new(NSFW_HECK_FILE_PATH)).await?;
        let data = match data.get(&id.to_string()) {
            Some(data) => Ok(data.clone()),
            None => Err(anyhow!("Heck with ID {id} not present!"))
        };
        data
    }

    async fn get_nsfw_heck(&self, id: &usize) -> anyhow::Result<Heck> {
        let data: Hecks = Self::get(Path::new(SFW_HECK_FILE_PATH)).await?;
        let data = match data.get(&id.to_string()) {
            Some(data) => Ok(data.clone()),
            None => Err(anyhow!("Heck with ID {id} not present!"))
        };
        data
    }

    async fn get_staff(&self) -> anyhow::Result<LuroUserData> {
        let staff_users: LuroUserData = Default::default();
        for staff in BOT_OWNERS {
            staff_users.insert(staff, self.get_user(staff.get()).await?);
        }
        Ok(staff_users)
    }

    async fn save_interaction(&self, interaction: &Interaction, key: &str) -> anyhow::Result<()> {
        let data: CommandManager = Self::get(Path::new(INTERACTION_FILE_PATH)).await?;
        data.entry(key.to_string()).insert(interaction.clone());
        Self::write(data, Path::new(Path::new(INTERACTION_FILE_PATH))).await
    }

    async fn get_interaction(&self, key: &str) -> anyhow::Result<twilight_model::application::interaction::Interaction> {
        let data: CommandManager = Self::get(Path::new(INTERACTION_FILE_PATH)).await?;
        let data = match data.get(&key.to_string()) {
            Some(data) => Ok(data.clone()),
            None => Err(anyhow!("Interaction with ID {key} not present!"))
        };
        data
    }

    async fn save_quote(&self, quote: &LuroMessage, key: usize) -> anyhow::Result<()> {
        let data: Quotes = Self::get(Path::new(QUOTES_FILE_PATH)).await?;
        data.entry(key.to_string()).insert(quote.clone());
        Self::write(data, Path::new(Path::new(QUOTES_FILE_PATH))).await    }

    async fn get_quote(&self, key: usize) -> anyhow::Result<LuroMessage> {
        let data: Quotes = Self::get(Path::new(QUOTES_FILE_PATH)).await?;
        let data = match data.get(&key.to_string()) {
            Some(data) => Ok(data.clone()),
            None => Err(anyhow!("Interaction with ID {key} not present!"))
        };
        data
    }

    async fn get_quotes(&self) -> anyhow::Result<Quotes> {
        let data: Quotes = Self::get(Path::new(QUOTES_FILE_PATH)).await?;
        Ok(data)
    }
    
}
