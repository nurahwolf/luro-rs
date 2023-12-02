use luro_framework::Luro;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::RoleMarker, Id};

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Add a role to the blacklist")]
pub struct Add {
    /// The role to add
    role: Id<RoleMarker>,
}

impl luro_framework::LuroCommand for Add {
    async fn interaction_command(self, ctx: luro_framework::CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut owner_match = false;

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

        // Safe to unwrap as this command can only be executed in a guild
        ctx.database
            .driver
            .guild_new_blacklisted_role(ctx.guild_id().unwrap(), self.role)
            .await?;

        ctx.respond(|r| {
            r.content(format!("Added role <@&{}> to the guild blacklist!", self.role))
                .ephemeral()
        })
        .await
    }
}
