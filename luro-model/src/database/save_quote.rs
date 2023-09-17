use crate::{
    database_driver::{LuroDatabase, LuroDatabaseDriver},
    message::LuroMessage,
};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn save_quote(&self, key: usize, quote: LuroMessage) -> anyhow::Result<()> {
        self.driver.save_quote(quote, key).await
    }
}
