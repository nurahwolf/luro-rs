use crate::{database::drivers::LuroDatabaseDriver, response::LuroResponse};
use twilight_http::Error;
use twilight_model::{application::interaction::Interaction, http::interaction::InteractionResponseType};

use super::Context;

impl<D: LuroDatabaseDriver> Context<D> {
    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn respond<F>(&self, interaction: &Interaction, response: F) -> Result<(), Error>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        if r.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource
            || r.interaction_response_type == InteractionResponseType::DeferredUpdateMessage
        {
            self.update_response(&r, interaction).await?;
            return Ok(());
        }
        self.create_response(&r, interaction).await?;
        Ok(())
    }
}
