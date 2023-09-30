use luro_model::response::LuroResponse;
use twilight_model::{channel::message::MessageFlags, http::interaction::InteractionResponseType};

use crate::CommandInteraction;

impl<T> CommandInteraction<T> {
    /// Acknowledge the interaction, showing a loading state. This will then be updated later.
    ///
    /// Use this for operations that take a long time. Generally its best to send this as soon as the reaction has been received.
    pub async fn acknowledge_interaction(&self, ephemeral: bool) -> anyhow::Result<LuroResponse> {
        let response = LuroResponse {
            interaction_response_type: InteractionResponseType::DeferredChannelMessageWithSource,
            flags: if ephemeral { Some(MessageFlags::EPHEMERAL) } else { None },
            ..Default::default()
        };

        self.response_create(&response).await?;
        Ok(response)
    }
}
