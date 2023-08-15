use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

use self::{get::Get, add::Add};

mod add;
mod get;

#[derive(CommandModel, CreateCommand)]
#[command(name = "quote", desc = "Get or save some quotes")]
pub enum QuoteCommands {
    #[command(name = "get")]
    Get(Get),
    #[command(name = "add")]
    Add(Add)
}

impl LuroCommand for QuoteCommands {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        match self {
            Self::Get(command) => command.run_command(ctx).await,
            Self::Add(command) => command.run_command(ctx).await
        }
    }
}