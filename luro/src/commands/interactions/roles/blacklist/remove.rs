use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::RoleMarker, Id};

#[derive(CommandModel, CreateCommand)]
#[command(name = "remove", desc = "Remove a role to the blacklist", dm_permission = false)]
pub struct Remove {
    /// The role to remove
    role: Id<RoleMarker>,
}

impl luro_framework::LuroCommand for Remove {
    async fn interaction_command(self, ctx: luro_framework::CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let guild_id = ctx.guild_id().unwrap(); // SAFETY: Safe to unwrap as this can only be run in a guild
        let mut owner_match = false;

        // We are using global data for this one in case an owner was removed from the application live

        for staff in ctx.database.user_fetch_staff().await? {
            if ctx.author.user_id == staff.user_id {
                owner_match = true
            }
        }

        if !owner_match {
            return ctx
                .simple_response(luro_model::response::SimpleResponse::PermissionNotBotStaff)
                .await;
        }

        let mut guild_settings = framework.database.get_guild(&guild_id).await?;
        guild_settings.assignable_role_blacklist.retain(|&x| x != data.role);
        framework.database.modify_guild(&guild_id, &guild_settings).await?;

        ctx
            .respond(|r| {
                r.content(format!("Added role <@&{}> to the guild blacklist!", self.role))
                    .ephemeral()
            })
            .await
    }
}
