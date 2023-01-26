use luro_core::{config::Config, heck::Hecks, quotes::Quotes, stories::Stories};

use luro_core::{Context, Error, CONFIG_FILE_PATH, HECK_FILE_PATH, QUOTES_FILE_PATH, STORIES_FILE_PATH};

// TODO: Write a function for this

/// Save variables to disk (Owner Only)
#[poise::command(owners_only, slash_command, prefix_command, ephemeral, category = "Owner")]
pub async fn save(ctx: Context<'_>) -> Result<(), Error> {
    Config::write(&ctx.data().config.write().await.clone(), CONFIG_FILE_PATH).await;
    Hecks::write(&ctx.data().heck.write().await.clone(), HECK_FILE_PATH).await;
    Quotes::write(&ctx.data().quotes.write().await.clone(), QUOTES_FILE_PATH).await;
    Stories::write(&ctx.data().stories.write().await.clone(), STORIES_FILE_PATH).await;

    ctx.say("Done!").await?;
    Ok(())
}
