use std::sync::Arc;

use luro_model::{database_driver::LuroDatabaseDriver, configuration::Configuration};
use twilight_model::{oauth::Application, user::CurrentUser};

use crate::LuroDatabase;

mod add_heck;
mod flush;
mod get_guild;
mod get_heck;
mod get_hecks;
mod get_interaction;
mod get_quote;
mod get_quotes;
mod get_staff;
mod get_stories;
mod get_story;
mod get_user;
mod remove_guild;
mod remove_user;
mod save_guild;
mod save_hecks;
mod save_interaction;
mod save_quote;
mod save_quotes;
mod save_stories;
mod save_story;
mod save_user;

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