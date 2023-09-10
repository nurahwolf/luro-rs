use crate::message::LuroMessage;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn save_quote(&self, key: usize, quote: LuroMessage) -> anyhow::Result<()> {
        self.driver.save_quote(quote, key).await
    }
}
