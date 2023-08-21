use std::fmt::Write;
use std::time::Instant;

use luro_builder::embed::EmbedBuilder;
use luro_builder::response::LuroResponse;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use crate::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "ping", desc = "See my ping!")]
pub struct PingCommand {}

impl LuroCommand for PingCommand {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let mut embed = EmbedBuilder::default();
        embed.colour(ctx.accent_colour().await).description("🏓 Pinging!");
        if let Some(average) = ctx.latency.average() {
            embed.create_field(
                "Average Latency",
                &format!("`{}` milliseconds", average.as_millis().to_string()),
                true
            );
        }

        if let Some(average) = ctx.latency.received() {
            embed.create_field(
                "Last Acknowledgement",
                &format!("{} milliseconds ago", average.elapsed().as_millis()),
                true
            );
        }

        if let Some(average) = ctx.latency.sent() {
            embed.create_field(
                "Hearbeat Sent",
                &format!("{} milliseconds ago", average.elapsed().as_millis()),
                true
            );
        }

        let mut num = 0;
        let mut heartbeats = String::new();
        for heartbeat in ctx.latency.recent() {
            num += 1;
            writeln!(heartbeats, "{num} - {} milliseconds", heartbeat.as_millis())?
        }
        if !heartbeats.is_empty() {
            embed.create_field(
                "Hearbeats",
                &format!(
                    "**Total Heartbeats Recorded:** `{}`\n```{heartbeats}```",
                    ctx.latency.periods().to_string()
                ),
                false
            );
        }

        let start = Instant::now();
        let mut response = LuroResponse::default();
        response.add_embed(embed);
        ctx.create_response(&response).await?;
        let sent = format!(
            "Pong!\n`Send MESSAGE` API request achnowledged and received in `{}` milliseconds!",
            start.elapsed().as_millis()
        );
        response.content(sent.clone());

        ctx.update_response(&response).await?;

        // A random command to check latency time
        if let Some(author) = ctx.interaction.author() {
            let start = Instant::now();
            let _ = ctx.framework.twilight_client.user(author.id).await?.model().await?;
            let user = format!(
                "{}\n`Get USER` API request achnowledged and received in `{}` milliseconds!",
                sent,
                start.elapsed().as_millis()
            );
            response.content(user);
            ctx.update_response(&response).await?;
        }

        Ok(())
    }
}
