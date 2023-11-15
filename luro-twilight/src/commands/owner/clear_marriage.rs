use luro_framework::{CommandInteraction, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "clear_marriage", desc = "Clears a marriage")]
pub struct ClearMarriage {
    /// First user to clear
    pub user_1: ResolvedUser,
    /// Second user to clear
    pub user_2: ResolvedUser,
}

impl LuroCommand for ClearMarriage {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        ctx.database
            .driver
            .delete_marriage((self.user_1.resolved.id.get() as i64, self.user_2.resolved.id.get() as i64))
            .await?;

        ctx.respond(|r| r.content("Looks like they are single now...").ephemeral()).await
    }
}
