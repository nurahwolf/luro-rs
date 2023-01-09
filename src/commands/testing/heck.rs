use poise::serenity_prelude::{CacheHttp, User};
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
    let hecks = &ctx.data().heck.read().await.heck;
    if let Some(add_heck) = add_heck {
        if add_heck.contains("<user>") && add_heck.contains("<author>") {
            let new_heck = add_heck.replace("<user>", user.to_string().as_str()).replace("<author>", ctx.author().to_string().as_str());

            ctx.data().heck.write().await.heck.append(&mut vec![new_heck.clone()]);

            ctx.say(format!(
                "{}\n\n*Oh yeah, I just added that heck succesfully <3 -{}*",
                new_heck,
                ctx.http().get_user(ctx.framework().bot_id.0).await?
            ))
            .await?;
            Heck::write(&ctx.data().heck.write().await.clone(), HECK_FILE_PATH);
        } else {
            ctx.say(format!(
                "Your heck was `{add_heck}` but the format was wrong. Make sure you include `<author>` and `<user>`!\n\nFor example: `<author> topped <user>!`"
            ))
            .await?;
        }
        return Ok(());
    }

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
