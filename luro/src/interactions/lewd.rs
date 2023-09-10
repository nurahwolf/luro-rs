use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::LuroSlash;
use luro_model::database_driver::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;

use self::muzzle::MuzzleCommand;

mod muzzle;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "lewd",
    desc = "Whoa! How very lewd of you! These are more... Adult orientated commands."
)]
pub enum LewdCommands {
    #[command(name = "muzzle")]
    Muzzle(MuzzleCommand),
}

impl LuroCommand for LewdCommands {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        if !ctx.interaction.channel.as_ref().unwrap().nsfw.unwrap_or_default() {
            return ctx.nsfw_in_sfw_response().await;
        }

        // Call the appropriate subcommand.
        match self {
            Self::Muzzle(command) => command.run_command(ctx).await,
        }
    }
}
