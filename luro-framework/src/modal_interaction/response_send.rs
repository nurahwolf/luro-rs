use luro_model::{response::InteractionResponse, types::CommandResponse};

impl crate::ModalInteraction {
    /// Send an existing response builder a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    pub async fn response_send(&self, response: InteractionResponse) -> anyhow::Result<CommandResponse> {
        self.respond_message(|r| {
            *r = response;
            r
        })
        .await
    }
}
