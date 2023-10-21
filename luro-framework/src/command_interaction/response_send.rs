use luro_model::response::LuroResponse;
use twilight_model::channel::Message;

use crate::CommandInteraction;

impl CommandInteraction {
    /// Send an existing response builder a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn response_send(&self, response: LuroResponse) -> anyhow::Result<Option<Message>> {
        self.respond_message(|r| {
            *r = response;
            r
        })
        .await
    }
}
