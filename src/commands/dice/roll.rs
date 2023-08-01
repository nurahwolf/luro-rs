use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    models::{LuroSlash, Roll, RollResult, RollValue},
    traits::luro_command::LuroCommand
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "dice", desc = "Roll those freaking dice!!!")]
pub struct DiceRollCommand {
    /// Standard Dice Notation: 6d20dl2-10 (6x d20 dice, drop lowest 2, take away 10 from result)
    dice: String,
    /// Add context to your role, such as for D&D
    reason: Option<String>,
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>,
}

#[async_trait]
impl LuroCommand for DiceRollCommand {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let result = Roll::roll_inline(&self.dice, false).unwrap_or(RollResult {
            string_result: "I genuinely am a loss for words for whatever fucking format you just tried. Here, have a free `69` since you bewildered me so goddarn much.".to_string(),
            dice_total: RollValue::Int(69)
        });
        let mut result_string = if let Some(reason) = self.reason {
            format!(
                "<@{}> is rolling for the reason:\n```{reason}```\n**Result:** `{}`\n**Total:** `{}`",
                ctx.author()?.id,
                result.string_result,
                result.dice_total
            )
        } else {
            format!("**Result:** `{}`\n**Total:** `{}`", result.string_result, result.dice_total)
        };

        if result.dice_total == RollValue::Int(20) {
            result_string.push_str(&format!("\n-----\n*Whoa, a d20!! Congrats!! <3*"))
        }

        if result.dice_total == RollValue::Int(0) {
            result_string.push_str(&format!("\n-----\n*You failed. This is known as a skill issue.*"))
        }

        if let Some(ephemeral) = self.ephemeral && ephemeral {
            ctx.content(result_string).ephemeral().respond().await
        } else {
            ctx.content(result_string).respond().await
        }

    }
}
