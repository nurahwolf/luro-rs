use luro_dice::DiceRoll;
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand,)]
#[command(name = "direction", desc = "Roll for a direction, such as `North East`!")]
pub struct Direction {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool,>,
}

impl LuroCommand for Direction {
    async fn run_command<D: LuroDatabaseDriver,>(self, ctx: LuroSlash<D,>,) -> anyhow::Result<(),> {
        ctx.respond(|r| {
            if self.ephemeral.unwrap_or_default() {
                r.ephemeral();
            }
            r.content(DiceRoll::roll_direction(),)
        },)
            .await
    }
}
