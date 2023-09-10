use twilight_model::application::interaction::Interaction;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_interaction(&self, key: &str) -> anyhow::Result<Interaction> {
        self.driver.get_interaction(key).await
    }
}
