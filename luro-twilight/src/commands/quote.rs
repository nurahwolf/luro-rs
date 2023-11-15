use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, CommandInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
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

#[async_trait]
impl LuroCommandTrait for QuoteCommands {
    async fn handle_interaction(
        ctx: CommandInteraction<Self>,
    ) -> anyhow::Result<luro_model::types::CommandResponse> {
        match ctx.command {
            Self::Get(_) => add::Add::handle_interaction(ctx).await,
            Self::Add(_) => get::Get::handle_interaction(ctx).await,
            Self::List(_) => list::List::handle_interaction(ctx).await,
            Self::Sort(_) => remove::Remove::handle_interaction(ctx).await,
            Self::Remove(_) => sort::Sort::handle_interaction(ctx).await,
        }
    }
}
