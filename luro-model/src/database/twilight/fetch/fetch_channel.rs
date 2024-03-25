use twilight_model::{
    channel::Channel,
    id::{marker::ChannelMarker, Id},
};

use crate::database::twilight::{Database, Error};

impl Database {
    pub async fn fetch_channel(&self, channel_id: Id<ChannelMarker>) -> Result<Channel, Error> {
        let twilight_channel = self.twilight_client.channel(channel_id).await?.model().await?;
        Ok(twilight_channel)
    }
}
