use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    interaction::{LuroSlash},
    luro_command::LuroCommand
};

use self::{role::InfoRole, user::InfoUser};

mod role;
mod user;

#[derive(CommandModel, CreateCommand)]
#[command(name = "info", desc = "Information about neat things")]
pub enum InfoCommands {
    #[command(name = "user")]
    User(InfoUser),
    #[command(name = "role")]
    Role(InfoRole)
}

impl LuroCommand for InfoCommands {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        match self {
            Self::User(command) => command.run_command(ctx).await,
            Self::Role(command) => command.run_command(ctx).await
        }
    }
}
