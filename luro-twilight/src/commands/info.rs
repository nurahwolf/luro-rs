use luro_framework::{
    {CreateLuroCommand, ExecuteLuroCommand},
    CommandInteraction,
};
use twilight_interactions::command::{CommandModel, CreateCommand};

mod database;
mod guild;
mod punishments;
mod role;
mod user;

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "info", desc = "Information about neat things")]
pub enum Info {
    #[command(name = "user")]
    User(user::InfoUser),
    #[command(name = "role")]
    Role(role::InfoRole),
    #[command(name = "guild")]
    Guild(guild::Guild),
    #[command(name = "punishments")]
    Punishments(punishments::Punishments),
    #[command(name = "database")]
    Database(database::Database),
}

impl CreateLuroCommand for Info {}

impl ExecuteLuroCommand for Info {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        match self {
            Self::Guild(command) => command.interaction_command(ctx).await,
            Self::Punishments(command) => command.interaction_command(ctx).await,
            Self::Role(command) => command.interaction_command(ctx).await,
            Self::User(command) => command.interaction_command(ctx).await,
            Self::Database(command) => command.interaction_command(ctx).await,
        }
    }
}
