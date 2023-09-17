use luro_model::response::LuroResponse;
use twilight_model::http::interaction::InteractionResponseType;

use crate::ComponentInteraction;

impl<T> ComponentInteraction<T> {
    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn respond<F>(&self, response: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse,
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        match r.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource
            || r.interaction_response_type == InteractionResponseType::DeferredUpdateMessage
        {
            true => {
                self.response_update(&r).await?;
            }
            false => {
                self.response_create(&r).await?;
            }
        }

        Ok(())
    }
}
