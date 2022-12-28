use crate::{
    commands::dice::function_diceroller::{roll_inline, RollResult, Value},
    Context, Error
};

/// Roll those dice nerd
#[poise::command(slash_command, prefix_command, category = "Dice")]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "Standard Dice Notation: 6d20dl2-10 (6x d20 dice, drop lowest 2, take away 10 from result)"]
    #[rest]
    dice: String
) -> Result<(), Error> {
    let result = roll_inline(&dice, false).unwrap_or(RollResult {
        string_result: "I genuinely am a loss for words for whatever fucking format you just tried. Here, have a free `69` since you bewildered me so goddarn much.".to_string(),
        dice_total: Value::Int(69)
    });
    let result_string = format!("**Result:** {}\n**Total:** {}", result.string_result, result.dice_total);

    ctx.say(result_string).await?;

    Ok(())
}
