use async_trait::async_trait;
use luro_framework::{
    command::{CreateLuroCommand, ExecuteLuroCommand},
    CommandInteraction,
};
use twilight_interactions::command::{CommandModel, CreateCommand};

use self::{guild::Guild, punishments::Punishments, role::InfoRole, user::InfoUser};

mod guild;
mod punishments;
mod role;
mod user;

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "info", desc = "Information about neat things")]
pub enum Info {
    #[command(name = "user")]
    User(InfoUser),
    #[command(name = "role")]
    Role(InfoRole),
    #[command(name = "guild")]
    Guild(Guild),
    #[command(name = "punishments")]
    Punishments(Punishments),
}

impl CreateLuroCommand for Info {}

#[async_trait]
impl ExecuteLuroCommand for Info {
    async fn interaction_command(&self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        match self {
            Self::Guild(command) => command.interaction_command(ctx).await,
            Self::Punishments(command) => command.interaction_command(ctx).await,
            Self::Role(command) => command.interaction_command(ctx).await,
            Self::User(command) => command.interaction_command(ctx).await,
        }
    }
}
