use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use twilight_model::application::interaction::Interaction;
use twilight_model::{oauth::Application, user::CurrentUser};

use crate::configuration::Configuration;
use crate::guild::{LuroGuild, LuroGuilds};
use crate::heck::{Heck, Hecks};
use crate::message::LuroMessage;
use crate::story::Story;
use crate::user::{LuroUser, LuroUsers};
use crate::{CommandManager, Quotes, Stories};

pub mod drivers;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct HeckManager {
    pub nsfw: Hecks,
    pub sfw: Hecks,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct StoryManager {
    pub nsfw: Stories,
    pub sfw: Stories,
}

/// Luro's database context. This itself just handles an abstraction for saving and loading data from whatever database it is using in the backend, depending on the feature selected.
///
/// NOTE: With the TOML driver, usize keys are serialised as strings!
#[derive(Debug)]
pub struct LuroDatabase<D: LuroDatabaseDriver> {
    pub application: RwLock<Application>,
    pub command_data: RwLock<CommandManager>,
    pub count: RwLock<usize>,
    pub current_user: RwLock<CurrentUser>,
    pub driver: D,
    pub guild_data: Box<RwLock<LuroGuilds>>,
    pub hecks: RwLock<HeckManager>,
    pub quotes: RwLock<Quotes>,
    pub staff: RwLock<LuroUsers>,
    pub stories: RwLock<StoryManager>,
    pub user_data: Box<RwLock<LuroUsers>>,
    pub config: Arc<Configuration<D>>,
}

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Build the key requirements of our database. The rest of our data is fetched as required.
    pub fn build(application: Application, current_user: CurrentUser, config: Arc<Configuration<D>>) -> LuroDatabase<D> {
        Self {
            application: application.into(),
            command_data: Default::default(),
            config: config.clone(),
            count: Default::default(),
            current_user: current_user.into(),
            driver: config.database_driver.clone(),
            guild_data: Default::default(),
            hecks: Default::default(),
            quotes: Default::default(),
            staff: Default::default(),
            stories: Default::default(),
            user_data: Default::default(),
        }
    }
}

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
