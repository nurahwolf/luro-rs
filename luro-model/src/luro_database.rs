use std::{mem, sync::RwLock};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use twilight_model::{
    id::{
        marker::{GuildMarker, UserMarker, MessageMarker},
        Id
    },
    oauth::Application,
    user::CurrentUser
};

use crate::{
    guild_setting::GuildSetting,
    heck::Heck,
    luro_database_driver::LuroDatabaseDriver,
    luro_user::LuroUser,
    story::Story,
    types::{CommandManager, GuildData, Hecks, LuroUserData, Stories}
};

/// Luro's database context. This itself just handles an abstraction for saving and loading data from whatever database it is using in the backend, depending on the feature selected.
/// Defaults to the TOML driver backend.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct LuroDatabase<D: LuroDatabaseDriver> {
    pub available_random_nsfw_hecks: RwLock<Vec<usize>>,
    pub available_random_sfw_hecks: RwLock<Vec<usize>>,
    pub application: RwLock<Application>,
    pub command_data: CommandManager<Id<MessageMarker>>,
    pub modal_interaction_data: CommandManager<String>,
    pub count: RwLock<usize>,
    pub current_user: RwLock<CurrentUser>,
    pub driver: D,
    pub guild_data: GuildData,
    #[serde(default)]
    pub nsfw_hecks: Hecks,
    pub nsfw_stories: Stories,
    pub staff: LuroUserData,
    #[serde(default)]
    pub sfw_hecks: Hecks,
    pub sfw_stories: Stories,
    pub user_data: LuroUserData
}

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Build the key requirements of our database. The rest of our data is fetched as required.
    pub async fn build(application: Application, current_user: CurrentUser, driver: D) -> Self {
        Self {
            application: application.into(),
            command_data: Default::default(),
            count: Default::default(),
            current_user: current_user.into(),
            driver,
            guild_data: Default::default(),
            nsfw_hecks: Default::default(),
            nsfw_stories: Default::default(),
            staff: Default::default(),
            sfw_hecks: Default::default(),
            sfw_stories: Default::default(),
            user_data: Default::default(),
            available_random_nsfw_hecks: Default::default(),
            available_random_sfw_hecks: Default::default(),
            modal_interaction_data: Default::default(),
        }
    }

    /// Attempts to get a user from the cache, otherwise gets the user from the database
    pub async fn get_user(&self, user_id: &Id<UserMarker>) -> anyhow::Result<LuroUser> {
        match self.user_data.get(user_id) {
            Some(user_data) => Ok(user_data.clone()),
            None => self.driver.get_user(user_id.get()).await
        }
    }

    /// Saves a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn save_user(&self, user_id: &Id<UserMarker>, user: &LuroUser) -> anyhow::Result<Option<LuroUser>> {
        self.driver.save_user(user_id.get(), user).await?;
        Ok(self.user_data.insert(*user_id, user.clone()))
    }

    /// Removes a user from the database
    pub async fn remove_user(&self, user_id: &Id<UserMarker>) -> anyhow::Result<()> {
        self.driver.remove_user(user_id.get()).await
    }

    /// Modifies a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn modify_user(&self, user_id: &Id<UserMarker>, user: &LuroUser) -> anyhow::Result<Option<LuroUser>> {
        self.driver.modify_user(user_id.get(), user).await?;
        Ok(self.user_data.insert(*user_id, user.clone()))
    }

    /// Attempts to get a user from the cache, otherwise gets the user from the database
    pub async fn get_guild(&self, guild_id: &Id<GuildMarker>) -> anyhow::Result<GuildSetting> {
        match self.guild_data.get(guild_id) {
            Some(guild) => Ok(guild.clone()),
            None => self.driver.get_guild(guild_id.get()).await
        }
    }

    /// Saves a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn save_guild(&self, id: Id<GuildMarker>, guild: &GuildSetting) -> anyhow::Result<Option<GuildSetting>> {
        self.driver.save_guild(id.get(), guild).await?;
        Ok(self.guild_data.insert(id, guild.clone()))
    }

    /// Removes a user from the database
    pub async fn remove_guild(&self, id: Id<GuildMarker>) -> anyhow::Result<()> {
        self.driver.remove_guild(id.get()).await
    }

    /// Modifies a guild and then flushes the result to disk.
    /// Returns the old settings if they existed in the cache
    pub async fn update_guild(&self, id: Id<GuildMarker>, guild: &GuildSetting) -> anyhow::Result<Option<GuildSetting>> {
        self.driver.update_guild(id.get(), guild).await?;
        Ok(self.guild_data.insert(id, guild.clone()))
    }

    /// Modifies a heck, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn modify_heck(&self, id: usize, heck: &Heck, nsfw: bool) -> anyhow::Result<Option<Heck>> {
        match nsfw {
            true => {
                self.driver.modify_nsfw_heck(id, heck).await?;
                Ok(self.nsfw_hecks.insert(id.to_string(), heck.clone()))
            }
            false => {
                self.driver.modify_nsfw_heck(id, heck).await?;
                Ok(self.nsfw_hecks.insert(id.to_string(), heck.clone()))
            }
        }
    }

    /// Modifies multiple hecks, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn modify_hecks(&self, hecks: Vec<(usize, Heck)>, nsfw: bool) -> anyhow::Result<Vec<(usize, Heck)>> {
        match nsfw {
            true => {
                let mut old_hecks = vec![];
                self.driver.modify_nsfw_hecks(hecks.clone()).await?;
                for (heck_id, heck) in hecks {
                    self.nsfw_hecks.insert(heck_id.to_string(), heck.clone());
                    old_hecks.push((heck_id, heck))
                }
                Ok(old_hecks)
            }
            false => {
                let mut old_hecks = vec![];
                self.driver.modify_sfw_hecks(hecks.clone()).await?;
                for (heck_id, heck) in hecks {
                    self.sfw_hecks.insert(heck_id.to_string(), heck.clone());
                    old_hecks.push((heck_id, heck))
                }
                Ok(old_hecks)
            }
        }
    }

    /// Modifies a heck, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn modify_story(&self, id: usize, story: &Story, nsfw: bool) -> anyhow::Result<Option<Story>> {
        match nsfw {
            true => {
                self.driver.modify_nsfw_story(id, story.clone()).await?;
                Ok(self.nsfw_stories.insert(id.to_string(), story.clone()))
            }
            false => {
                self.driver.modify_sfw_story(id, story.clone()).await?;
                Ok(self.sfw_stories.insert(id.to_string(), story.clone()))
            }
        }
    }

    /// Modifies multiple hecks, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn modify_stories(&self, stories: Vec<(usize, Story)>, nsfw: bool) -> anyhow::Result<Vec<(usize, Story)>> {
        match nsfw {
            true => {
                let mut old_stories = vec![];
                self.driver.modify_nsfw_stories(stories.clone()).await?;
                for (id, story) in stories {
                    self.nsfw_stories.insert(id.to_string(), story.clone());
                    old_stories.push((id, story))
                }
                Ok(old_stories)
            }
            false => {
                let mut old_stories = vec![];
                self.driver.modify_sfw_stories(stories.clone()).await?;
                for (id, story) in stories {
                    self.sfw_stories.insert(id.to_string(), story.clone());
                    old_stories.push((id, story))
                }
                Ok(old_stories)
            }
        }
    }

    /// Attempts to get a story from the cache, otherwise gets the user from the database
    pub async fn get_story(&self, id: &usize, nsfw: bool) -> anyhow::Result<Story> {
        match nsfw {
            true => match self.nsfw_stories.get(&id.to_string()) {
                Some(data) => Ok(data.clone()),
                None => self.driver.get_nsfw_story(id).await
            },
            false => match self.sfw_stories.get(&id.to_string()) {
                Some(data) => Ok(data.clone()),
                None => self.driver.get_sfw_story(id).await
            }
        }
    }

    /// Attempts to get stories from the cache, otherwise gets the stories from the database
    pub async fn get_stories(&self, nsfw: bool) -> anyhow::Result<Stories> {
        match nsfw {
            true => self.driver.get_nsfw_stories().await,
            false => self.driver.get_sfw_stories().await
        }
    }

    /// Attempts to get a heck from the cache, otherwise gets the user from the database
    pub async fn get_heck(&self, id: &usize, nsfw: bool) -> anyhow::Result<Heck> {
        match nsfw {
            true => match self.nsfw_hecks.get(&id.to_string()) {
                Some(data) => Ok(data.clone()),
                None => self.driver.get_nsfw_heck(id).await
            },
            false => match self.sfw_hecks.get(&id.to_string()) {
                Some(data) => Ok(data.clone()),
                None => self.driver.get_sfw_heck(id).await
            }
        }
    }

    /// Attempts to get all hecks from the cache, otherwise gets the user from the database
    pub async fn get_hecks(&self, nsfw: bool) -> anyhow::Result<Hecks> {
        let hecks = match nsfw {
            true => &self.nsfw_hecks,
            false => &self.sfw_hecks
        };

        Ok(match hecks.is_empty() {
            true => {
                if nsfw {
                    self.driver.get_nsfw_hecks().await?
                } else {
                    self.driver.get_sfw_hecks().await?
                }
            }
            false => hecks.clone()
        })
    }

    pub async fn get_staff(&self) -> anyhow::Result<LuroUserData> {
        self.driver.get_staff().await
    }

    pub async fn reload_global_heck_ids(&self, nsfw: bool) -> anyhow::Result<()> {
        let hecks = match nsfw {
            true => self.driver.get_nsfw_hecks().await?,
            false => self.driver.get_sfw_hecks().await?
        };

        let heck_db_raw = match nsfw {
            true => self.available_random_nsfw_hecks.write(),
            false => self.available_random_sfw_hecks.write()
        };

        let mut heck_db = match heck_db_raw {
            Ok(lock) => lock,
            Err(_) => return Err(Error::msg("Lock was poisoned"))
        };

        for (heck_id, _) in hecks {
            match nsfw {
                true => heck_db.push(heck_id.parse()?),
                false => heck_db.push(heck_id.parse()?)
            }
        }
        mem::drop(heck_db);

        Ok(())
    }

    pub async fn reload_guild_heck_ids(&self, guild_id: &Id<GuildMarker>, nsfw: bool) -> anyhow::Result<()> {
        let mut guild_setings = self.get_guild(guild_id).await?;

        match nsfw {
            true => {
                for (heck_id, _) in guild_setings.nsfw_hecks {
                    guild_setings.available_random_nsfw_hecks.push(heck_id.parse()?)
                }
            }
            false => {
                for (heck_id, _) in guild_setings.sfw_hecks {
                    guild_setings.available_random_sfw_hecks.push(heck_id.parse()?)
                }
            }
        }

        Ok(())
    }
}
