
use luro_model::{user::LuroUsers, database_driver::LuroDatabaseDriver};

use crate::LuroDatabase;



impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_staff(&self) -> anyhow::Result<LuroUsers> {
        self.driver.get_staff().await
    }
}