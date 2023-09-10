use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

use self::{add::Add, get::Get, list::List, remove::Remove, sort::Sort};

mod add;
mod get;
mod list;
mod remove;
mod sort;

#[derive(CommandModel, CreateCommand)]
#[command(name = "quote", desc = "Get or save some quotes")]
pub enum QuoteCommands {
    #[command(name = "get")]
    Get(Get),
    #[command(name = "add")]
    Add(Add),
    #[command(name = "list")]
    List(List),
    #[command(name = "sort")]
    Sort(Sort),
    #[command(name = "remove")]
    Remove(Remove),
}

impl LuroCommand for QuoteCommands {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        match self {
            Self::Get(command) => command.run_command(ctx).await,
            Self::Add(command) => command.run_command(ctx).await,
            Self::List(command) => command.run_command(ctx).await,
            Self::Sort(command) => command.run_command(ctx).await,
            Self::Remove(command) => command.run_command(ctx).await,
        }
    }
}
