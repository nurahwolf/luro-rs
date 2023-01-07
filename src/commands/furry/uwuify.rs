use crate::{Context, Error};

use poise::serenity_prelude::{self as serenity, Message};
use std::vec;
use uwuifier::uwuify_str_sse;

/// UwU time!
#[poise::command(slash_command, prefix_command, category = "Furry")]
pub async fn uwu(
    ctx: Context<'_>,
    #[description = "The user I should uwu to be"] user: Option<serenity::User>,
    #[description = "What uwu should they say"]
    #[rest]
    msg: String
) -> Result<(), Error> {
    let guild = ctx.serenity_context().cache.guild(ctx.guild_id().unwrap()).unwrap();
    let uwu = uwuify_str_sse(&msg.to_string());

    match user {
        Some(user) => {
            let mut webhooks = ctx.channel_id().webhooks(ctx).await?;
            let member = guild.member(ctx, &user).await?;

            if !webhooks.iter().any(|w| w.name.contains(&"LuroHook")) || webhooks.is_empty() {
                ctx.channel_id().create_webhook(ctx, "LuroHook").await?;
                webhooks = ctx.channel_id().webhooks(ctx).await?;
            }

            for webhook in webhooks {
                if webhook.name.contains(&"LuroHook") {
                    webhook
                        .execute(ctx, false, |w| {
                            w.content(&uwu);

                            if !member.display_name().is_empty() {
                                w.username(&member.display_name());
                            } else {
                                w.username(&user.name);
                            }

                            if member.avatar.is_some() {
                                w.avatar_url(&member.avatar_url().unwrap());
                            } else {
                                w.avatar_url(
                                    &user
                                        .avatar_url()
                                        .unwrap_or("https://cdn.discordapp.com/avatars/267365356912246784/7d4ed643250f41f18d94fd8377841884.webp?size=1024".to_string())
                                );
                            }
                            w
                        })
                        .await?;
                }
            }

            ctx.send(|f| f.content("Mirrored!").ephemeral(true)).await?;
        }
        None => {
            ctx.say(&uwu).await?;
        }
    }

    Ok(())
}

/// UwUify a message!
#[poise::command(context_menu_command = "UwUify this message", category = "Furry")]
pub async fn uwuify(ctx: Context<'_>, #[description = "Message to be UwUified"] message: Message) -> Result<(), Error> {
    let guild = ctx.serenity_context().cache.guild(ctx.guild_id().unwrap()).unwrap();

    if message.content.is_empty() {
        ctx.say("You can't UwUify an empty messge, dork >:c").await?;

        return Ok(());
    }

    if message.webhook_id.is_some() {
        ctx.say(message.content).await?;

        return Ok(());
    }

    let uwu = uwuify_str_sse(&message.content.to_string());

    let mut webhooks = ctx.channel_id().webhooks(ctx).await?;
    let member = guild.member(ctx, &message.author).await?;

    if !webhooks.iter().any(|w| w.name.contains(&"LuroHook")) || webhooks.is_empty() {
        ctx.channel_id().create_webhook(ctx, "LuroHook").await?;
        webhooks = ctx.channel_id().webhooks(ctx).await?;
    }

    for webhook in webhooks {
        if webhook.name.contains(&"LuroHook") {
            webhook
                .execute(ctx, false, |w| {
                    w.content(&uwu);

                    if !member.display_name().is_empty() {
                        w.username(&member.display_name());
                    } else {
                        w.username(&message.author.name);
                    }

                    if member.avatar.is_some() {
                        w.avatar_url(&member.avatar_url().unwrap());
                    } else {
                        w.avatar_url(
                            &message
                                .author
                                .avatar_url()
                                .unwrap_or("https://cdn.discordapp.com/avatars/267365356912246784/7d4ed643250f41f18d94fd8377841884.webp?size=1024".to_string())
                        );
                    }
                    w
                })
                .await?;
        }

        ctx.send(|f| f.content("Mirrored!").ephemeral(true)).await?;
    }

    Ok(())
}
