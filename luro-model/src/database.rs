use std::sync::{RwLock, Arc};
use serde::{Deserialize, Serialize};
use twilight_model::{oauth::Application, user::CurrentUser};

use crate::configuration::Configuration;
use crate::guild::LuroGuilds;
use crate::heck::Hecks;
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
mod get_staff;
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
mod save_interaction;
mod get_interaction;
mod save_quote;
mod save_quotes;

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
    pub config: Arc<Configuration<D>>
}

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Build the key requirements of our database. The rest of our data is fetched as required.
    pub fn build(
        application: Application,
        current_user: CurrentUser,
        config: Arc<Configuration<D>>,
    ) -> LuroDatabase<D> {
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
