use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

use self::{guild::Guild, punishments::Punishments, role::InfoRole, user::InfoUser};

mod guild;
mod punishments;
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
    Guild(Guild),
    #[command(name = "punishments")]
    Punishments(Punishments)
}

impl LuroCommand for InfoCommands {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        match self {
            Self::Guild(command) => command.run_command(ctx).await,
            Self::Punishments(command) => command.run_command(ctx).await,
            Self::Role(command) => command.run_command(ctx).await,
            Self::User(command) => command.run_command(ctx).await
        }
    }
}
