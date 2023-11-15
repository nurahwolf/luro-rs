use luro_framework::{responses::Response, CommandInteraction, ExecuteLuroCommand, InteractionTrait};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "info", desc = "Information about the music player", dm_permission = false)]
pub struct InfoCommand {}

impl ExecuteLuroCommand for InfoCommand {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let guild_id = match ctx.guild_id {
            Some(guild_id) => guild_id,
            None => return ctx.response_simple(Response::NotGuild).await,
        };

        let mut description = String::new();

        let stats = ctx.lavalink.player(guild_id).await?.node().stats().await;
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

        let accent_colour = ctx.accent_colour();
        ctx.respond(|response| {
            response.embed(|embed| {
                embed
                    .description(description)
                    .create_field("Total Players", &stats.players.to_string(), true)
                    .create_field("Players playing Music", &stats.playing_players.to_string(), true)
                    .create_field("Uptime", &stats.uptime.to_string(), true)
                    .colour(accent_colour)
            })
        })
        .await
    }
}
