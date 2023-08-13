

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "save_guilds",
    desc = "Save all guilds in the cache into configuration files, useful for updating global data"
)]
pub struct SaveGuildsCommand {}


impl LuroCommand for SaveGuildsCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        // let mut total = 0;

        // for guild_setting in &ctx.framework.database.g {
        //     guild_setting.flush_to_disk(guild_setting.key()).await?;
        //     total += 1;
        // }

        ctx.content(format!("Saved {} guilds to disk!", 0))
            .ephemeral()
            .respond()
            .await
    }
}
