use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq,)]
#[command(name = "info", desc = "Information about the music player", dm_permission = false)]
pub struct InfoCommand {}

impl LuroCommand for InfoCommand {
    async fn run_command<D: LuroDatabaseDriver,>(self, ctx: LuroSlash<D,>,) -> anyhow::Result<(),> {
        let guild_id = match ctx.interaction.guild_id {
            Some(guild_id,) => guild_id,
            None => return ctx.not_guild_response().await,
        };

        let mut description = String::new();

        let stats = ctx.framework.lavalink.player(guild_id,).await?.node().stats().await;
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

        let accent_colour = ctx.accent_colour().await;
        ctx.respond(|response| {
            response.embed(|embed| {
                embed
                    .description(description,)
                    .create_field("Total Players", &stats.players.to_string(), true,)
                    .create_field("Players playing Music", &stats.playing_players.to_string(), true,)
                    .create_field("Uptime", &stats.uptime.to_string(), true,)
                    .colour(accent_colour,)
            },)
        },)
            .await
    }
}
