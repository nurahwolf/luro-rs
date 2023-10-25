use twilight_model::http::interaction::InteractionResponseType;

use super::LuroResponse;

impl LuroResponse {
    /// Sets the resposne as dererred
    pub fn deferred(&mut self) -> &mut Self {
        self.interaction_response_type = InteractionResponseType::DeferredChannelMessageWithSource;
        self
    }
}
