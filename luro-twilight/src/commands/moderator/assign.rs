use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, responses::Response, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
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
#[async_trait]
impl LuroCommandTrait for Assign {
    async fn handle_interaction(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let author = interaction.author_id();

        // User to action
        let user = if let Some(user) = data.user {
            user.resolved.id
        } else {
            author
        };

        // Guild to modify
        let guild_id = match interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return Response::NotGuild().respond(&ctx, &interaction).await,
        };

        ctx.twilight_client.add_guild_member_role(guild_id, user, data.role).await?;

        interaction
            .respond(&ctx, |r| {
                r.content(format!("Assigned the role <@&{}> successfully", data.role))
                    .ephemeral()
            })
            .await
    }
}
