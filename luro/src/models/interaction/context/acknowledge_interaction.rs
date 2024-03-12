use crate::models::interaction::InteractionResult;

impl super::InteractionContext {
    /// Acknowledge the interaction, showing a loading state. This will then be updated later.
    ///
    /// Use this for operations that take a long time. Generally its best to send this as soon as the interaction has been received.
    pub async fn ack_interaction(&mut self, ephemeral: bool) -> InteractionResult<()> {
        self.response.interaction_response_type = twilight_model::http::interaction::InteractionResponseType::DeferredChannelMessageWithSource;
        self.response.flags = if ephemeral {
            Some(twilight_model::channel::message::MessageFlags::EPHEMERAL)
        } else {
            None
        };

        let client = self.interaction_client();
        let request = self.response.interaction_response();

        client
            .create_response(self.interaction.id, &self.interaction.token, &request)
            .await?;

        Ok(())
    }
}
