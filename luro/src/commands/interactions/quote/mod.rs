use luro_framework::{CommandInteraction, CreateLuroCommand, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

mod add;
mod get;
mod list;
// mod remove;

#[derive(CommandModel, CreateCommand)]
#[command(name = "quote", desc = "Get or save some quotes")]
pub enum Quote {
    #[command(name = "get")]
    Get(get::Get),
    #[command(name = "add")]
    Add(add::Add),
    #[command(name = "list")]
    List(list::List),
    // #[command(name = "remove")]
    // Remove(remove::Remove),
}

impl CreateLuroCommand for Quote {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        match self {
            Self::Get(cmd) => cmd.interaction_command(ctx).await,
            Self::Add(cmd) => cmd.interaction_command(ctx).await,
            Self::List(cmd) => cmd.interaction_command(ctx).await,
            // Self::Remove(cmd) => cmd.interaction_command(ctx).await,
        }
    }
}
