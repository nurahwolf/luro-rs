use luro_model::types::Channel;
use twilight_model::id::{Id, marker::ChannelMarker};

impl crate::Database {
    pub async fn channel_fetch(&self, channel_id: Id<ChannelMarker>) -> anyhow::Result<Channel> {
        if let Ok(Some(channel)) = self.driver.channel_fetch(channel_id).await {
            return Ok(channel)
        }


        tracing::warn!("Failed to find channel `{channel_id}` in the database, falling back to Twilight");
        let twilight_channel = self.api_client.channel(channel_id).await?.model().await?;

        if let Err(why) = self.driver.update_channel(twilight_channel.clone()).await {
            tracing::error!(why = ?why, "failed to sync channel `{channel_id}` to the database");
        }

        Ok(twilight_channel.into())
    }
}