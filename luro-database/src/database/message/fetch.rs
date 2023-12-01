use luro_model::types::Message;
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

impl crate::Database {
    pub async fn message_fetch(&self, message_id: Id<MessageMarker>, channel_id: Option<Id<ChannelMarker>>) -> anyhow::Result<Message> {
        match self.driver.get_message(message_id).await {
            Ok(Some(message)) => return Ok(message),
            Ok(None) => (),
            Err(why) => tracing::error!("Failed to fetch message {message_id} - channel {channel_id:?} from database: {why:?}"),
        }

        tracing::info!("message_fetch - Failed to find the message in the database, falling back to Twilight!");

        // TODO: Implement this
        // #[cfg(feature = "database-cache-twilight")]
        // if let Some(message) = self.cache.message(message_id) {
        //     let test = message.value();
        //     return Ok(message.value().clone().interaction())
        // }

        let channel_id = match channel_id {
            Some(channel_id) => channel_id,
            None => {
                return Err(anyhow::anyhow!(
                    "Could not fetch message from database or driver, and no channel ID was passed to try fetching via the API"
                ))
            }
        };

       let message =  match self.api_client.message(channel_id, message_id).await {
            Ok(response) => response.model().await?,
            Err(why) => return Err(anyhow::anyhow!("Failed to find the message: {why}")),
        };

        match self.driver.update_message(&message).await {
            Ok(update) => tracing::info!("Messaged was flushed back to database with '{update}' updates"),
            Err(why) => tracing::error!("Failed to flush message back to database: {why:?}"),
        }

        Ok(message.into())
    }
}
