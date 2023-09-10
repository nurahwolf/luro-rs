use async_trait::async_trait;
use luro_framework::{
    command::{LuroCommandBuilder, LuroCommandTrait},
    Framework, InteractionCommand,
};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

mod add;
mod get;
mod list;
mod remove;
mod sort;

#[derive(CommandModel, CreateCommand)]
#[command(name = "quote", desc = "Get or save some quotes")]
pub enum QuoteCommands {
    #[command(name = "get")]
    Get(get::Get),
    #[command(name = "add")]
    Add(add::Add),
    #[command(name = "list")]
    List(list::List),
    #[command(name = "sort")]
    Sort(sort::Sort),
    #[command(name = "remove")]
    Remove(remove::Remove),
}

impl<D: LuroDatabaseDriver + 'static> LuroCommandBuilder<D> for QuoteCommands {}

#[async_trait]
impl LuroCommandTrait for QuoteCommands {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;

        match data {
            Self::Get(_) => add::Add::handle_interaction(ctx, interaction).await,
            Self::Add(_) => get::Get::handle_interaction(ctx, interaction).await,
            Self::List(_) => list::List::handle_interaction(ctx, interaction).await,
            Self::Sort(_) => remove::Remove::handle_interaction(ctx, interaction).await,
            Self::Remove(_) => sort::Sort::handle_interaction(ctx, interaction).await,
        }
    }
}
