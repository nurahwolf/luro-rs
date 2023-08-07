#![feature(async_fn_in_trait)]
use luro_model::{
    guild_setting::GuildSetting,
    heck::Heck,
    luro_user::LuroUser,
    story::Story,
    types::{CommandManager, GuildData, Hecks, LuroUserData, Stories}
};
use serde::{Deserialize, Serialize};
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id
};

pub mod drivers;

/// Luro's database context. This itself just handles an abstraction for saving and loading data from whatever database it is using in the backend, depending on the feature selected.
/// Defaults to the TOML driver backend.
///
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct LuroDatabase<D> {
    pub command_data: CommandManager,
    pub driver: D,
    pub guild_data: GuildData,
    pub nsfw_hecks: Hecks,
    pub sfw_hecks: Hecks,
    pub nsfw_stories: Stories,
    pub sfw_stories: Stories,
    pub user_data: LuroUserData
}

impl<D> LuroDatabase<D>
where
    D: LuroDatabaseDriver
{
    /// Attempts to get a user from the cache, otherwise gets the user from the database
    async fn get_user(&self, id: Id<UserMarker>) -> anyhow::Result<LuroUser> {
        match self.user_data.get(&id) {
            Some(user_data) => Ok(user_data.clone()),
            None => self.driver.get_user(id.get()).await
        }
    }

    /// Saves a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    async fn save_user(&self, id: Id<UserMarker>, user: &LuroUser) -> anyhow::Result<Option<LuroUser>> {
        self.driver.save_user(id.get(), user).await?;
        Ok(self.user_data.insert(id, user.clone()))
    }

    /// Removes a user from the database
    async fn remove_user(&self, id: Id<UserMarker>) -> anyhow::Result<()> {
        self.driver.remove_user(id.get()).await
    }

    /// Modifies a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    async fn modify_user(&self, id: Id<UserMarker>, user: &LuroUser) -> anyhow::Result<Option<LuroUser>> {
        self.driver.modify_user(id.get(), user).await?;
        Ok(self.user_data.insert(id, user.clone()))
    }

    /// Attempts to get a user from the cache, otherwise gets the user from the database
    async fn get_guild(&self, id: Id<GuildMarker>) -> anyhow::Result<GuildSetting> {
        match self.guild_data.get(&id) {
            Some(guild) => Ok(guild.clone()),
            None => self.driver.get_guild(id.get()).await
        }
    }

    /// Saves a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    async fn save_guild(&self, id: Id<GuildMarker>, guild: &GuildSetting) -> anyhow::Result<Option<GuildSetting>> {
        self.driver.save_guild(id.get(), guild).await?;
        Ok(self.guild_data.insert(id, guild.clone()))
    }

    /// Removes a user from the database
    async fn remove_guild(&self, id: Id<GuildMarker>) -> anyhow::Result<()> {
        self.driver.remove_guild(id.get()).await
    }

    /// Modifies a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    async fn modify_guild(&self, id: Id<GuildMarker>, guild: &GuildSetting) -> anyhow::Result<Option<GuildSetting>> {
        self.driver.modify_guild(id.get(), guild).await?;
        Ok(self.guild_data.insert(id, guild.clone()))
    }

    /// Modifies a heck, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    async fn modify_heck(&self, id: usize, heck: &Heck, nsfw: bool) -> anyhow::Result<Option<Heck>> {
        match nsfw {
            true => {
                self.driver.modify_nsfw_heck(id, heck).await?;
                Ok(self.nsfw_hecks.insert(id, heck.clone()))
            }
            false => {
                self.driver.modify_nsfw_heck(id, heck).await?;
                Ok(self.nsfw_hecks.insert(id, heck.clone()))
            }
        }
    }

    /// Modifies multiple hecks, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    async fn modify_hecks(&self, hecks: Vec<(usize, Heck)>, nsfw: bool) -> anyhow::Result<Vec<(usize, Heck)>> {
        match nsfw {
            true => {
                let mut old_hecks = vec![];
                self.driver.modify_nsfw_hecks(hecks.clone()).await?;
                for (heck_id, heck) in hecks {
                    self.nsfw_hecks.insert(heck_id, heck.clone());
                    old_hecks.push((heck_id, heck))
                }
                Ok(old_hecks)
            }
            false => {
                let mut old_hecks = vec![];
                self.driver.modify_sfw_hecks(hecks.clone()).await?;
                for (heck_id, heck) in hecks {
                    self.sfw_hecks.insert(heck_id, heck.clone());
                    old_hecks.push((heck_id, heck))
                }
                Ok(old_hecks)
            }
        }
    }

    /// Modifies a heck, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    async fn modify_story(&self, id: usize, story: &Story, nsfw: bool) -> anyhow::Result<Option<Story>> {
        match nsfw {
            true => {
                self.driver.modify_nsfw_story(id, story.clone()).await?;
                Ok(self.nsfw_stories.insert(id, story.clone()))
            }
            false => {
                self.driver.modify_sfw_story(id, story.clone()).await?;
                Ok(self.sfw_stories.insert(id, story.clone()))
            }
        }
    }

    /// Modifies multiple hecks, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    async fn modify_stories(&self, stories: Vec<(usize, Story)>, nsfw: bool) -> anyhow::Result<Vec<(usize, Story)>> {
        match nsfw {
            true => {
                let mut old_stories = vec![];
                self.driver.modify_nsfw_stories(stories.clone()).await?;
                for (id, story) in stories {
                    self.nsfw_stories.insert(id, story.clone());
                    old_stories.push((id, story))
                }
                Ok(old_stories)
            }
            false => {
                let mut old_stories = vec![];
                self.driver.modify_sfw_stories(stories.clone()).await?;
                for (id, story) in stories {
                    self.sfw_stories.insert(id, story.clone());
                    old_stories.push((id, story))
                }
                Ok(old_stories)
            }
        }
    }

    /// Attempts to get a story from the cache, otherwise gets the user from the database
    async fn get_story(&self, id: &usize, nsfw: bool) -> anyhow::Result<Story> {
        match nsfw {
            true => match self.nsfw_stories.get(id) {
                Some(data) => Ok(data.clone()),
                None => self.driver.get_nsfw_story(id).await
            },
            false => match self.sfw_stories.get(id) {
                Some(data) => Ok(data.clone()),
                None => self.driver.get_sfw_story(id).await
            }
        }
    }

    /// Attempts to get a heck from the cache, otherwise gets the user from the database
    async fn get_heck(&self, id: &usize, nsfw: bool) -> anyhow::Result<Heck> {
        match nsfw {
            true => match self.nsfw_hecks.get(id) {
                Some(data) => Ok(data.clone()),
                None => self.driver.get_nsfw_heck(id).await
            },
            false => match self.sfw_hecks.get(id) {
                Some(data) => Ok(data.clone()),
                None => self.driver.get_sfw_heck(id).await
            }
        }
    }
}

