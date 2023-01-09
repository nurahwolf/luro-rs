use crate::{
    config::{Config, Heck, Quotes, Stories},
    Context, Error, CONFIG_FILE_PATH, HECK_FILE_PATH, QUOTES_FILE_PATH, STORIES_FILE_PATH
};

// TODO: Write a function for this

/// Save variables to disk (Owner Only)
#[poise::command(owners_only, slash_command, prefix_command, ephemeral, category = "Owner")]
pub async fn save(ctx: Context<'_>) -> Result<(), Error> {
    Config::write(&ctx.data().config.write().await.clone(), CONFIG_FILE_PATH);
    Heck::write(&ctx.data().heck.write().await.clone(), HECK_FILE_PATH);
    Quotes::write(&ctx.data().quotes.write().await.clone(), QUOTES_FILE_PATH);
    Stories::write(&ctx.data().stories.write().await.clone(), STORIES_FILE_PATH);

    ctx.say("Done!").await?;
    Ok(())
}
