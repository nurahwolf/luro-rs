use luro_model::sync::GuildSync;

impl crate::Database {
    pub async fn guild_update(&self, guild: impl Into<GuildSync>) -> anyhow::Result<u64> {
        self.driver.update_guild(guild).await
    }
}