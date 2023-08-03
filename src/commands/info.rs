use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{models::LuroSlash, traits::luro_command::LuroCommand};

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

#[async_trait]
impl LuroCommand for InfoCommands {
    async fn run_commands(self, ctx: LuroSlash) -> anyhow::Result<()> {
        match self {
            Self::User(command) => command.run_command(ctx).await,
            Self::Role(command) => command.run_command(ctx).await
        }
    }
}
