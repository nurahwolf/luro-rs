use luro_framework::{command::LuroCommandTrait, responses::SimpleResponse, Framework, InteractionCommand, LuroInteraction};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::RoleMarker, Id};

use luro_model::database_driver::LuroDatabaseDriver;

#[derive(CommandModel, CreateCommand)]
#[command(name = "remove", desc = "Remove a role to the blacklist", dm_permission = false)]
pub struct Remove {
    /// The role to remove
    role: Id<RoleMarker>,
}
#[async_trait::async_trait]

impl LuroCommandTrait for Remove {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        framework: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;

        let guild_id = interaction.guild_id.unwrap(); // SAFETY: Safe to unwrap as this can only be run in a guild
        let interaction_author = interaction.author_id();
        let mut owner_match = false;

        // We are using global data for this one in case an owner was removed from the application live

        for (id, _) in framework.database.get_staff().await? {
            if interaction_author == id {
                owner_match = true
            }
        }

        if !owner_match {
            return SimpleResponse::PermissionNotBotStaff()
                .respond(&framework, &interaction)
                .await;
        }

        let mut guild_settings = framework.database.get_guild(&guild_id).await?;
        guild_settings.assignable_role_blacklist.retain(|&x| x != data.role);
        framework.database.save_guild(&guild_id, &guild_settings).await?;

        interaction
            .respond(&framework, |r| {
                r.content(format!("Added role <@&{}> to the guild blacklist!", data.role))
                    .ephemeral()
            })
            .await
    }
}
