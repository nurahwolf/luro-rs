use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::CacheHttp;

use luro_core::{Context, Error};
use poise::serenity_prelude::Permissions;
use tracing::log::error;

/// Send an invite link to add me to your server!
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn invite(ctx: Context<'_>) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let bot_user = match ctx.cache() {
        Some(cache) => cache.current_user(),
        None => {
            ctx.say("Failed to get the current bot user from the cache, sorry :(").await?;
            return Ok(());
        }
    };

    let url = match bot_user.invite_url(ctx, Permissions::ADMINISTRATOR).await {
        Ok(invite) => invite,
        Err(why) => {
            error!("Encountered an error while trying to generate an invite: {}", why);
            ctx.say("Failed to generate an invite {why}").await?;
            return Ok(());
        }
    };

    let name = &bot_user.name;
    let description = match &ctx.data().config.read().await.git_url {
        Some(git_url) => format!("Click [here]({url}) to add {name} to your Discord server.\nNote! You can also get my source code from github at {git_url}"),
        None => format!("Click [here]({url}) to add {name} to your Discord server."),
    };

    ctx.send(|builder| {
        builder.embed(|embed| {
            embed
                .title(format!("{name}'s Invite URL"))
                .thumbnail(bot_user.avatar_url().unwrap_or_default())
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .description(description)
        })
    })
    .await?;

    Ok(())
}
