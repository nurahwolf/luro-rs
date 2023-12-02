impl crate::ComponentInteraction {
    /// Sends a message to the specified channel
    ///
    /// NOTE: Make sure to create a private DM channel if you want to DM someone!
    pub async fn send_message<
        F: FnOnce(&mut luro_model::response::InteractionResponse) -> &mut luro_model::response::InteractionResponse,
    >(
        &self,
        channel_id: &twilight_model::id::Id<twilight_model::id::marker::ChannelMarker>,
        response: F,
    ) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut r = luro_model::response::InteractionResponse::default();
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

        let response = sender.await?.model().await?;
        Ok(response.into())
    }
}
