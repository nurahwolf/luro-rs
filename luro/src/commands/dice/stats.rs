use luro_model::roll::Roll;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "stats", desc = "Get some stats for your character sheet")]
pub struct DiceStatsCommand {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}

impl LuroCommand for DiceStatsCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.respond(|r| {
            if let Some(ephemeral) = self.ephemeral && ephemeral {
                r.ephemeral();
            }
            r.content(format!("**Your stats, as requested:**\n{}", Roll::roll_stats()))
        })
        .await
    }
}
