use luro_model::database_driver::LuroDatabaseDriver;
use tracing::{error, warn};
use twilight_model::application::interaction::{InteractionData, InteractionType};

use super::LuroSlash;

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    /// A handler around different type of interactions
    /// TODO: Refactor this
    pub async fn handle(self) -> anyhow::Result<()> {
        let interaction = &self.interaction;

        let data = match interaction.data.clone() {
            Some(data) => data,
            None => {
                warn!(interaction = ?interaction, "Interaction without any data!");
                return Ok(());
            }
        };

        let (save, response) = match data {
            InteractionData::ApplicationCommand(data) => match &interaction.kind {
                InteractionType::ApplicationCommand => (true, self.clone().handle_command(data).await),
                InteractionType::ApplicationCommandAutocomplete => (true, self.clone().handle_autocomplete(data).await),
                _ => {
                    warn!(interaction = ?interaction, "Application Command with unexpected application data!");
                    (false, Ok(()))
                }
            },
            InteractionData::MessageComponent(data) => (false, self.clone().handle_component(data).await),
            InteractionData::ModalSubmit(data) => (false, self.clone().handle_modal(data).await),
            _ => {
                warn!(interaction = ?interaction, "Application Command with unexpected application data!");
                (false, Ok(()))
            }
        };

        match response {
            Ok(_) => {
                if let Ok(response) = self.interaction_client().response(&interaction.token).await && save {
                    self.framework
                        .database
                        .save_interaction(&response.model().await?.id.to_string(), interaction)
                        .await?;
                } else if save {
                    self.framework
                    .database
                    .save_interaction(&interaction.id.to_string(), interaction)
                    .await?;
                }
            }
            Err(why) => {
                error!(error = ?why, "error while processing interaction");
                // Attempt to send an error response
                if let Err(send_fail) = self.internal_error_response(why).await {
                    error!(error = ?send_fail, "Failed to respond to the interaction with an error response");
                };
            }
        };

        Ok(())
    }
}
