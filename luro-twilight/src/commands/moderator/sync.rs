use async_trait::async_trait;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "sync", desc = "Sync the latest guild settings", dm_permission = false)]
pub struct Sync {}

#[async_trait]
impl LuroCommandTrait for Sync {
    async fn handle_interaction(
        ctx: Framework,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        // Can only run this command in a guild
        let guild_id = interaction.guild_id.unwrap();
        let mut luro_guild = ctx.database.get_guild(&guild_id).await?;
        let guild = ctx.twilight_client.guild(guild_id).await?.model().await?;
        luro_guild.update_guild(guild);

        interaction.respond(&ctx, |r| r.content("Updated!").ephemeral()).await
    }
}
