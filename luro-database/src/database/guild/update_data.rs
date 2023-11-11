use luro_model::types::GuildData;

impl crate::Database {
    pub async fn guild_update_data(&self, data: &GuildData) -> anyhow::Result<u64> {
        self.driver.guild_update_data(data).await
    }
}
