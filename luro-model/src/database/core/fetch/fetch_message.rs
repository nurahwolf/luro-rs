use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

use crate::{database::Error, message::Message};

impl crate::database::Database {
    pub async fn fetch_message(&self, channel_id: Id<ChannelMarker>, message_id: Id<MessageMarker>) -> Result<Message, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_message(message_id).await {
            Ok(Some(user)) => return Ok(user),
            Ok(None) => tracing::debug!("Message '{message_id}' was not found in the database."),
            Err(why) => tracing::error!("Error raised while trying to find message: {why}"),
        };

        Ok(self
            .twilight_client
            .message(channel_id, message_id)
            .await?
            .model()
            .await
            .map(|x| x.into())?)
    }
}
