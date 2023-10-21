use luro_framework::{
    CommandInteraction, {CreateLuroCommand, LuroCommand},
};
use twilight_interactions::command::{CommandModel, CreateCommand};

mod add;
mod random;

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "images", desc = "Get some images!")]
pub enum Images {
    #[command(name = "add")]
    Add(add::Add),
    #[command(name = "random")]
    Random(random::Random),
}

impl CreateLuroCommand for Images {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        match self {
            Self::Add(command) => command.interaction_command(ctx).await,
            Self::Random(command) => command.interaction_command(ctx).await,
        }
    }
}
