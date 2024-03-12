use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(
    twilight_interactions::command::CommandModel, twilight_interactions::command::CreateCommand,
)]
#[command(name = "stats", desc = "Get some stats for your character sheet")]
pub struct Stats {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>,
}

impl crate::models::CreateCommand for Stats {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        framework.respond(|r| {
            if self.ephemeral.unwrap_or_default() {
                r.ephemeral();
            }
            r.content(format!(
                "**Your stats, as requested:**\n{}",
                luro_dice::DiceRoll::roll_stats()
            ))
        })
        .await
    }
}
