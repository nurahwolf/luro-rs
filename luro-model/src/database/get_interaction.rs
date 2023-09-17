use twilight_model::application::interaction::Interaction;

use crate::database_driver::{LuroDatabase, LuroDatabaseDriver};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_interaction(&self, key: &str) -> anyhow::Result<Interaction> {
        self.driver.get_interaction(key).await
    }
}
