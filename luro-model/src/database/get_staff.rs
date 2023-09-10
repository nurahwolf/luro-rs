use crate::user::LuroUsers;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_staff(&self) -> anyhow::Result<LuroUsers> {
        self.driver.get_staff().await
    }
}