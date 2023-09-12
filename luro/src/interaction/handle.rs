use luro_model::database_driver::LuroDatabaseDriver;
use tracing::{error, warn};
use twilight_model::application::interaction::{InteractionData, InteractionType};

use super::LuroSlash;

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    /// A handler around different type of interactions
    /// TODO: Refactor this
    pub async fn handle(self) -> anyhow::Result<()> {
        let mut update = false;
        let interaction = &self.interaction;

        let data = match interaction.data.clone() {
            Some(data) => data,
            None => {
                warn!(interaction = ?interaction, "Interaction without any data!");
                return Ok(());
            }
        };

        let response = match data {
            InteractionData::ApplicationCommand(data) => match &interaction.kind {
                InteractionType::ApplicationCommand => {
                    update = true;
                    self.clone().handle_command(data).await
                },
                InteractionType::ApplicationCommandAutocomplete => self.clone().handle_autocomplete(data).await,
                _ => {
                    warn!(interaction = ?interaction, "Application Command with unexpected application data!");
                    Ok(())
                }
            },
            InteractionData::MessageComponent(data) => self.clone().handle_component(data).await,
            InteractionData::ModalSubmit(data) => self.clone().handle_modal(data).await,
            _ => todo!(),
        };

        match response {
            Ok(_) => {
                if let Ok(response) = self.interaction_client().response(&interaction.token).await && update {
                    self.framework
                        .database
                        .save_interaction(&response.model().await?.id.to_string(), interaction)
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
