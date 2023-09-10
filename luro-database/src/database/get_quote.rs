use luro_model::{message::LuroMessage, database_driver::LuroDatabaseDriver};
use tracing::error;

use crate::LuroDatabase;

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_quote(&self, key: usize) -> anyhow::Result<LuroMessage> {
        let quote = match self.quotes.read() {
            Ok(quotes) => quotes.get(&key).cloned(),
            Err(why) => {
                error!(why = ?why, "Quotes are poisoned! I'm returning the quote from the driver directly, bypassing the cache. This NEEDS to be investigated and fixed!");
                None
            }
        };

        match quote {
            Some(quote) => Ok(quote),
            None => self.driver.get_quote(key).await,
        }
    }
}
