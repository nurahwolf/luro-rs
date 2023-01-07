use crate::{functions::diceroller::roll_stats, Context, Error};

/// Roll your stats (4d6, drop lowest)
#[poise::command(slash_command, prefix_command, category = "Dice")]
pub async fn stats(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(format!("**Your stats, as requested:**\n{}", roll_stats())).await?;
    Ok(())
}
