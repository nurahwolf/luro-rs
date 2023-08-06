use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use twilight_model::id::{marker::RoleMarker, Id};

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;
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
    user: Option<ResolvedUser>,
    /// Set this to instead remove the role
    remove: Option<bool>
}

#[async_trait]
impl LuroCommand for AssignCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let guild_id = slash.interaction.guild_id;

        let (user, _slash_user) = ctx.get_specified_user_or_author(&self.user, &slash)?;
        let user_id = user.id;

        // Guild to modify
        let guild_id = match guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response(&mut slash).await
        };

        // If the user wants' to remove a role
        if let Some(remove) = self.remove && remove {
            match ctx
            .twilight_client
            .remove_guild_member_role(guild_id, user.id, self.role)
            .await {
                Ok(_) => {slash.content(format!("Role <@&{}> removed from <@{}>!", self.role, &user_id)).ephemeral();ctx.respond(&mut slash).await},
                Err(why) => ctx.internal_error_response(why.to_string(), &mut slash).await
            }
        } else {
        // Otherwise we just assign a role as expected
        match ctx
            .twilight_client
            .add_guild_member_role(guild_id, user.id, self.role)
            .await {
                Ok(_) => {slash.content(format!("Role <@&{}> assigned to <@{}>!", self.role, &user_id)).ephemeral();ctx.respond(&mut slash).await},
                Err(why) => ctx.internal_error_response(why.to_string(), &mut slash).await
            }
        }
    }
}
