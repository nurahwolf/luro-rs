use luro_model::{response::LuroResponse, database_driver::LuroDatabaseDriver};
use twilight_http::{Error, Response};
use twilight_model::{
    channel::Message,
    id::{marker::ChannelMarker, Id},
};

use crate::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    /// Sends a message to the specified channel
    ///
    /// NOTE: Make sure to create a private DM channel if you want to DM someone!
    pub async fn send_message<F: FnOnce(&mut LuroResponse) -> &mut LuroResponse>(
        &self,
        channel_id: &Id<ChannelMarker>,
        response: F,
    ) -> Result<Response<Message>, Error> {
        let mut r = LuroResponse::default();
        response(&mut r);

        let mut sender = self
            .twilight_client
            .create_message(*channel_id)
            .allowed_mentions(r.allowed_mentions.as_ref())
            .tts(r.tts.unwrap_or_default());

        if let Some(ref data) = r.attachments {
            sender = sender.attachments(data);
        }
        if let Some(ref data) = r.components {
            sender = sender.components(data);
        }
        if let Some(ref data) = r.embeds {
            sender = sender.embeds(data);
        }
        if let Some(ref data) = r.flags {
            sender = sender.flags(*data);
        }
        if let Some(ref data) = r.reply {
            sender = sender.reply(*data);
        }
        if let Some(ref data) = r.stickers {
            sender = sender.sticker_ids(data);
        }

        sender.await
    }
}
