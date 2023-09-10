use crate::interaction::LuroSlash;
use luro_model::database_driver::LuroDatabaseDriver;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "sync", desc = "Sync the latest guild settings", dm_permission = false)]
pub struct SyncCommand {}

impl LuroCommand for SyncCommand {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        // Can only run this command in a guild
        let guild_id = ctx.interaction.guild_id.unwrap();
        let mut luro_guild = ctx.framework.database.get_guild(&guild_id).await?;
        let guild = ctx.framework.twilight_client.guild(guild_id).await?.model().await?;
        luro_guild.update_guild(guild);

        ctx.respond(|r| r.content("Updated!").ephemeral()).await
    }
}
