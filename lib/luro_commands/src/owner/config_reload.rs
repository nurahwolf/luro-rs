use luro_core::{config::Config, heck::Heck, quotes::Quotes, stories::Stories};

use luro_core::{Context, Error, CONFIG_FILE_PATH, HECK_FILE_PATH, QUOTES_FILE_PATH, STORIES_FILE_PATH};

/// Load variables from disk(Owner Only)
#[poise::command(owners_only, slash_command, prefix_command, ephemeral, category = "Owner")]
pub async fn reload(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data().config.write().await.reload(&Config::get(CONFIG_FILE_PATH).await);
    ctx.data().heck.write().await.reload(&Heck::get(HECK_FILE_PATH).await);
    ctx.data().quotes.write().await.reload(&Quotes::get(QUOTES_FILE_PATH).await);
    ctx.data()
        .stories
        .write()
        .await
        .reload(&Stories::get(STORIES_FILE_PATH).await);

    ctx.say("Done!").await?;
    Ok(())
}
