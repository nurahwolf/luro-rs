use anyhow::{Context, Error};

use twilight_interactions::command::{CommandModel, CreateCommand};

use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::Interaction;

use crate::framework::LuroFramework;
use crate::interactions::InteractionResponse;

use self::save::SaveCommand;

mod save;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "owner", desc = "Bot owner commands, for those with special privileges uwu!", dm_permission = false)]
pub enum OwnerCommands {
    #[command(name = "save")]
    Save(SaveCommand),

}

impl OwnerCommands {
    pub async fn run(
        interaction: &Interaction,
        ctx: &LuroFramework,
        data: CommandData,
    ) -> Result<InteractionResponse, Error> {
        // Parse the command data into a structure using twilight-interactions.
        let command =
            Self::from_interaction(data.into()).context("failed to parse command data")?;

        match command {
            Self::Save(data) => data.run(ctx, interaction).await,
        }
    }
}