use crate::{Context, Error};

/// Are you optimistic or pessimistic?
#[poise::command(slash_command, prefix_command, category = "Silly")]
pub async fn oracle(ctx: Context<'_>, #[description = "Take a decision"] b: bool) -> Result<(), Error> {
    if b {
        ctx.say("You seem to be an optimistic kind of person...").await?;
    } else {
        ctx.say("You seem to be a pessimistic kind of person...").await?;
    }
    Ok(())
}
