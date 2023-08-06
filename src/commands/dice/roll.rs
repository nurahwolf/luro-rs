use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{
    models::{LuroResponse, Roll, RollResult, RollValue},
    traits::luro_command::LuroCommand,
    LuroContext
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "dice", desc = "Roll those freaking dice!!!")]
pub struct DiceRollCommand {
    /// Standard Dice Notation: 6d20dl2-10 (6x d20 dice, drop lowest 2, take away 10 from result)
    dice: String,
    /// Add context to your role, such as for D&D. Use `\` to not have your reason in a code block.
    reason: Option<String>,
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>
}

#[async_trait]
impl LuroCommand for DiceRollCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let (author, _) = ctx.get_interaction_author(&slash)?;
        let result = Roll::roll_inline(&self.dice, false).unwrap_or(RollResult {
            string_result: "I genuinely am a loss for words for whatever fucking format you just tried. Here, have a free `69` since you bewildered me so goddarn much.".to_string(),
            dice_total: RollValue::Int(69)
        });
        let mut result_string = if let Some(mut reason) = self.reason {
            if !reason.starts_with('\\') {
                reason = format!("```{reason}```")
            } else {
                reason.remove(0);
                reason.push('\n')
            }

            format!(
                "<@{}> is rolling for the reason:\n{reason}\n**Result:** `{}`\n**Total:** `{}`",
                author.id, result.string_result, result.dice_total
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

        if let Some(ephemeral) = self.ephemeral && ephemeral {
            slash.content(result_string).ephemeral();ctx.respond(&mut slash).await
        } else {
            slash.content(result_string);ctx.respond(&mut slash).await
        }
    }
}
