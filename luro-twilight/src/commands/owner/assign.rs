use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, responses::Response, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::{marker::RoleMarker, Id};

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
    user: Option<ResolvedUser>,
    /// Set this to instead remove the role
    remove: Option<bool>,
}

#[async_trait]
impl LuroCommandTrait for Assign {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let interaction_user = interaction.author();

        // User to action
        let user = if let Some(user) = data.user {
            user.resolved
        } else {
            interaction_user.clone()
        };

        // Guild to modify
        let guild_id = match interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return Response::NotGuild().respond(&ctx, &interaction).await,
        };

        // If the user wants' to remove a role
        if let Some(remove) = data.remove && remove {
            match ctx
            .twilight_client
            .remove_guild_member_role(guild_id, user.id, data.role)
            .await {
                Ok(_) => interaction.respond(&ctx,|r|r.content(format!("Role <@&{}> removed from <@{}>!", data.role, user.id)).ephemeral()).await,
                Err(why) => Response::InternalError(why.into()).respond(&ctx, &interaction).await
            }
        } else {
        // Otherwise we just assign a role as expected
        match ctx
            .twilight_client
            .add_guild_member_role(guild_id, user.id, data.role)
            .await {
                Ok(_) => interaction.respond(&ctx, |r|r.content(format!("Role <@&{}> assigned to <@{}>!", data.role, user.id)).ephemeral()).await,
                Err(why) => Response::InternalError(why.into()).respond(&ctx, &interaction).await
            }
        }
    }
}
