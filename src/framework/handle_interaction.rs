use std::sync::Arc;

use tracing::{error, warn};
use twilight_gateway::MessageSender;
use twilight_model::{application::interaction::InteractionType, gateway::payload::incoming::InteractionCreate};

use crate::models::LuroResponse;

use super::LuroFramework;

impl LuroFramework {
    // Handle an interaction
    pub async fn handle_interaction(
        self: Arc<Self>,
        shard: MessageSender,
        event: Box<InteractionCreate>
    ) -> anyhow::Result<()> {
        let mut slash = LuroResponse::new(event.0, shard);
        let callback = match slash.interaction.kind {
            InteractionType::ApplicationCommand => self.handle_command(slash.clone()).await,
            InteractionType::MessageComponent => self.handle_component(slash.clone()).await,
            InteractionType::ModalSubmit => self.clone().handle_modal(slash.clone()).await,
            other => {
                warn!("received unexpected {} interaction", other.kind());
                Ok(())
            }
        };

        if let Err(why) = callback {
            error!(error = ?why, "error while processing interaction");

            // Attempt to send an error response
            if let Err(send_fail) = self.internal_error_response(why.to_string(), &mut slash).await {
                error!(error = ?send_fail, "Failed to respond to the interaction with an error response");
            };
        };

        Ok(())
    }
}
