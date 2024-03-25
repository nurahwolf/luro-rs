use twilight_model::id::{
    marker::{ChannelMarker, MessageMarker},
    Id,
};

use crate::{database::Error, message::Message};

impl crate::database::twilight::Database {
    pub async fn fetch_message(&self, channel_id: Id<ChannelMarker>, message_id: Id<MessageMarker>) -> Result<Message, Error> {
        let twilight_message = self.twilight_client.message(channel_id, message_id).await?.model().await?;
        Ok(twilight_message.into())
    }
}
