use crate::{Context, Error};

/// Print information of the guilds I'm in!
#[poise::command(slash_command, prefix_command, category = "Owner")]
pub async fn guilds(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::servers(ctx).await?;
    Ok(())
}
