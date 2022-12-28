use poise::serenity_prelude::CacheHttp;

use crate::{utils::guild_accent_colour, Context, Error};
use poise::serenity_prelude::Permissions;
use tracing::log::error;

/// Send an invite link to add me to your server!
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn invite(ctx: Context<'_>) -> Result<(), Error> {
    let bot_user = ctx.cache().expect("Failed to get the current bot user in cache").current_user();
    let url = match bot_user.invite_url(ctx, Permissions::ADMINISTRATOR).await {
        Ok(invite) => invite,
        Err(why) => {
            error!("Encountered an error while trying to generate an invite: {}", why);
            ctx.say("Failed to generate an invite {why}").await?;
            return Ok(());
        }
    };

    let name = &bot_user.name;
    let avatar = bot_user.avatar_url().unwrap_or_default();

    ctx.send(|builder| {
        builder.embed(|embed| {
            embed
                .title(format!("{name} Invite URL"))
                .thumbnail(avatar)
                .color(guild_accent_colour(ctx.data().config.accent_colour, ctx.guild()))
                .description(format!("Click [here]({url}) to add {name} to your Discord server."))
        })
    })
    .await?;

    Ok(())
}
