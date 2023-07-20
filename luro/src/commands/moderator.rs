use anyhow::Context;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::{application_command::CommandData, Interaction};

use crate::{interactions::InteractionResponse, LuroContext};

use self::{ban::BanCommand, kick::KickCommand, purge::PurgeCommand};

mod ban;
mod kick;
mod purge;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "mod", desc = "Commands that can be used by moderators", dm_permission = false)]
pub enum ModeratorCommands {
    #[command(name = "ban")]
    Ban(BanCommand),
    #[command(name = "kick")]
    Kick(KickCommand),
    #[command(name = "purge")]
    Purge(PurgeCommand)
}

impl ModeratorCommands {
    /// Handle incoming `/mod` commands.
    pub async fn run(interaction: &Interaction, ctx: &LuroContext, data: CommandData) -> anyhow::Result<InteractionResponse> {
        // Parse the command data into a structure using twilight-interactions.
        let command = ModeratorCommands::from_interaction(data.into()).context("failed to parse command data")?;

        // Call the appropriate subcommand.
        Ok(match command {
            Self::Ban(command) => command.run(ctx, interaction).await?,
            Self::Kick(command) => command.run(ctx, interaction).await?,
            Self::Purge(command) => command.run(ctx, interaction).await?
        })
    }
}
