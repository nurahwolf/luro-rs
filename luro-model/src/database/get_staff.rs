use crate::{
    database_driver::{LuroDatabase, LuroDatabaseDriver},
    user::LuroUsers,
};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_staff(&self) -> anyhow::Result<LuroUsers> {
        self.driver.get_staff().await
    }
}
