use crate::{builders::InteractionResponseBuilder, models::interaction::InteractionResult};

impl super::InteractionContext {
    /// Respond directly to an interaction.
    pub async fn respond<F>(&self, response: F) -> InteractionResult<()>
    where
        F: FnOnce(&mut InteractionResponseBuilder) -> &mut InteractionResponseBuilder,
    {
        let mut framework_response = self.response.clone();
        response(&mut framework_response);

        self.response_send(&framework_response).await
    }
}
