use luro_framework::{standard_response::Response, CommandInteraction, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::{marker::{RoleMarker, UserMarker}, Id};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "assign",
    desc = "Use the bot to assign a role to a user or self if not defined. Bypasses all other restrictions.",
    dm_permission = false
)]
pub struct Assign {
    /// The role that should be assigned. It HAS to be below the bot for this to work.
    role: Id<RoleMarker>,
    /// Optionally the user to apply the role to. Applies to self if not defined.
    user: Option<Id<UserMarker>>,
    /// Set this to instead remove the role
    remove: Option<bool>,
}

impl LuroCommand for Assign {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let user = ctx.get_specified_user_or_author(self.user).await?;
        let guild = match &ctx.guild {
            Some(guild) => guild,
            None => return ctx.response_simple(luro_framework::Response::NotGuild).await,
        };

        // If the user wants' to remove a role
        if let Some(remove) = self.remove && remove {
            match ctx
            .twilight_client
            .remove_guild_member_role(guild.guild_id, user.user_id, self.role)
            .await {
                Ok(_) => ctx.respond(|r|r.content(format!("Role <@&{}> removed from <@{}>!", self.role, user.user_id)).ephemeral()).await,
                Err(why) => ctx.response_simple(Response::InternalError(why.into())).await
            }
        } else {
        // Otherwise we just assign a role as expected
        match ctx
            .twilight_client
            .add_guild_member_role(guild.guild_id, user.user_id, self.role)
            .await {
                Ok(_) => ctx.respond(|r|r.content(format!("Role <@&{}> assigned to <@{}>!", self.role, user.user_id)).ephemeral()).await,
                Err(why) => ctx.response_simple(Response::InternalError(why.into())).await
            }
        }
    }
}
