use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::{marker::RoleMarker, Id};

use crate::interaction::LuroSlash;

use crate::luro_command::LuroCommand;
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

impl LuroCommand for AssignCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let author = ctx.interaction.author_id().unwrap();

        // User to action
        let user = if let Some(user) = self.user {
            user.resolved.id
        } else {
            author
        };

        // Guild to modify
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response().await
        };

        ctx.framework
            .twilight_client
            .add_guild_member_role(guild_id, user, self.role)
            .await?;

        ctx.respond(|r| {
            r.content(format!("Assigned the role <@&{}> successfully", self.role))
                .ephemeral()
        })
        .await
    }
}
