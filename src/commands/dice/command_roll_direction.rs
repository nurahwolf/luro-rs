use crate::{commands::dice::function_diceroller::roll_direction, Context, Error};

/// Roll a direction, such as 'North East'
#[poise::command(slash_command, prefix_command, category = "Dice")]
pub async fn direction(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(roll_direction()).await?;
    Ok(())
}
