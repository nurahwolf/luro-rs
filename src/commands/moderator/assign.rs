use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use twilight_model::id::{marker::RoleMarker, Id};

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "assign",
    desc = "Use the bot to assign a role to a user or self if not defined. You need permisison for this.",
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
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = slash.interaction.guild_id;
        let (author, _slash_author) = ctx.get_specified_user_or_author(&self.user, &slash)?;

        // Guild to modify
        let guild_id = match guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response(&mut slash).await
        };

        ctx.twilight_client
            .add_guild_member_role(guild_id, author.id, self.role)
            .await?;

        // TODO: Real response
        slash.content("All good!".to_owned());
        ctx.respond(&mut slash).await
    }
}
