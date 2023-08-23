use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::error;
use twilight_model::application::interaction::InteractionType;

use super::LuroSlash;

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    /// A handler around different type of interactions
    /// TODO: Refactor this
    pub async fn handle(self) -> anyhow::Result<()> {
        let interaction = &self.interaction;

        let response = match interaction.kind {
            InteractionType::ApplicationCommand => self.clone().handle_command().await,
            InteractionType::MessageComponent => self.clone().handle_component().await,
            InteractionType::ModalSubmit => self.clone().handle_modal().await,
            InteractionType::ApplicationCommandAutocomplete => self.clone().handle_autocomplete().await,
            _ => Ok(())
        };

        match response {
            Ok(_) => {
                if let Ok(response) = self.interaction_client().response(&interaction.token).await {
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

        // Update user
        if let Some(user_id) = self.interaction.author_id() {
            let mut user = self.framework.database.get_user(&user_id).await?;
            if let Ok(twilight_user) = self.framework.twilight_client.user(user_id).await {
                user.update_user(&twilight_user.model().await?);
            }
            self.framework.database.save_user(&user_id, &user).await?;
        }

        Ok(())
    }
}
