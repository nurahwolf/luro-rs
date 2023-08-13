

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;

use self::muzzle::MuzzleCommand;

mod muzzle;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "lewd",
    desc = "Whoa! How very lewd of you! These are more... Adult orientated commands."
)]
pub enum LewdCommands {
    #[command(name = "muzzle")]
    Muzzle(MuzzleCommand)
}


impl LuroCommand for LewdCommands {
    async fn run_commands(self, ctx: Slash) -> anyhow::Result<()> {
        let interaction_channel = ctx.channel()?;

        // TODO: Create a response type for this
        // TODO: Check for both
        if let Some(nsfw) = interaction_channel.nsfw && !nsfw {
            return ctx.nsfw_in_sfw_response().await
        }

        if interaction_channel.nsfw.is_none() {
            return ctx.nsfw_in_sfw_response().await;
        }

        // Call the appropriate subcommand.
        match self {
            Self::Muzzle(command) => command.run_command(ctx).await
        }
    }
}
