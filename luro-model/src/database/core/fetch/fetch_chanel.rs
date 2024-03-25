use twilight_model::{
    channel::Channel,
    id::{marker::ChannelMarker, Id},
};

use crate::database::Error;

impl crate::database::Database {
    pub async fn fetch_channel(&self, channel_id: Id<ChannelMarker>) -> Result<Channel, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_channel(channel_id).await {
            Ok(Some(data)) => return Ok(data),
            Ok(None) => tracing::debug!("Channel `{channel_id}` was not found in the database."),
            Err(why) => tracing::error!(?why, "Error raised while trying to find channel `{channel_id}`"),
        };

        Ok(self.twilight_driver.fetch_channel(channel_id).await?)
    }
}
