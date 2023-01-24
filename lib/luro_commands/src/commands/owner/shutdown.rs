use luro_core::{Context, Error};

/// Imagine finding a hidden command. Shuts down the bot.
#[poise::command(prefix_command, owners_only, hide_in_help, category = "Owner")]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Goodbye cruel world...").await?;
    ctx.framework().shard_manager().lock().await.shutdown_all().await;
    Ok(())
}
