use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::responses::LuroSlash;

use super::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "save_guilds",
    desc = "Save all guilds in the cache into configuration files, useful for updating global data"
)]
pub struct SaveGuildsCommand {}

#[async_trait]
impl LuroCommand for SaveGuildsCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut total = 0;
        let guild_data;

        {
            guild_data = ctx.luro.guild_data.read().clone();
        }

        for (guild_id, guild_setting) in guild_data.iter() {
            guild_setting.flush_to_disk(guild_id).await?;
            total += 1;
        }

        ctx.content(format!("Saved {total} guilds to disk!"))
            .ephemeral()
            .respond()
            .await
    }
}
