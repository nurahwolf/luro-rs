use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

use self::{guild::Guild, role::InfoRole, user::InfoUser};

mod guild;
mod role;
mod user;

#[derive(CommandModel, CreateCommand)]
#[command(name = "info", desc = "Information about neat things")]
pub enum InfoCommands {
    #[command(name = "user")]
    User(InfoUser),
    #[command(name = "role")]
    Role(InfoRole),
    #[command(name = "guild")]
    Guild(Guild)
}

impl LuroCommand for InfoCommands {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        match self {
            Self::User(command) => command.run_command(ctx).await,
            Self::Role(command) => command.run_command(ctx).await,
            Self::Guild(command) => command.run_command(ctx).await
        }
    }
}
