use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    models::{LuroResponse, Roll},
    traits::luro_command::LuroCommand,
    LuroContext
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "direction", desc = "Roll for a direction, such as `North East`!")]
pub struct DiceRollDirectionCommand {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}

#[async_trait]
impl LuroCommand for DiceRollDirectionCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        if let Some(ephemeral) = self.ephemeral && ephemeral {
            slash.content(Roll::roll_direction()).ephemeral();ctx.respond(&mut slash).await
        } else {
            slash.content(Roll::roll_direction());ctx.respond(&mut slash).await
        }
    }
}
