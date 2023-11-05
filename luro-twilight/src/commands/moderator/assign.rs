use luro_framework::{CommandInteraction, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::{marker::RoleMarker, Id};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "assign",
    desc = "Use the bot to assign a role to a user or self if not defined. You need permisison for this.",
    dm_permission = false
)]
pub struct Assign {
    /// The role that should be assigned. It HAS to be below the bot for this to work.
    role: Id<RoleMarker>,
    /// Optionally the user to apply the role to. Applies to self if not defined.
    user: Option<ResolvedUser>,
}

impl LuroCommand for Assign {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let user = ctx.get_specified_user_or_author(self.user.as_ref()).await?;
        ctx.twilight_client
            .add_guild_member_role(ctx.guild.as_ref().unwrap().guild_id, user.user_id, self.role)
            .await?;

        ctx.respond(|r| r.content(format!("Assigned the role <@&{}> successfully", self.role)).ephemeral())
            .await
    }
}
