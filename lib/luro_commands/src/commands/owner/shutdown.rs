use luro_core::{
    config::Config, heck::Hecks, quotes::Quotes, stories::Stories, Context, Error, CONFIG_FILE_PATH, HECK_FILE_PATH,
    QUOTES_FILE_PATH, STORIES_FILE_PATH
};

/// Imagine finding a hidden command. Shuts down the bot.
#[poise::command(prefix_command, owners_only, hide_in_help, category = "Owner")]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    // Save config to disk
    Config::write(&ctx.data().config.write().await.clone(), CONFIG_FILE_PATH).await;
    Hecks::write(&ctx.data().heck.write().await.clone(), HECK_FILE_PATH).await;
    Quotes::write(&ctx.data().quotes.write().await.clone(), QUOTES_FILE_PATH).await;
    Stories::write(&ctx.data().stories.write().await.clone(), STORIES_FILE_PATH).await;
    // Now shutdown
    ctx.say("Goodbye cruel world...").await?;
    ctx.framework().shard_manager().lock().await.shutdown_all().await;
    Ok(())
}
