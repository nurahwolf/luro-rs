use std::fmt::Write;
use std::time::Instant;

use async_trait::async_trait;
use luro_builder::embed::EmbedBuilder;
use luro_framework::{
    command::{LuroCommandBuilder, LuroCommandTrait},
    Framework, InteractionCommand, LuroInteraction,
};
use luro_model::response::LuroResponse;
use twilight_interactions::command::{CommandModel, CreateCommand};

use luro_model::database_driver::LuroDatabaseDriver;

#[derive(CommandModel, CreateCommand)]
#[command(name = "ping", desc = "See my ping!")]
pub struct Ping {}

impl<D: LuroDatabaseDriver + 'static> LuroCommandBuilder<D> for Ping {}

#[async_trait]
impl LuroCommandTrait for Ping {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let mut embed = EmbedBuilder::default();
        embed.colour(interaction.accent_colour(&ctx).await).description("🏓 Pinging!");
        if let Some(average) = interaction.latency.average() {
            embed.create_field(
                "Average Latency",
                &format!("`{}` milliseconds", average.as_millis().to_string()),
                true,
            );
        }

        if let Some(average) = interaction.latency.received() {
            embed.create_field(
                "Last Acknowledgement",
                &format!("{} milliseconds ago", average.elapsed().as_millis()),
                true,
            );
        }

        if let Some(average) = interaction.latency.sent() {
            embed.create_field(
                "Hearbeat Sent",
                &format!("{} milliseconds ago", average.elapsed().as_millis()),
                true,
            );
        }

        let mut num = 0;
        let mut heartbeats = String::new();
        for heartbeat in interaction.latency.recent() {
            num += 1;
            writeln!(heartbeats, "{num} - {} milliseconds", heartbeat.as_millis())?
        }
        if !heartbeats.is_empty() {
            embed.create_field(
                "Hearbeats",
                &format!(
                    "**Total Heartbeats Recorded:** `{}`\n```{heartbeats}```",
                    interaction.latency.periods().to_string()
                ),
                false,
            );
        }

        let start = Instant::now();
        let mut response = LuroResponse::default();
        response.add_embed(embed);
        interaction.send_response(&ctx, response.clone()).await?;
        let sent = format!(
            "Pong!\n`Send MESSAGE` API request achnowledged and received in `{}` milliseconds!",
            start.elapsed().as_millis()
        );
        response.content(sent.clone());

        interaction.send_response(&ctx, response.clone()).await?;

        // A random command to check latency time
        let start = Instant::now();
        let _ = ctx.twilight_client.user(interaction.author_id()).await?.model().await?;
        let user = format!(
            "{}\n`Get USER` API request achnowledged and received in `{}` milliseconds!",
            sent,
            start.elapsed().as_millis()
        );
        response.content(user);
        interaction.send_response(&ctx, response).await?;

        Ok(())
    }
}
