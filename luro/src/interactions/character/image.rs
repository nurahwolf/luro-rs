use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

mod add;
mod get;

#[derive(CommandModel, CreateCommand)]
#[command(name = "image", desc = "Images relating to a character")]
pub enum Image {
    #[command(name = "add")]
    Add(add::Add),
    #[command(name = "get")]
    Get(get::Get),
}
impl Image {
    pub async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        match self {
            Self::Add(command) => command.run_command(ctx).await,
            Self::Get(command) => command.run_command(ctx).await,
        }
    }
}