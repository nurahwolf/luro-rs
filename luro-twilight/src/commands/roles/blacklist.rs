use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand};
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
#[async_trait::async_trait]

impl LuroCommandTrait for Blacklist {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        match data {
            Self::Add(_command) => add::Add::handle_interaction(ctx, interaction).await,
            Self::Remove(_command) => remove::Remove::handle_interaction(ctx, interaction).await
        }
    }
}
