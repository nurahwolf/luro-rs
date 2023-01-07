use crate::{
    config::{Config, Heck, Quotes},
    Context, Error, CONFIG_FILE_PATH, HECK_FILE_PATH, QUOTES_FILE_PATH
};

// TODO: Write a function for this

/// Save variables to disk (Owner Only)
#[poise::command(owners_only, slash_command, prefix_command, ephemeral, category = "Owner")]
pub async fn save(ctx: Context<'_>) -> Result<(), Error> {
    let config = if let Ok(lock) = ctx.data().config.lock() {
        Config::write(&lock, CONFIG_FILE_PATH);
        true
    } else {
        false
    };

    let heck = if let Ok(lock) = ctx.data().heck.lock() {
        Heck::write(&lock, HECK_FILE_PATH);
        true
    } else {
        false
    };

    let quotes = if let Ok(lock) = ctx.data().quotes.lock() {
        Quotes::write(&lock, QUOTES_FILE_PATH);
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
