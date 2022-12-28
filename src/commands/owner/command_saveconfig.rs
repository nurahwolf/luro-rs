use crate::{
    config::{Config, Heck, Quotes, Secrets, Stories},
    Context, Error, CONFIG_FILE_PATH, HECK_FILE_PATH, QUOTES_FILE_PATH, SECRETS_FILE_PATH, STORIES_FILE_PATH
};

/// Save variables to disk (Owner Only)
#[poise::command(owners_only, slash_command, prefix_command, ephemeral, category = "Owner")]
pub async fn save_config(ctx: Context<'_>) -> Result<(), Error> {
    Config::write(&ctx.data().config, CONFIG_FILE_PATH);
    Heck::write(&ctx.data().heck, HECK_FILE_PATH);
    Quotes::write(&ctx.data().quotes, QUOTES_FILE_PATH);
    Secrets::write(&ctx.data().secrets, SECRETS_FILE_PATH);
    Stories::write(&ctx.data().stories, STORIES_FILE_PATH);

    ctx.say("Done!").await?;
    Ok(())
}
