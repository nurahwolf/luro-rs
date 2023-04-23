use anyhow::{bail, Error, Result};
use twilight_model::{application::interaction::{
    application_command::CommandData, message_component::MessageComponentInteractionData,
    Interaction, InteractionData, InteractionType,
}, gateway::payload::incoming::InteractionCreate};

use crate::{Luro, LuroError, State};

use self::{
    about::about_command,
    boop::{boop_button, boop_command},
    command_usage::command_usage,
    heck::heck_command,
    hello_world::hello_world,
    say::say_command,
};

pub mod about;
pub mod boop;
pub mod command_usage;
pub mod heck;
pub mod hello_world;
pub mod say;

/// Context to be passed through to interactions
pub struct LuroInteraction {
    pub interaction: Interaction,
    pub luro: State,
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

    pub async fn get_button_data(
        interaction: &Interaction,
    ) -> Result<MessageComponentInteractionData> {
        let interaction_data = match interaction.data.clone() {
            Some(ok) => ok,
            None => bail!(LuroError::NoInteractionData),
        };

        match interaction_data {
            InteractionData::MessageComponent(command_data) => Ok(command_data),
            _ => bail!(LuroError::NoMessageInteractionData),
        }
    }

    /// Handle an interaction request
    pub async fn handle_interaction(state: State, interaction: Box<InteractionCreate>) -> anyhow::Result<(), Error> {
        match interaction.kind {
            InteractionType::ApplicationCommand => {
                let data = Luro::get_interaction_data(&interaction).await?;

                match data.name.split_whitespace().next() {
                    Some("hello") => hello_world(state, &interaction).await,
                    Some("about") => about_command(state, &interaction).await,
                    Some("say") => say_command(state, &interaction).await,
                    Some("heck") => heck_command(state, &interaction).await,
                    Some("usage") => command_usage(state, &interaction).await,
                    Some("boop") => boop_command(state, &interaction).await,
                    _ => Ok(()),
                }
            }
            InteractionType::MessageComponent => {
                let data = Luro::get_button_data(&interaction).await?;

                match data.custom_id.as_str() {
                    "boop" => boop_button(state, &interaction).await,
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }
}
