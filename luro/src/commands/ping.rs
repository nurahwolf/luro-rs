use std::fmt::Write;
use std::time::{Instant, SystemTime, UNIX_EPOCH};



use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "ping", desc = "See my ping!")]
pub struct PingCommand {}


impl LuroCommand for PingCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let mut embed = ctx.default_embed().await?.description("🏓 Pinging!");
        if let Some(average) = ctx.latency.average() {
            embed = embed.field(
                EmbedFieldBuilder::new(
                    "Average Latency",
                    format!("`{}` milliseconds", average.as_millis().to_string())
                )
                .inline()
            )
        }

        if let Some(average) = ctx.latency.received() {
            embed = embed.field(
                EmbedFieldBuilder::new(
                    "Last Acknowledgement",
                    format!("{} milliseconds ago", average.elapsed().as_millis(),)
                )
                .inline()
            )
        }

        if let Some(average) = ctx.latency.sent() {
            embed = embed.field(
                EmbedFieldBuilder::new(
                    "Heartbeat Sent",
                    format!("{} milliseconds ago", average.elapsed().as_millis(),)
                )
                .inline()
            )
        }

        let mut num = 0;
        let mut heartbeats = String::new();
        for heartbeat in ctx.latency.recent() {
            num += 1;
            writeln!(heartbeats, "{num} - {} milliseconds", heartbeat.as_millis())?
        }
        if !heartbeats.is_empty() {
            embed = embed.field(EmbedFieldBuilder::new(
                "Heartbeats",
                format!(
                    "**Total Heartbeats Recorded:** `{}`\n```{heartbeats}```",
                    ctx.latency.periods().to_string()
                )
            ))
        }

        let start = Instant::now();
        ctx.embed(embed.build())?.respond().await?;
        let sent = format!(
            "Pong!\n`Send MESSAGE` API request achnowledged and received in `{}` milliseconds!",
            start.elapsed().as_millis()
        );

        ctx.set_deferred().content(sent.clone()).respond().await?;

        // A random command to check latency time
        if let Some(author) = ctx.interaction.author() {
            let start = Instant::now();
            let _ = ctx.framework.twilight_client.user(author.id).await?.model().await?;
            let response = format!(
                "{}\n`Get USER` API request achnowledged and received in `{}` milliseconds!",
                sent,
                start.elapsed().as_millis()
            );
            ctx.set_deferred().content(response).respond().await
        } else {
            Ok(())
        }
    }
}
