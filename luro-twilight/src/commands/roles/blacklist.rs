use twilight_interactions::command::{CommandModel, CreateCommand};

mod add;
// mod remove;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "blacklist",
    desc = "Add or remove roles from the blacklist. Needs manage server permissons"
)]
pub enum Blacklist {
    #[command(name = "add")]
    Add(add::Add),
    // #[command(name = "remove")]
    // Remove(Remove),
}

impl luro_framework::LuroCommand for Blacklist {
    async fn interaction_command(self, ctx: luro_framework::CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        match self {
            Self::Add(cmd) => cmd.interaction_command(ctx).await,
            // Self::Remove(cmd) => cmd.interaction_command(ctx).await,
        }
    }
}
