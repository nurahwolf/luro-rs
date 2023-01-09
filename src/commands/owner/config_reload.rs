use crate::{
    config::{Config, Heck, Quotes, Stories},
    Context, Error, CONFIG_FILE_PATH, HECK_FILE_PATH, QUOTES_FILE_PATH, STORIES_FILE_PATH
};

/// Load variables from disk(Owner Only)
#[poise::command(owners_only, slash_command, prefix_command, ephemeral, category = "Owner")]
pub async fn reload(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data().config.write().await.reload(&Config::get(CONFIG_FILE_PATH));
    ctx.data().heck.write().await.reload(&Heck::get(HECK_FILE_PATH));
    ctx.data().quotes.write().await.reload(&Quotes::get(QUOTES_FILE_PATH));
    ctx.data().stories.write().await.reload(&Stories::get(STORIES_FILE_PATH));

    ctx.say("Done!").await?;
    Ok(())
}
