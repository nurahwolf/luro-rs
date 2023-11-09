use luro_model::sync::ChannelSync;

impl crate::Database {
    pub async fn channel_update(&self, channel: impl Into<ChannelSync<'_>>) -> anyhow::Result<u64> {
        Ok(self.driver.update_channel(channel).await?)
    }
}