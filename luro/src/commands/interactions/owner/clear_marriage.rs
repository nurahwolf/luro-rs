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

impl crate::models::CreateCommand for ClearMarriage {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        ctx.database
            .sqlx
            .delete_marriage((self.user_1.resolved.id.get() as i64, self.user_2.resolved.id.get() as i64))
            .await?;

        ctx.respond(|r| r.content("Looks like they are single now...").ephemeral()).await
    }
}
