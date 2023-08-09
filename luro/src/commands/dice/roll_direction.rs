use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{models::Roll, slash::Slash, traits::luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "direction", desc = "Roll for a direction, such as `North East`!")]
pub struct DiceRollDirectionCommand {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}

#[async_trait]
impl LuroCommand for DiceRollDirectionCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        if let Some(ephemeral) = self.ephemeral && ephemeral {
            ctx.content(Roll::roll_direction()).ephemeral().respond().await
        } else {
            ctx.content(Roll::roll_direction()).respond().await
        }
    }
}
