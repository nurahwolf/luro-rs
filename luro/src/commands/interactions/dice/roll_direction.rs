use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(
    twilight_interactions::command::CommandModel, twilight_interactions::command::CreateCommand,
)]
#[command(
    name = "direction",
    desc = "Roll for a direction, such as `North East`!"
)]
pub struct Direction {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>,
}

impl crate::models::CreateCommand for Direction {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        framework.respond(|r| {
            if self.ephemeral.unwrap_or_default() {
                r.ephemeral();
            }
            r.content(luro_dice::DiceRoll::roll_direction())
        })
        .await
    }
}
