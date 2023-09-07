use luro_dice::{DiceRoll, RollResult, RollValue};
use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "roll", desc = "Roll those freaking dice!!!")]
pub struct Roll {
    /// Standard Dice Notation: 6d20dl2-10 (6x d20 dice, drop lowest 2, take away 10 from result)
    dice: String,
    /// Add context to your role, such as for D&D. Use `\` to not have your reason in a code block.
    reason: Option<String>,
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}
#[async_trait::async_trait]

impl LuroCommandTrait for Roll {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let result = DiceRoll::roll_inline(&data.dice, false).unwrap_or(RollResult {
            string_result: "I genuinely am a loss for words for whatever fucking format you just tried. Here, have a free `69` since you bewildered me so goddarn much.".to_string(),
            dice_total: RollValue::Int(69)
        });
        let mut result_string = if let Some(mut reason) = data.reason {
            if !reason.starts_with('\\') {
                reason = format!("```{reason}```")
            } else {
                reason.remove(0);
                reason.push('\n')
            }

            format!(
                "<@{}> is rolling for the reason:\n{reason}\n**Result:** `{}`\n**Total:** `{}`",
                interaction.author_id(),
                result.string_result,
                result.dice_total
            )
        } else {
            format!("**Result:** `{}`\n**Total:** `{}`", result.string_result, result.dice_total)
        };

        if result.dice_total == RollValue::Int(20) {
            result_string.push_str(&format!("\n-----\n*Whoa, a 20!! Congrats!! <3*"))
        }

        if result.dice_total == RollValue::Int(1) {
            result_string.push_str(&format!("\n-----\n*You failed. This is known as a skill issue.*"))
        }

        interaction
            .respond(&ctx, |r| {
                if data.ephemeral.unwrap_or_default() {
                    r.ephemeral();
                }
                r.content(result_string)
            })
            .await?;
        Ok(())
    }
}
