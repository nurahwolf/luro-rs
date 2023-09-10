use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

use self::add::Add;

mod add;
mod remove;

#[derive(CommandModel, CreateCommand,)]
#[command(name = "fetish", desc = "Add or remove some fetishes to your profile")]
pub enum Fetish {
    #[command(name = "add")]
    Add(Add,), // #[command(name = "remove")]
               // Remove(Remove),
}
impl Fetish {
    pub async fn run_command<D: LuroDatabaseDriver,>(self, ctx: LuroSlash<D,>,) -> anyhow::Result<(),> {
        match self {
            Self::Add(command,) => command.run_command(ctx,).await, // Self::Remove(command) => command.run_command(ctx).await,
        }
    }
}
