use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::{marker::RoleMarker, Id};

use crate::responses::LuroSlash;

use super::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "assign",
    desc = "Use the bot to assign a role to a user or self if not defined. Bypasses all other restrictions.",
    dm_permission = false
)]
pub struct AssignCommand {
    /// The role that should be assigned. It HAS to be below the bot for this to work.
    role: Id<RoleMarker>,
    /// Optionally the user to apply the role to. Applies to self if not defined.
    user: Option<ResolvedUser>
}

#[async_trait]
impl LuroCommand for AssignCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let interaction_user = ctx.author()?;

        // User to action
        let user = if let Some(user) = self.user {
            user.resolved
        } else {
            interaction_user
        };

        // Guild to modify
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response().await
        };

        let _response = ctx
            .luro
            .twilight_client
            .add_guild_member_role(guild_id, user.id, self.role)
            .await?
            .status();

        // TODO: A success message
        ctx.content("All good!".to_owned()).respond().await
    }
}
