use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    models::{LuroSlash, Roll},
    traits::luro_command::LuroCommand
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "stats", desc = "Get some stats for your character sheet")]
pub struct DiceStatsCommand {}

#[async_trait]
impl LuroCommand for DiceStatsCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.content(format!("**Your stats, as requested:**\n{}", Roll::roll_stats()))
            .respond()
            .await
    }
}
