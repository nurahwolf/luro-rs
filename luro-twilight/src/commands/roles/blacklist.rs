use luro_framework::{command::LuroCommand, Framework, InteractionCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

use luro_model::database::drivers::LuroDatabaseDriver;

use self::{add::Add, remove::Remove};

mod add;
mod remove;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "blacklist",
    desc = "Add or remove roles from the blacklist. Needs manage server permissons"
)]
pub enum Blacklist {
    #[command(name = "add")]
    Add(Add),
    #[command(name = "remove")]
    Remove(Remove)
}

impl LuroCommand for Blacklist {
    async fn interaction_command<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        match self {
            Self::Add(command) => command.interaction_command(ctx, interaction).await,
            Self::Remove(command) => command.interaction_command(ctx, interaction).await
        }
    }
}
