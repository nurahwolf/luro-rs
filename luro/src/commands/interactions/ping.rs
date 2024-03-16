use std::fmt::Write;
use std::time::Instant;

use crate::{
    builders::{EmbedBuilder, InteractionResponseBuilder},
    models::interaction::{InteractionContext, InteractionResult},
};

#[derive(twilight_interactions::command::CommandModel, twilight_interactions::command::CreateCommand)]
#[command(name = "ping", desc = "See my ping!")]
pub struct Ping {}

impl crate::models::CreateCommand for Ping {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        let mut embed = EmbedBuilder::default();
        embed.colour(framework.accent_colour().await).description("🏓 Pinging!");

        if let Some(average) = framework.latency.average() {
            embed.create_field(
                "Average Latency",
                &format!("`{}` milliseconds", average.as_millis().to_string()),
                true,
            );
        }

        if let Some(average) = framework.latency.received() {
            embed.create_field(
                "Last Acknowledgement",
                &format!("{} milliseconds ago", average.elapsed().as_millis()),
                true,
            );
        }

        if let Some(average) = framework.latency.sent() {
            embed.create_field(
                "Hearbeat Sent",
                &format!("{} milliseconds ago", average.elapsed().as_millis()),
                true,
            );
        }

        let mut num = 0;
        let mut heartbeats = String::new();
        for heartbeat in framework.latency.recent() {
            num += 1;
            writeln!(heartbeats, "{num} - {} milliseconds", heartbeat.as_millis())?
        }
        if !heartbeats.is_empty() {
            embed.create_field(
                "Hearbeats",
                &format!(
                    "**Total Heartbeats Recorded:** `{}`\n```{heartbeats}```",
                    framework.latency.periods().to_string()
                ),
                false,
            );
        }

        let start = Instant::now();
        let mut response = InteractionResponseBuilder::default();
        response.add_embed(embed);
        framework.response_send(&response).await?;
        let sent = format!(
            "Pong!\n`Send MESSAGE` API request achnowledged and received in `{}` milliseconds!",
            start.elapsed().as_millis()
        );
        response.content(sent.clone());

        framework.response_update(&response).await?;

        // A random command to check latency time
        let start = Instant::now();
        let _ = framework
            .gateway
            .twilight_client
            .user(framework.author_id()?)
            .await?
            .model()
            .await?;
        let content = format!(
            "{}\n`Get USER` API request achnowledged and received in `{}` milliseconds!",
            sent,
            start.elapsed().as_millis()
        );
        response.content(content);
        framework.response_update(&response).await
    }
}
