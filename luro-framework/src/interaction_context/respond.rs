use luro_builder::response::LuroResponse;
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_model::http::interaction::InteractionResponseType::DeferredChannelMessageWithSource;
use twilight_model::http::interaction::InteractionResponseType::DeferredUpdateMessage;

use crate::Framework;
use crate::InteractionContext;
impl InteractionContext {
    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn respond<D, F>(&self, framework: Framework<D>, response: F) -> anyhow::Result<()>
    where
        D: LuroDatabaseDriver,
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        match r.interaction_response_type == DeferredChannelMessageWithSource
            || r.interaction_response_type == DeferredUpdateMessage
        {
            true => self.response_update(framework, &r).await.map(|_| ()),
            false => self.response_create(framework, &r).await
        }
    }
}
