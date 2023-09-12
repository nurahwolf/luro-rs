use luro_model::database_driver::LuroDatabaseDriver;
use twilight_model::application::interaction::Interaction;

use crate::LuroDatabase;

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn save_interaction(&self, key: &str, interaction: &Interaction) -> anyhow::Result<()> {
        self.driver.save_interaction(interaction, key).await
    }
}