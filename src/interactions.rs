use crate::{interactions::hello_world::hello_world, luro::LuroError};
use anyhow::{bail, Error, Result};
use twilight_model::application::interaction::{
    application_command::CommandData, Interaction, InteractionData, InteractionType,
};

use crate::Luro;

use self::{about::about_command, say::say_command};

pub mod about;
pub mod hello_world;
pub mod say;

/// Context to be passed through to interactions
pub struct LuroInteraction {
    pub interaction: Interaction,
    pub luro: Luro,
}

impl Luro {
    /// Try to get command data from an interaction
    pub async fn get_interaction_data(interaction: &Interaction) -> Result<Box<CommandData>> {
        let interaction_data = match interaction.data.clone() {
            Some(ok) => ok,
            None => bail!(LuroError::NoInteractionData),
        };

        match interaction_data {
            InteractionData::ApplicationCommand(command_data) => Ok(command_data),
            _ => bail!(LuroError::NoApplicationCommand),
        }
    }

    /// Handle an interaction request
    pub async fn handle_interaction(&self, interaction: Interaction) -> anyhow::Result<(), Error> {
        match interaction.kind {
            InteractionType::ApplicationCommand => {
                let data = Luro::get_interaction_data(&interaction).await?;

                match data.name.split_whitespace().next() {
                    Some("hello") => hello_world(self, &interaction).await,
                    Some("about") => about_command(self, &interaction).await,
                    Some("say") => say_command(self, &interaction).await,
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }
}
