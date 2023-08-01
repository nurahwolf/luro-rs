use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    models::{LuroSlash, Roll},
    traits::luro_command::LuroCommand
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "direction", desc = "Roll for a direction, such as `North East`!")]
pub struct DiceRollDirectionCommand {}

#[async_trait]
impl LuroCommand for DiceRollDirectionCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.content(Roll::roll_direction()).respond().await
    }
}
