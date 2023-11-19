use luro_model::{response::InteractionResponse, types::CommandResponse};
use twilight_model::http::interaction::InteractionResponseType;

impl crate::ComponentInteraction {
    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    /// This method returns an optional message, if the message was updated
    pub async fn respond_message<F>(&self, response: F) -> anyhow::Result<CommandResponse>
    where
        F: FnOnce(&mut InteractionResponse) -> &mut InteractionResponse,
    {
        let mut r = InteractionResponse::default();
        response(&mut r);

        match r.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource
            || r.interaction_response_type == InteractionResponseType::DeferredUpdateMessage
        {
            true => self.response_update(&r).await,
            false => self.response_create(&r).await,
        }
    }
}
