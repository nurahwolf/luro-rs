use crate::{Context, Error};

/// Make me say garbage
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn command_usage(
    ctx: Context<'_>
) -> Result<(), Error> {
    let total = ctx.data().command_total.read().await;
    ctx.say(format!("**{total:?}** commands have been run since I was last restarted!!")).await?;
    Ok(())
}
