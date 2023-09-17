use crate::{
    database_driver::{LuroDatabase, LuroDatabaseDriver},
    Quotes,
};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_quotes(&self) -> anyhow::Result<Quotes> {
        self.driver.get_quotes().await
    }
}
