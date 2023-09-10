use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::{marker::RoleMarker, Id};

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq,)]
#[command(
    name = "assign",
    desc = "Use the bot to assign a role to a user or self if not defined. Bypasses all other restrictions.",
    dm_permission = false
)]
pub struct AssignCommand {
    /// The role that should be assigned. It HAS to be below the bot for this to work.
    role: Id<RoleMarker,>,
    /// Optionally the user to apply the role to. Applies to self if not defined.
    user: Option<ResolvedUser,>,
    /// Set this to instead remove the role
    remove: Option<bool,>,
}

impl LuroCommand for AssignCommand {
    async fn run_command<D: LuroDatabaseDriver,>(self, ctx: LuroSlash<D,>,) -> anyhow::Result<(),> {
        let interaction_user = ctx.interaction.author().unwrap();

        // User to action
        let user = if let Some(user,) = self.user {
            user.resolved
        } else {
            interaction_user.clone()
        };

        // Guild to modify
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id,) => guild_id,
            None => return ctx.not_guild_response().await,
        };

        // If the user wants' to remove a role
        if let Some(remove) = self.remove && remove {
            match ctx
            .framework
            .twilight_client
            .remove_guild_member_role(guild_id, user.id, self.role)
            .await {
                Ok(_) => ctx.respond(|r|r.content(format!("Role <@&{}> removed from <@{}>!", self.role, user.id)).ephemeral()).await,
                Err(why) => ctx.internal_error_response(why.into()).await
            }
        } else {
        // Otherwise we just assign a role as expected
        match ctx
            .framework
            .twilight_client
            .add_guild_member_role(guild_id, user.id, self.role)
            .await {
                Ok(_) => ctx.respond(|r|r.content(format!("Role <@&{}> assigned to <@{}>!", self.role, user.id)).ephemeral()).await,
                Err(why) => ctx.internal_error_response(why.into()).await
            }
        }
    }
}
