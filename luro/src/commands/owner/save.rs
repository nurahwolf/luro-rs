use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::LuroSlash;

use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "save", desc = "Flush data to disk")]
pub struct SaveCommand {}

impl LuroCommand for SaveCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.respond(|r| r.content("Flushed data to disk!").ephemeral()).await
    }
}
