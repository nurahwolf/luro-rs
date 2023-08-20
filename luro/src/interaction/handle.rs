use tracing::error;
use twilight_model::application::interaction::InteractionType;

use super::LuroSlash;

impl LuroSlash {
    /// A handler around different type of interactions
    /// TODO: Refactor this
    pub async fn handle(self) -> anyhow::Result<()> {
        let interaction = &self.interaction;
        let response = match interaction.kind {
            InteractionType::ApplicationCommand => {
                // Attempt to get the original message to save it to our cache
                self.clone().handle_command().await?;
                if let Ok(response) = self.interaction_client().response(&interaction.token).await {
                    self.framework
                        .database
                        .save_interaction(&response.model().await?.id.to_string(), interaction)
                        .await?;
                };
                Ok(())
            }
            InteractionType::MessageComponent => self.clone().handle_component().await,
            InteractionType::ModalSubmit => self.clone().handle_modal().await,
            InteractionType::ApplicationCommandAutocomplete => self.clone().handle_autocomplete().await,
            _ => todo!() // other => {
                         //     warn!("received unexpected {} interaction", other.kind());
                         //     Ok(())
                         // }
        };

        if let Err(why) = response {
            error!(error = ?why, "error while processing interaction");
            // Attempt to send an error response
            if let Err(send_fail) = self.internal_error_response(why).await {
                error!(error = ?send_fail, "Failed to respond to the interaction with an error response");
            };
        };

        Ok(())
    }
}
