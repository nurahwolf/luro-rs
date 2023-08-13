use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, models::Roll, traits::luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "direction", desc = "Roll for a direction, such as `North East`!")]
pub struct DiceRollDirectionCommand {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}

impl LuroCommand for DiceRollDirectionCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.respond(|r| {
            if let Some(ephemeral) = self.ephemeral && ephemeral {
                r.ephemeral();
            }
            r.content(Roll::roll_direction())
        })
        .await
    }
}
