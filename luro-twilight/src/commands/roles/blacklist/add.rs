use luro_framework::{command::LuroCommandTrait, responses::Response, Framework, InteractionCommand, LuroInteraction};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::RoleMarker, Id};

use luro_model::database_driver::LuroDatabaseDriver;

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Add a role to the blacklist")]
pub struct Add {
    /// The role to add
    role: Id<RoleMarker>,
}
#[async_trait::async_trait]

impl LuroCommandTrait for Add {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let interaction_author = interaction.author_id();
        let mut owner_match = false;

        // We are using global data for this one in case an owner was removed from the application live

        for (id, _) in ctx.database.get_staff().await? {
            if interaction_author == id {
                owner_match = true
            }
        }

        if !owner_match {
            return Response::PermissionNotBotStaff().respond(&ctx, &interaction).await;
        }

        let mut guild_settings = ctx.database.get_guild(&interaction.guild_id.unwrap()).await?;
        guild_settings.assignable_role_blacklist.push(data.role);
        ctx.database
            .modify_guild(&interaction.guild_id.unwrap(), &guild_settings)
            .await?;

        interaction
            .respond(&ctx, |r| {
                r.content(format!("Added role <@&{}> to the guild blacklist!", data.role))
                    .ephemeral()
            })
            .await
    }
}
