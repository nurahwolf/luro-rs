use async_trait::async_trait;
use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::models::LuroSlash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "info", desc = "Information about the music player", dm_permission = false)]
pub struct InfoCommand {}

#[async_trait]
impl LuroCommand for InfoCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.not_guild_response().await
        };

        let mut description = String::new();

        let stats = ctx.luro.lavalink.player(guild_id).await?.node().stats().await;
        writeln!(
            description,
            "**Consumption:** `{}` cores assigned - `{:.2}` lavalink load - `{:.2}` system load",
            stats.cpu.cores, stats.cpu.lavalink_load, stats.cpu.system_load
        )?;
        writeln!(
            description,
            "**Memory:** `{}MB` allocated - `{}MB` used - `{}MB` reservable - `{}MB` free",
            stats.memory.allocated / 1024 / 1024,
            stats.memory.used / 1024 / 1024,
            stats.memory.reservable / 1024 / 1024,
            stats.memory.free / 1024 / 1024
        )?;

        let embed = ctx
            .default_embed()
            .await?
            .title("Lavalink Music Stats")
            .description(description)
            .field(EmbedFieldBuilder::new("Total Players", stats.players.to_string()).inline())
            .field(EmbedFieldBuilder::new("Playing Players", stats.playing_players.to_string()).inline())
            .field(EmbedFieldBuilder::new("Uptime", stats.uptime.to_string()).inline());
        ctx.embed(embed.build())?.respond().await
    }
}
