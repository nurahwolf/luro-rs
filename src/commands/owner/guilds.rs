use std::fmt::Write;

use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::LuroContext;

use crate::models::LuroResponse;
use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "guilds", desc = "Information about all guilds")]
pub struct OwnerGuildsCommand {
    /// Optionally include the guild ID
    show_id: Option<bool>
}

#[async_trait]
impl LuroCommand for OwnerGuildsCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let mut guilds = String::new();
        for guild in ctx.twilight_cache.iter().guilds() {
            if let Some(show_id) = self.show_id && show_id {
                writeln!(guilds, "{} - <#{1}> - {1}", guild.name(), guild.id())?
            } else {
                writeln!(guilds, "{} - <#{}>", guild.name(), guild.id())?
            }
        }

        let embed = ctx
            .default_embed(&slash.interaction.guild_id)
            .title("All the guilds I am in!")
            .description(guilds);

        slash.embed(embed.build())?.ephemeral();
        ctx.respond(&mut slash).await
    }
}
