use anyhow::anyhow;
use std::mem;

use twilight_model::application::interaction::{
    message_component::MessageComponentInteractionData, modal::ModalInteractionData, Interaction, InteractionData
};

use super::LuroSlash;

impl LuroSlash {
    /// Parse incoming [`ModalSubmit`] interaction and return the inner data.
    ///
    /// This takes a mutable [`Interaction`] since the inner [`ModalInteractionData`]
    /// is replaced with [`None`] to avoid useless clones.
    ///
    /// [`ModalSubmit`]: twilight_model::application::interaction::InteractionType::ModalSubmit
    /// [`ModalInteractionData`]: twilight_model::application::interaction::modal::ModalInteractionData
    pub fn parse_modal_data(&self, interaction: &mut Interaction) -> anyhow::Result<ModalInteractionData> {
        match mem::take(&mut interaction.data) {
            Some(InteractionData::ModalSubmit(data)) => Ok(data),
            _ => Err(anyhow!("unable to parse modal data, received unexpected data type"))
        }
    }

    pub fn parse_component_data(&self, interaction: &mut Interaction) -> anyhow::Result<Box<MessageComponentInteractionData>> {
        match mem::take(&mut interaction.data) {
            Some(InteractionData::MessageComponent(data)) => Ok(data),
            _ => Err(anyhow!("unable to parse component data, received unexpected data type"))
        }
    }
}