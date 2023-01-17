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

    let invite_admin = match bot_user.invite_url(ctx, Permissions::ADMINISTRATOR).await {
        Ok(invite) => invite,
        Err(why) => {
            error!("Encountered an error while trying to generate an invite: {}", why);
            ctx.say("Failed to generate an invite {why}").await?;
            return Ok(());
        }
    };

    let invite_generic = match bot_user.invite_url(ctx, Permissions::ADD_REACTIONS | Permissions::ATTACH_FILES | Permissions::BAN_MEMBERS  | Permissions::CHANGE_NICKNAME | Permissions::CONNECT | Permissions::CREATE_INSTANT_INVITE | Permissions::CREATE_PRIVATE_THREADS | Permissions::CREATE_PUBLIC_THREADS | Permissions::DEAFEN_MEMBERS | Permissions::EMBED_LINKS | Permissions::KICK_MEMBERS | Permissions::MANAGE_CHANNELS | Permissions::MANAGE_EMOJIS_AND_STICKERS | Permissions::MANAGE_EVENTS | Permissions::MANAGE_GUILD | Permissions::MANAGE_MESSAGES | Permissions::MANAGE_NICKNAMES | Permissions::MANAGE_ROLES | Permissions::MANAGE_THREADS | Permissions::MANAGE_WEBHOOKS | Permissions::MODERATE_MEMBERS | Permissions::READ_MESSAGE_HISTORY | Permissions::SEND_MESSAGES |Permissions::SEND_MESSAGES_IN_THREADS | Permissions::USE_EMBEDDED_ACTIVITIES | Permissions::USE_EXTERNAL_EMOJIS | Permissions::USE_EXTERNAL_STICKERS | Permissions::VIEW_AUDIT_LOG | Permissions::VIEW_CHANNEL).await {
        Ok(invite) => invite,
        Err(why) => {
            error!("Encountered an error while trying to generate an invite: {}", why);
            ctx.say("Failed to generate an invite {why}").await?;
            return Ok(());
        }
    };

    let name = &bot_user.name;
    let git_url = &ctx.data().config.read().await.git_url;

    ctx.send(|builder| {
        builder.embed(|embed| {
            embed
                .title(format!("Invite {name} to your server!"))
                .thumbnail(bot_user.avatar_url().unwrap_or_default())
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .description(format!("**Click [here]({invite_admin}) to add {name} to your Discord server.**\nOr click [here]({invite_generic}) for an invite where you can customise my permissions!"))
                .footer(|footer|footer.text("Use the first link where possible, the second is for those that know what they are doing, as the permissions requested are needed for some commands."));
            if let Some(git_url) = git_url {
                // Slightly customise our formatting so that it flows well.
                embed.title("Click here to view my source code!")
                .url(git_url)
                .description(format!("**And click [here]({invite_admin}) to add {name} to your Discord server!**\nYou can click [here]({invite_generic}) for an invite where you can customise my permissions."));
            };
            embed
        })
    })
    .await?;

    Ok(())
}
