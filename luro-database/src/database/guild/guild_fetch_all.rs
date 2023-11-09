use luro_model::types::Guild;

impl crate::Database {
    pub async fn guild_fetch_all(&self) -> anyhow::Result<Vec<Guild>> {
        self.driver.get_all_guilds().await
    }
}