use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};
use luro_model::database::drivers::LuroDatabaseDriver;

use self::{add::Add, remove::Remove};

mod add;
mod remove;

#[derive(CommandModel, CreateCommand,)]
#[command(
    name = "blacklist",
    desc = "Add or remove roles from the blacklist. Needs manage server permissons"
)]
pub enum Blacklist {
    #[command(name = "add")]
    Add(Add,),
    #[command(name = "remove")]
    Remove(Remove,),
}

impl LuroCommand for Blacklist {
    async fn run_command<D: LuroDatabaseDriver,>(self, ctx: LuroSlash<D,>,) -> anyhow::Result<(),> {
        match self {
            Self::Add(command,) => command.run_command(ctx,).await,
            Self::Remove(command,) => command.run_command(ctx,).await,
        }
    }
}
