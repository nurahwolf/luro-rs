use std::time::SystemTime;

use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::slash::Slash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "ping", desc = "See my ping!")]
pub struct PingCommand {}

#[async_trait]
impl LuroCommand for PingCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let start = SystemTime::now();
        let mut embed = ctx.default_embed().await?.description("🏓 Pinging!");
        if let Some(average) = ctx.latency.average() {
            embed = embed.field(EmbedFieldBuilder::new("Average Latency", average.as_secs().to_string()).inline())
        }
        if let Some(average) = ctx.latency.received() {
            embed = embed.field(EmbedFieldBuilder::new("Last Acknowledgement", average.elapsed().as_secs().to_string()).inline())
        }
        if let Some(average) = ctx.latency.sent() {
            embed = embed.field(EmbedFieldBuilder::new("Last Heartbeat Sent", average.elapsed().as_secs().to_string()).inline())
        }
        if let Some(average) = ctx.latency.recent().last() {
            embed = embed.field(EmbedFieldBuilder::new("Last Heartbeat Sent", average.as_secs().to_string()).inline())
        }
        embed = embed.field(EmbedFieldBuilder::new("Total Heartbeats", ctx.latency.periods().to_string()).inline());
        ctx.embed(embed.clone().build())?.respond().await?;

        embed = embed.description(format!(
            "🏓 Pong! API responded in `{}` seconds! 🏓",
            start.elapsed()?.as_secs()
        ));

        ctx.update().embed(embed.build())?.respond().await
    }
}
