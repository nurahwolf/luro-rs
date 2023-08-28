use luro_framework::{command::LuroCommand, responses::SimpleResponse, Framework, InteractionCommand, LuroInteraction};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::RoleMarker, Id};

use luro_model::database::drivers::LuroDatabaseDriver;

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Add a role to the blacklist")]
pub struct Add {
    /// The role to add
    role: Id<RoleMarker>
}

impl LuroCommand for Add {
    async fn interaction_command<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let interaction_author = interaction.author_id();
        let mut owner_match = false;

        // We are using global data for this one in case an owner was removed from the application live

        for (id, _) in ctx.database.get_staff().await? {
            if interaction_author == id {
                owner_match = true
            }
        }

        if !owner_match {
            return SimpleResponse::PermissionNotBotStaff().respond(&ctx, &interaction).await;
        }

        let mut guild_settings = ctx.database.get_guild(&interaction.guild_id.unwrap()).await?;
        guild_settings.assignable_role_blacklist.push(self.role);
        ctx.database
            .save_guild(&interaction.guild_id.unwrap(), &guild_settings)
            .await?;

        interaction
            .respond(&ctx, |r| {
                r.content(format!("Added role <@&{}> to the guild blacklist!", self.role))
                    .ephemeral()
            })
            .await
    }
}
