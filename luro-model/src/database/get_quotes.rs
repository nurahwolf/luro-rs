use crate::Quotes;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver,> LuroDatabase<D,> {
    pub async fn get_quotes(&self,) -> anyhow::Result<Quotes,> {
        self.driver.get_quotes().await
    }
}
