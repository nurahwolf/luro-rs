use luro_model::database_driver::LuroDatabaseDriver;

use crate::{Quotes, LuroDatabase};


impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_quotes(&self) -> anyhow::Result<Quotes> {
        self.driver.get_quotes().await
    }
}
