use anyhow::{Context, Error};

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::Interaction;

use crate::functions::interaction_context;
use crate::interactions::InteractionResponse;
use crate::responses::not_owner::not_owner_response;
use crate::LuroContext;

use self::log::LogCommand;
use self::save::SaveCommand;

mod log;
mod save;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "owner", desc = "Bot owner commands, for those with special privileges uwu!")]
pub enum OwnerCommands {
    #[command(name = "save")]
    Save(SaveCommand),
    #[command(name = "log")]
    Log(LogCommand)
}

impl OwnerCommands {
    pub async fn run(interaction: &Interaction, ctx: &LuroContext, data: CommandData) -> Result<InteractionResponse, Error> {
        let (_, interaction_author, _) = interaction_context(interaction, "owner command invoked")?;

        if !interaction_author.id.get() == 97003404601094144 {
            return Ok(not_owner_response());
        }

        // Parse the command data into a structure using twilight-interactions.
        let command = Self::from_interaction(data.into()).context("failed to parse command data")?;

        match command {
            Self::Save(data) => data.run(interaction, ctx).await,
            Self::Log(data) => data.run(interaction, ctx).await
        }
    }
}
