use std::fmt::Write;

use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::LuroSlash;

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "guilds", desc = "Information about all guilds")]
pub struct OwnerGuildsCommand {
    /// Optionally include the guild ID
    show_id: Option<bool>
}

#[async_trait]
impl LuroCommand for OwnerGuildsCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut guilds = String::new();
        for guild in ctx.luro.twilight_cache.iter().guilds() {
            if let Some(show_id) = self.show_id && show_id {
                writeln!(guilds, "{} - <#{1}> - {1}", guild.name(), guild.id())?
            } else {
                writeln!(guilds, "{} - <#{}>", guild.name(), guild.id())?
            }
        }

        let embed = ctx
            .default_embed()
            .await?
            .title("All the guilds I am in!")
            .description(guilds);

        ctx.embed(embed.build())?.ephemeral().respond().await
    }
}
