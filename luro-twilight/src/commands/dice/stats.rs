use async_trait::async_trait;
use luro_dice::DiceRoll;
use luro_framework::{command::ExecuteLuroCommand, CommandInteraction};
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "stats", desc = "Get some stats for your character sheet")]
pub struct Stats {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>,
}

#[async_trait]
impl ExecuteLuroCommand for Stats {
    async fn interaction_command(&self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        ctx.respond(|r| {
            if self.ephemeral.unwrap_or_default() {
                r.ephemeral();
            }
            r.content(format!("**Your stats, as requested:**\n{}", DiceRoll::roll_stats()))
        })
        .await?;
        Ok(())
    }
}
