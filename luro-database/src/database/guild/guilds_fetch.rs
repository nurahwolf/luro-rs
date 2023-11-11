use luro_model::types::Guild;

impl crate::Database {
    pub async fn guilds_fetch(&self) -> anyhow::Result<Vec<Guild>> {
        self.driver.get_all_guilds().await
    }
}