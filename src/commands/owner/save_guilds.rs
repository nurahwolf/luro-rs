use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{models::GuildSetting, responses::LuroSlash};

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
        let guilds = ctx.luro.twilight_client.current_user_guilds().await?.model().await?;
        for guild in guilds {
            GuildSetting::manage_guild_settings(&ctx.luro, guild.id, None, true).await?;
            total += 1;
        }

        ctx.content(format!("Saved {total} guilds to disk!"))
            .ephemeral()
            .respond()
            .await
    }
}
