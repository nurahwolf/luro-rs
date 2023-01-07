use crate::{
    config::{Config, Heck, Quotes},
    Context, Error, CONFIG_FILE_PATH, HECK_FILE_PATH, QUOTES_FILE_PATH
};

/// Load variables from disk(Owner Only)
#[poise::command(owners_only, slash_command, prefix_command, ephemeral, category = "Owner")]
pub async fn reload(ctx: Context<'_>) -> Result<(), Error> {
    let config = if let Ok(mut lock) = ctx.data().config.lock() {
        lock.reload(&Config::get(CONFIG_FILE_PATH));
        true
    } else {
        false
    };

    let heck = if let Ok(mut lock) = ctx.data().heck.lock() {
        lock.reload(&Heck::get(HECK_FILE_PATH));
        true
    } else {
        false
    };

    let quotes = if let Ok(mut lock) = ctx.data().quotes.lock() {
        lock.reload(&Quotes::get(QUOTES_FILE_PATH));
        true
    } else {
        false
    };

    if !config {
        ctx.say("Failed to lock config mutex").await?;
    }

    if !heck {
        ctx.say("Failed to lock heck mutex").await?;
    }

    if !quotes {
        ctx.say("Failed to lock quotes mutex").await?;
    }

    ctx.say("Done!").await?;
    Ok(())
}
