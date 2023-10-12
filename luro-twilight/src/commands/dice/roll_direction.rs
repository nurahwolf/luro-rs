use luro_dice::DiceRoll;
use luro_framework::{command::ExecuteLuroCommand, CommandInteraction};

use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "direction", desc = "Roll for a direction, such as `North East`!")]
pub struct Direction {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>,
}

impl ExecuteLuroCommand for Direction {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        ctx.respond(|r| {
            if self.ephemeral.unwrap_or_default() {
                r.ephemeral();
            }
            r.content(DiceRoll::roll_direction())
        })
        .await?;
        Ok(())
    }
}
