use std::sync::Arc;

use luro_model::{configuration::Configuration, database_driver::LuroDatabaseDriver};

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
mod modify_guild;
mod remove_guild;
mod remove_user;
mod save_hecks;
mod save_interaction;
mod save_quote;
mod save_quotes;
mod save_stories;
mod save_story;
mod update_user;

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Build the key requirements of our database. The rest of our data is fetched as required.
    pub fn build(config: Arc<Configuration<D>>) -> LuroDatabase<D> {
        Self {
            command_data: Default::default(),
            config: config.clone(),
            count: Default::default(),
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
