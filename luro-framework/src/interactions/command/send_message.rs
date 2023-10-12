use luro_model::response::LuroResponse;
use twilight_http::{Error, Response};
use twilight_model::{
    channel::Message,
    id::{marker::ChannelMarker, Id},
};

use crate::CommandInteraction;

impl CommandInteraction {
    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn send_message<F>(&self, channel: &Id<ChannelMarker>, response: F) -> Result<Response<Message>, Error>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse,
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        let mut create_message = self
            .twilight_client
            .create_message(*channel)
            .allowed_mentions(r.allowed_mentions.as_ref());

        if let Some(attachments) = &r.attachments {
            create_message = create_message.attachments(attachments);
        }
        if let Some(components) = &r.components {
            create_message = create_message.components(components);
        }
        if let Some(content) = &r.content {
            create_message = create_message.content(content);
        }
        if let Some(embeds) = &r.embeds {
            create_message = create_message.embeds(embeds);
        }
        if let Some(flags) = r.flags {
            create_message = create_message.flags(flags);
        }
        if let Some(reply) = r.reply {
            create_message = create_message.reply(reply);
        }
        if let Some(stickers) = &r.stickers {
            create_message = create_message.sticker_ids(stickers);
        }
        if let Some(tts) = r.tts {
            create_message = create_message.tts(tts);
        }

        create_message.await
    }
}
