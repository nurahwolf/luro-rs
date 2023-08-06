use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    models::{LuroResponse, Roll},
    traits::luro_command::LuroCommand,
    LuroContext
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "stats", desc = "Get some stats for your character sheet")]
pub struct DiceStatsCommand {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}

#[async_trait]
impl LuroCommand for DiceStatsCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let content = format!("**Your stats, as requested:**\n{}", Roll::roll_stats());
        if let Some(ephemeral) = self.ephemeral && ephemeral {
            slash.content(content).ephemeral();ctx.respond(&mut slash).await
        } else {
            slash.content(content);ctx.respond(&mut slash).await
        }
    }
}
