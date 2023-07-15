use anyhow::Context;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::{application_command::CommandData, Interaction};

use crate::{framework::LuroFramework, interactions::InteractionResponse};

use self::{ban::BanCommand, kick::KickCommand};

mod ban;
mod kick;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "mod",
    desc = "Commands that can be used by moderators",
    dm_permission = false
)]
pub enum ModeratorCommands {
    #[command(name = "ban")]
    Ban(BanCommand),
    #[command(name = "kick")]
    Kick(KickCommand),
}

impl ModeratorCommands {
    /// Handle incoming `/mod` commands.
    pub async fn run(
        interaction: &Interaction,
        ctx: &LuroFramework,
        data: CommandData,
    ) -> anyhow::Result<InteractionResponse> {
        // Parse the command data into a structure using twilight-interactions.
        let command = ModeratorCommands::from_interaction(data.into())
            .context("failed to parse command data")?;

        // Call the appropriate subcommand.
        Ok(match command {
            ModeratorCommands::Ban(command) => command.run(ctx, interaction).await?,
            ModeratorCommands::Kick(command) => command.run(ctx, interaction).await?,
        })
    }
}
