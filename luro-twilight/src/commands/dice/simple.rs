use luro_dice::{DiceRoll, RollResult, RollValue};
use luro_framework::{command::LuroCommand, Framework, InteractionCommand, LuroInteraction};
use luro_model::database::drivers::LuroDatabaseDriver;
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
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
    divide: Option<i64>
}

impl LuroCommand for Simple {
    async fn interaction_command<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
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

        let result = DiceRoll::roll_inline(&roll, false).unwrap_or(RollResult {
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

        if result.dice_total == RollValue::Int(0) {
            result_string.push_str(&format!("\n-----\n*You failed. This is known as a skill issue.*"))
        }

        interaction
            .respond(&ctx, |r| {
                if self.ephemeral.unwrap_or_default() {
                    r.ephemeral();
                }
                r.content(result_string)
            })
            .await?;
        Ok(())
    }
}
