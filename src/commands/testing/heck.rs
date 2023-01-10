use poise::serenity_prelude::User;
use rand::seq::SliceRandom;

use crate::{config::Heck, Context, Error, HECK_FILE_PATH};

async fn heck_function(author: &User, user: User, hecks: &Vec<String>) -> String {
    let rng = &mut rand::thread_rng();
    match hecks.choose(rng) {
        Some(heck) => heck.replace("<user>", user.to_string().as_str()).replace("<author>", author.to_string().as_str()),
        None => "No hecks found! (Make sure `heck.toml` exists :)".to_string()
    }
}

/// Send a silly message at a user
#[poise::command(slash_command, prefix_command, category = "Testing")]
pub async fn heck(
    ctx: Context<'_>,
    #[description = "User to heck"] user: User,
    #[description = "Add a heck message. Format: `<author> topped <user>!"] add_heck: Option<String>
) -> Result<(), Error> {
    if let Some(add_heck) = add_heck {
        let mut write = ctx.data().heck.write().await;
        // First Check: If an owner is running the command, don't check to make sure the message contains both <user> and <author>.
        // This is so you can have custom messages, and its implied the owners know what they are doing...
        // Second Check: Make sure the input contains both <user> and <author>
        if ctx.framework().options.owners.contains(&ctx.author().id) || add_heck.contains("<user>") && add_heck.contains("<author>") {
            write.heck.append(&mut vec![add_heck.clone()]);
        } else {
            // Format not allowed!
            ctx.say(format!(
                "Your heck was `{add_heck}` but the format was wrong. Make sure you include `<author>` and `<user>`!\n\nFor example: `<author> topped <user>!`"
            ))
            .await?;
        }
        Heck::write(&write, HECK_FILE_PATH); // Save our new heck to the database
        let new_heck = add_heck.replace("<user>", user.to_string().as_str()).replace("<author>", ctx.author().to_string().as_str()); // Format the heck to mention the user in this instance
        ctx.say(format!("{new_heck}\n*Added this heck to the database!*")).await?; // send our response!
        return Ok(()); // We can exit the function now
    };

    // User is not adding a heck, so lets get one randomly
    let hecks = &ctx.data().heck.read().await.heck;
    let heck = heck_function(ctx.author(), user, hecks).await;
    ctx.say(heck).await?;

    Ok(())
}

/// Send a silly message at a user - Context menu edition
#[poise::command(category = "Testing", context_menu_command = "Heck this user :3c")]
pub async fn heck_user(ctx: Context<'_>, #[description = "User to heck"] user: User) -> Result<(), Error> {
    let hecks = &ctx.data().heck.read().await.heck;
    let heck = heck_function(ctx.author(), user, hecks).await;
    ctx.say(heck).await?;
    Ok(())
}
