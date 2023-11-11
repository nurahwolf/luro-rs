use luro_model::message::Message;
use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

impl crate::Database {
    pub async fn message_fetch(&self, message_id: Id<MessageMarker>, channel_id: Option<Id<ChannelMarker>>) -> anyhow::Result<Message> {
        if let Ok(Some(message)) = self.driver.get_message(message_id).await {
            return Ok(message);
        }

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

        match self.api_client.message(channel_id, message_id).await {
            Ok(response) => Ok(response.model().await?.into()),
            Err(why) => Err(anyhow::anyhow!("Failed to find the message: {why}")),
        }
    }
}
