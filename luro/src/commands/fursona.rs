use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser, AutocompleteValue};

use crate::slash::LuroSlash;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "fursona",
    desc = "Get your own or other fursonas!"
)]
pub enum LewdCommands {
    #[command(name = "nsfw")]
    NSFW(FursonaNSFWCommand)
}

#[async_trait]
impl LuroCommand for LewdCommands {
    async fn run_commands(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let interaction_channel = ctx.channel()?;

        if let Some(nsfw) = interaction_channel.nsfw && !nsfw {
            return ctx.nsfw_in_sfw_response().await
        }


        // Call the appropriate subcommand.
        match self {
            Self::NSFW(command) => command.run_command(ctx).await
        }
    }
}


use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel)]
#[command(autocomplete = true, name = "nsfw", desc = "Make me say garbage!")]
pub struct FursonaNSFWCommand {
    /// The fursona to get
    name: AutocompleteValue<String>,
}

#[async_trait]
impl LuroCommand for FursonaNSFWCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let content = if let Some(user) = self.user {
            format!("Hey <@{}>!\n{}", user.resolved.id, self.message)
        } else {
            self.message
        };

        ctx.content(content).respond().await
    }
}
