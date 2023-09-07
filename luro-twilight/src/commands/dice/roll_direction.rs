use luro_dice::DiceRoll;
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "direction", desc = "Roll for a direction, such as `North East`!")]
pub struct Direction {
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}
#[async_trait::async_trait]

impl LuroCommandTrait for Direction {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        interaction
            .respond(&ctx, |r| {
                if data.ephemeral.unwrap_or_default() {
                    r.ephemeral();
                }
                r.content(DiceRoll::roll_direction())
            })
            .await?;
        Ok(())
    }
}
