use async_trait::async_trait;
use twilight_model::application::interaction::Interaction;

#[cfg(feature = "toml-driver")]
pub mod toml;

use crate::{
    guild::LuroGuild,
    heck::{Heck, Hecks},
    message::LuroMessage,
    story::Story,
    user::{LuroUser, LuroUsers},
    Quotes, Stories
};

/// This trait enforces all implementation required to be compatible with [LuroDatabase].
#[async_trait]
pub trait LuroDatabaseDriver: Clone + Sync + Send {
    async fn add_heck(&self, heck: &Heck, nsfw: bool) -> anyhow::Result<()>;
    async fn add_stories(&self, stories: &Stories, nsfw: bool) -> anyhow::Result<()>;
    async fn add_story(&self, story: &Story, nsfw: bool) -> anyhow::Result<()>;
    async fn get_guild(&self, id: u64) -> anyhow::Result<LuroGuild>;
    async fn get_hecks(&self, nsfw: bool) -> anyhow::Result<Hecks>;
    async fn get_heck(&self, id: &usize, nsfw: bool) -> anyhow::Result<Heck>;
    async fn get_stories(&self, nsfw: bool) -> anyhow::Result<Stories>;
    async fn get_story(&self, id: &usize, nsfw: bool) -> anyhow::Result<Story>;
    async fn get_user(&self, id: u64) -> anyhow::Result<LuroUser>;
    async fn update_guild(&self, id: u64, guild: &LuroGuild) -> anyhow::Result<()>;
    async fn modify_heck(&self, id: usize, heck: &Heck, nsfw: bool) -> anyhow::Result<()>;
    async fn modify_hecks(&self, modified_hecks: &Hecks, nsfw: bool) -> anyhow::Result<()>;
    async fn modify_stories(&self, modified_stories: &Stories, nsfw: bool) -> anyhow::Result<()>;
    async fn modify_story(&self, id: &usize, story: &Story, nsfw: bool) -> anyhow::Result<()>;
    async fn modify_user(&self, id: u64, user: &LuroUser) -> anyhow::Result<()>;
    async fn remove_guild(&self, id: u64) -> anyhow::Result<()>;
    async fn remove_heck(&self, id: usize, nsfw: bool) -> anyhow::Result<()>;
    async fn remove_story(&self, id: usize, nsfw: bool) -> anyhow::Result<()>;
    async fn remove_user(&self, id: u64) -> anyhow::Result<()>;
    async fn save_guild(&self, id: u64, guild: LuroGuild) -> anyhow::Result<()>;
    async fn save_hecks(&self, hecks: Hecks, nsfw: bool) -> anyhow::Result<()>;
    async fn save_stories(&self, stories: Stories, nsfw: bool) -> anyhow::Result<()>;
    async fn save_story(&self, story: &Story, nsfw: bool) -> anyhow::Result<()>;
    async fn save_user(&self, id: u64, user: &LuroUser) -> anyhow::Result<()>;
    async fn get_staff(&self) -> anyhow::Result<LuroUsers>;
    async fn save_interaction(&self, interaction: &Interaction, key: &str) -> anyhow::Result<()>;
    async fn get_interaction(&self, key: &str) -> anyhow::Result<Interaction>;
    async fn save_quote(&self, quote: LuroMessage, key: usize) -> anyhow::Result<()>;
    async fn save_quotes(&self, quotes: Quotes) -> anyhow::Result<()>;
    async fn get_quote(&self, key: usize) -> anyhow::Result<LuroMessage>;
    async fn get_quotes(&self) -> anyhow::Result<Quotes>;
}
