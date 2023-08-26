use luro_dice::DiceRoll;
use luro_framework::{command::LuroCommand, Framework, InteractionCommand, LuroInteraction};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "stats", desc = "Get some stats for your character sheet")]
pub struct Stats {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}

impl LuroCommand for Stats {
    async fn interaction_command<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        interaction
            .respond(&ctx, |r| {
                if self.ephemeral.unwrap_or_default() {
                    r.ephemeral();
                }
                r.content(format!("**Your stats, as requested:**\n{}", DiceRoll::roll_stats()))
            })
            .await?;
        Ok(())
    }
}
