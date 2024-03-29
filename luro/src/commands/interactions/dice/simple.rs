use std::fmt::Write;

use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(twilight_interactions::command::CommandModel, twilight_interactions::command::CreateCommand)]
#[command(name = "simple", desc = "A simpler version, for those not wanting to deal with foruma")]
pub struct Simple {
    /// Total number of dice, for example two dice
    #[command(min_value = 1, max_value = 10000)]
    dice: i64,
    /// How many sides the dice should have. 20 for a d20
    #[command(min_value = 1, max_value = 10000)]
    sides: i64,
    /// Add context to your role, such as for D&D
    reason: Option<String>,
    /// Set your message to ephemeral, useful for if you don't want someone to see your rolls.
    ephemeral: Option<bool>,
    /// Keep the highest amount of X dice
    #[command(min_value = 1, max_value = 100)]
    keep_highest: Option<i64>,
    /// Keep the lowest amount of X dice
    #[command(min_value = 1, max_value = 100)]
    keep_lowest: Option<i64>,
    /// Drop the highest amount of X dice
    #[command(min_value = 1, max_value = 100)]
    drop_highest: Option<i64>,
    /// Drop the lowest amount of X dice
    #[command(min_value = 1, max_value = 100)]
    drop_lowest: Option<i64>,
    /// Add X value to the result
    #[command(min_value = 1, max_value = 100)]
    add: Option<i64>,
    /// Take away X value from the result
    #[command(min_value = 1, max_value = 10000)]
    take: Option<i64>,
    /// Multiply the result by X amount
    #[command(min_value = 1, max_value = 1000)]
    multiply: Option<i64>,
    /// Divide the result by X amount
    #[command(min_value = 1, max_value = 1000)]
    divide: Option<i64>,
}

impl crate::models::CreateCommand for Simple {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        let mut roll = format!("{}d{}", self.dice, self.sides);

        if let Some(operation) = self.keep_highest {
            write!(roll, "kh{operation}")?
        }

        if let Some(operation) = self.keep_lowest {
            write!(roll, "kl{operation}")?
        }

        if let Some(operation) = self.drop_highest {
            write!(roll, "dh{operation}")?
        }

        if let Some(operation) = self.drop_lowest {
            write!(roll, "dl{operation}")?
        }

        if let Some(operation) = self.add {
            write!(roll, "+{operation}")?
        }

        if let Some(operation) = self.take {
            write!(roll, "-{operation}")?
        }

        if let Some(operation) = self.multiply {
            write!(roll, "*{operation}")?
        }

        if let Some(operation) = self.divide {
            write!(roll, "/{operation}")?
        }

        let result = luro_dice::DiceRoll::roll_inline(&roll, false).unwrap_or(luro_dice::RollResult {
            string_result: "I genuinely am a loss for words for whatever fucking format you just tried. Here, have a free `69` since you bewildered me so goddarn much.".to_string(),
            dice_total: luro_dice::RollValue::Int(69)
        });
        let mut result_string = if let Some(mut reason) = self.reason.clone() {
            if !reason.starts_with('\\') {
                reason = format!("```{reason}```")
            } else {
                reason.remove(0);
                reason.push('\n')
            }

            format!(
                "<@{}> is rolling for the reason:\n{reason}\n**Result:** `{}`\n**Total:** `{}`",
                framework.author_id(),
                result.string_result,
                result.dice_total
            )
        } else {
            format!("**Result:** `{}`\n**Total:** `{}`", result.string_result, result.dice_total)
        };

        if result.dice_total == luro_dice::RollValue::Int(20) {
            result_string.push_str("\n-----\n*Whoa, a 20!! Congrats!! <3*")
        }

        if result.dice_total == luro_dice::RollValue::Int(0) {
            result_string.push_str("\n-----\n*You failed. This is known as a skill issue.*")
        }

        framework
            .respond(|r| {
                if self.ephemeral.unwrap_or_default() {
                    r.ephemeral();
                }
                r.content(result_string)
            })
            .await
    }
}
