use luro_model::{database_driver::LuroDatabaseDriver, user::LuroUsers};

use crate::LuroDatabase;

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_staff(&self) -> anyhow::Result<LuroUsers> {
        self.driver.get_staff().await
    }
}
