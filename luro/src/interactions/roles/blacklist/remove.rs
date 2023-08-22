use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::RoleMarker, Id};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};
use luro_model::database::drivers::LuroDatabaseDriver;

#[derive(CommandModel, CreateCommand)]
#[command(name = "remove", desc = "Remove a role to the blacklist")]
pub struct Remove {
    /// The role to remove
    role: Id<RoleMarker>
}

impl LuroCommand for Remove {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let interaction_author = ctx.interaction.author_id().unwrap();
        let mut owner_match = false;

        // We are using global data for this one in case an owner was removed from the application live

        for (id, _) in ctx.framework.database.get_staff().await? {
            if interaction_author == id {
                owner_match = true
            }
        }

        if !owner_match {
            return ctx
                .not_owner_response(&interaction_author, &ctx.interaction.guild_id, "role-menu")
                .await;
        }

        let mut guild_settings = ctx.framework.database.get_guild(&ctx.interaction.guild_id.unwrap(), &ctx.framework.twilight_client).await?;
        guild_settings.assignable_role_blacklist.retain(|&x| x != self.role);
        ctx.framework
            .database
            .save_guild(&ctx.interaction.guild_id.unwrap(), &guild_settings)
            .await?;

        ctx.respond(|r| {
            r.content(format!("Added role <@&{}> to the guild blacklist!", self.role))
                .ephemeral()
        })
        .await
    }
}
