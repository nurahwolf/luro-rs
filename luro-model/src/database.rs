use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use twilight_http::Client;
use twilight_model::{application::interaction::Interaction, oauth::Application, user::CurrentUser};

use crate::guild::LuroGuilds;
use crate::heck::Hecks;
use crate::message::LuroMessage;
use crate::user::LuroUsers;
use crate::{CommandManager, Quotes, Stories};

use self::drivers::LuroDatabaseDriver;

pub mod drivers;
mod flush;
mod get_guild;
mod get_heck;
mod get_hecks;
mod get_quote;
mod get_quotes;
mod get_stories;
mod get_story;
mod get_user;
mod remove_guild;
mod remove_user;
mod save_guild;
mod save_heck;
mod save_hecks;
mod save_stories;
mod save_story;
mod save_user;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct HeckManager {
    pub nsfw: Hecks,
    pub sfw: Hecks
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct StoryManager {
    pub nsfw: Stories,
    pub sfw: Stories
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
    pub twilight_client: Arc<Client>
}

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Build the key requirements of our database. The rest of our data is fetched as required.
    pub fn build(
        application: Application,
        current_user: CurrentUser,
        twilight_client: Arc<Client>,
        driver: D
    ) -> LuroDatabase<D> {
        Self {
            application: application.into(),
            command_data: Default::default(),
            count: Default::default(),
            current_user: current_user.into(),
            driver,
            guild_data: Default::default(),
            stories: Default::default(),
            hecks: Default::default(),
            staff: Default::default(),
            user_data: Default::default(),
            quotes: Default::default(),
            twilight_client
        }
    }

    pub async fn get_staff(&self) -> anyhow::Result<LuroUsers> {
        self.driver.get_staff().await
    }

    pub async fn save_interaction(&self, key: &str, interaction: &Interaction) -> anyhow::Result<()> {
        self.driver.save_interaction(interaction, key).await
    }

    pub async fn get_interaction(&self, key: &str) -> anyhow::Result<Interaction> {
        self.driver.get_interaction(key).await
    }

    pub async fn save_quote(&self, key: usize, quote: LuroMessage) -> anyhow::Result<()> {
        self.driver.save_quote(quote, key).await
    }

    pub async fn save_quotes(&self, quotes: Quotes) -> anyhow::Result<()> {
        self.driver.save_quotes(quotes).await
    }
}
