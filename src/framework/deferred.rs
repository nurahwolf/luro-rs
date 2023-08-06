use twilight_model::http::interaction::InteractionResponseType;

use crate::models::LuroResponse;

use super::LuroFramework;

impl LuroFramework {
    /// Set's the response type to be sent as a response to a deferred message and acknowledge this interaction.
    pub async fn deferred(&self, slash: &mut LuroResponse) -> anyhow::Result<&Self> {
        slash.interaction_response_type = InteractionResponseType::DeferredChannelMessageWithSource;
        self.interaction_client(slash)
            .create_response(slash.interaction.id, &slash.interaction.token, &slash.interaction_response())
            .await?;
        Ok(self)
    }
}
