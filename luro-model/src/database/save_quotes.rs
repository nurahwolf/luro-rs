use crate::{
    database_driver::{LuroDatabase, LuroDatabaseDriver},
    Quotes,
};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn save_quotes(&self, quotes: Quotes) -> anyhow::Result<()> {
        self.driver.save_quotes(quotes).await
    }
}