/// This trait enforces all implementation required to be compatible with [LuroDatabase].
pub trait LuroDatabaseDriver {
    async fn add_guild(&self, id: u64, user: &GuildSetting) -> anyhow::Result<()>;
    async fn add_nsfw_heck(&self, heck: &Heck) -> anyhow::Result<()>;
    async fn add_nsfw_stories(&self, stories: &[Story]) -> anyhow::Result<()>;
    async fn add_nsfw_story(&self, story: &Story) -> anyhow::Result<()>;
    async fn add_sfw_heck(&self, heck: &Heck) -> anyhow::Result<()>;
    async fn add_sfw_stories(&self, stories: &[Story]) -> anyhow::Result<()>;
    async fn add_sfw_story(&self, story: &Story) -> anyhow::Result<()>;
    async fn get_guild(&self, id: u64) -> anyhow::Result<GuildSetting>;
    async fn get_nsfw_hecks(&self) -> anyhow::Result<Hecks>;
    async fn get_sfw_heck(&self, id: &usize) -> anyhow::Result<Heck>;
    async fn get_nsfw_heck(&self, id: &usize) -> anyhow::Result<Heck>;
    async fn get_nsfw_stories(&self) -> anyhow::Result<Stories>;
    async fn get_nsfw_story(&self, id: &usize) -> anyhow::Result<Story>;
    async fn get_sfw_hecks(&self) -> anyhow::Result<Hecks>;
    async fn get_sfw_stories(&self) -> anyhow::Result<Stories>;
    async fn get_sfw_story(&self, id: &usize) -> anyhow::Result<Story>;
    async fn get_user(&self, id: u64) -> anyhow::Result<LuroUser>;
    async fn modify_guild(&self, id: u64, user: &GuildSetting) -> anyhow::Result<()>;
    async fn modify_nsfw_heck(&self, id: usize, heck: &Heck) -> anyhow::Result<()>;
    async fn modify_nsfw_hecks(&self, modified_hecks: Vec<(usize, Heck)>) -> anyhow::Result<()>;
    async fn modify_nsfw_stories(&self, modified_stories: Vec<(usize, Story)>) -> anyhow::Result<()>;
    async fn modify_sfw_stories(&self, modified_stories: Vec<(usize, Story)>) -> anyhow::Result<()>;
    async fn modify_nsfw_story(&self, id: usize, story: Story) -> anyhow::Result<()>;
    async fn modify_sfw_heck(&self, id: usize, heck: &Heck) -> anyhow::Result<()>;
    async fn modify_sfw_hecks(&self, modified_hecks: Vec<(usize, Heck)>) -> anyhow::Result<()>;
    async fn modify_sfw_story(&self, id: usize, story: Story) -> anyhow::Result<()>;
    async fn modify_user(&self, id: u64, user: &LuroUser) -> anyhow::Result<()>;
    async fn remove_guild(&self, id: u64) -> anyhow::Result<()>;
    async fn remove_nsfw_heck(&self, id: usize) -> anyhow::Result<()>;
    async fn remove_nsfw_story(&self, id: usize) -> anyhow::Result<()>;
    async fn remove_sfw_heck(&self, id: usize) -> anyhow::Result<()>;
    async fn remove_sfw_story(&self, id: usize) -> anyhow::Result<()>;
    async fn remove_user(&self, id: u64) -> anyhow::Result<()>;
    async fn save_guild(&self, id: u64, user: &GuildSetting) -> anyhow::Result<()>;
    async fn save_nsfw_hecks(&self, hecks: &Hecks) -> anyhow::Result<()>;
    async fn save_nsfw_stories(&self, stories: Stories) -> anyhow::Result<()>;
    async fn save_nsfw_story(&self, story: Story) -> anyhow::Result<()>;
    async fn save_sfw_hecks(&self, hecks: &Hecks) -> anyhow::Result<()>;
    async fn save_sfw_stories(&self, stories: Stories) -> anyhow::Result<()>;
    async fn save_sfw_story(&self, story: Story) -> anyhow::Result<()>;
    async fn save_user(&self, id: u64, user: &LuroUser) -> anyhow::Result<()>;
    // TODO
    async fn get_users(&self) -> LuroUserData;
    async fn save_users(&self) -> LuroUserData;
    async fn get_guilds(&self) -> GuildData;
    async fn save_guilds(&self) -> GuildSetting;
}
