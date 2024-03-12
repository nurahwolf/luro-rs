use twilight_model::http::interaction::InteractionResponseType;

use super::InteractionResponseBuilder;

impl InteractionResponseBuilder {
    /// Set the custom ID
    pub fn response_type(&mut self, response: InteractionResponseType) -> &mut Self {
        self.interaction_response_type = response;
        self
    }
}
