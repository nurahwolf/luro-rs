use luro_model::{database_driver::LuroDatabaseDriver, message::LuroMessage};

use crate::LuroDatabase;

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn save_quote(&self, key: usize, quote: LuroMessage) -> anyhow::Result<()> {
        self.driver.save_quote(quote, key).await
    }
}
