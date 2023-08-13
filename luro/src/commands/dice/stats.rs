
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{models::Roll, slash::Slash, traits::luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "stats", desc = "Get some stats for your character sheet")]
pub struct DiceStatsCommand {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}


impl LuroCommand for DiceStatsCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        let content = format!("**Your stats, as requested:**\n{}", Roll::roll_stats());
        if let Some(ephemeral) = self.ephemeral && ephemeral {
            ctx.content(content).ephemeral().respond().await
        } else {
            ctx.content(content).respond().await
        }
    }
}
