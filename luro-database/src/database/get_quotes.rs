use luro_model::database_driver::LuroDatabaseDriver;

use crate::{LuroDatabase, Quotes};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_quotes(&self) -> anyhow::Result<Quotes> {
        self.driver.get_quotes().await
    }
}
