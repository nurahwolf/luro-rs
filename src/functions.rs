use anyhow::Error;
use twilight_model::application::interaction::{
    application_command::CommandData, Interaction, InteractionData,
};

pub async fn get_interaction_data(interaction: &Interaction) -> Result<Box<CommandData>, Error> {
    if let Some(InteractionData::ApplicationCommand(command_data)) = interaction.data.clone() {
        return Ok(command_data);
    }

    Err(Error::msg("No interaction data"))
}

pub mod get_guild_avatar;
