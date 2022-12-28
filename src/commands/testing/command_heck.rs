use poise::serenity_prelude::{CacheHttp, User};
use rand::seq::SliceRandom;

use crate::{config::Heck, Context, Error, HECK_FILE_PATH};

async fn heck_function(ctx: Context<'_>, user: User) -> Result<(), Error> {
    let heck = ctx.data().heck.heck.choose(&mut rand::thread_rng()).unwrap();
    let heck_format_once = heck.as_str().replace("<user>", user.to_string().as_str());
    let heck_format_twice = heck_format_once.replace("<author>", ctx.author().to_string().as_str());
    ctx.say(heck_format_twice).await?;

    Ok(())
}

/// Send a silly message at a user
#[poise::command(slash_command, prefix_command, category = "Testing")]
pub async fn heck(
    ctx: Context<'_>,
    #[description = "User to heck"] user: User,
    #[description = "Add a heck message. Format: `<author> topped <user>!"] add_heck: Option<String>
) -> Result<(), Error> {
    if let Some(add_heck) = add_heck {
        if add_heck.contains("<user>") && add_heck.contains("<author>") {
            let mut new_heck = vec![add_heck.clone()];
            let hecks = &mut ctx.data().heck.clone();
            hecks.heck.append(&mut new_heck);
            let heck_format_once = add_heck.as_str().replace("<user>", user.to_string().as_str());
            let heck_format_twice = heck_format_once.replace("<author>", ctx.author().to_string().as_str());
            ctx.say(format!(
                "{}\n\n*Oh yeah, I just added that heck succesfully <3 -{}*",
                heck_format_twice,
                ctx.http().get_user(ctx.framework().bot_id.0).await?
            ))
            .await?;
            Heck::write(&ctx.data().heck, HECK_FILE_PATH);
        } else {
            ctx.say(format!(
                "Your heck was `{add_heck}` but the format was wrong. Make sure you include `<author>` and `<user>`!\n\nFor example: `<author> topped <user>!`"
            ))
            .await?;
        }
        return Ok(());
    }

    heck_function(ctx, user).await?;
    Ok(())
}

/// Send a silly message at a user - Context menu edition
#[poise::command(category = "Testing", context_menu_command = "Heck this user :3c")]
pub async fn heck_user(ctx: Context<'_>, #[description = "User to heck"] user: User) -> Result<(), Error> {
    heck_function(ctx, user).await?;
    Ok(())
}
