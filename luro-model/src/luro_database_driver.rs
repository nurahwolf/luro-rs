use twilight_model::application::interaction::Interaction;

use crate::{
    guild_setting::GuildSetting,
    heck::Heck,
    luro_user::LuroUser,
    story::Story,
    types::{GuildData, Hecks, LuroUserData, Stories}
};

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
    async fn update_guild(&self, id: u64, user: &GuildSetting) -> anyhow::Result<()>;
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
    async fn get_staff(&self) -> anyhow::Result<LuroUserData>;
    async fn save_interaction(&self, interaction: &Interaction, key: &str) -> anyhow::Result<()>;
    async fn get_interaction(&self, key: &str) -> anyhow::Result<Interaction>;
    // TODO
    async fn get_users(&self) -> LuroUserData;
    async fn save_users(&self) -> LuroUserData;
    async fn get_guilds(&self) -> GuildData;
    async fn save_guilds(&self) -> GuildSetting;
}
