use luro_model::database_driver::LuroDatabaseDriver;

use crate::{Quotes, LuroDatabase};


impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn save_quotes(&self, quotes: Quotes) -> anyhow::Result<()> {
        self.driver.save_quotes(quotes).await
    }
}
