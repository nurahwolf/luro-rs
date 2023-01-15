use crate::{Context, Error};

use poise::serenity_prelude::{self as serenity, ExecuteWebhook, Message, User};
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
    let uwu = uwuify_str_sse(&msg.to_string());

    match user {
        Some(user) => {
            execute_webhook_as_user(ctx, &user, &uwu).await?;
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
    // If there is no `message.content`, tell the user off
    if message.content.is_empty() {
        ctx.say("You can't UwUify an empty messge, dork >:c").await?;

        return Ok(());
    }

    let uwu = uwuify_str_sse(&message.content.to_string());
    execute_webhook_as_user(ctx, &message.author, &uwu).await?;
    ctx.send(|f| f.content("Mirrored!").ephemeral(true)).await?;

    Ok(())
}

/// Note, this function panics if not ran in a guild!
pub async fn execute_webhook_as_user(ctx: Context<'_>, user: &User, uwu: &String) -> Result<(), Error> {
    let guild = match ctx.guild() {
        Some(guild) => guild,
        None => panic!("Not in a guild!")
    };

    let mut webhooks = ctx.channel_id().webhooks(ctx).await?;
    let message_author = guild.member(ctx, &user.id).await?;
    let mut build_webhook = ExecuteWebhook::default();
    build_webhook.content(uwu);
    build_webhook.username(message_author.display_name());
    build_webhook.avatar_url(message_author.avatar_url().unwrap_or_default());
    let webhook_name = &ctx.data().config.read().await.webhook_name;

    if !webhooks.iter().any(|w| w.name.contains(webhook_name)) || webhooks.is_empty() {
        ctx.channel_id().create_webhook(ctx, webhook_name).await?;
        webhooks = ctx.channel_id().webhooks(ctx).await?;
    }

    for webhook in webhooks {
        if webhook.name.contains(webhook_name) {
            webhook
                .execute(ctx, false, |w| {
                    w.content(uwu).username(message_author.display_name()).avatar_url(
                        message_author
                            .avatar_url()
                            .unwrap_or(message_author.user.avatar_url().unwrap_or_default())
                    )
                })
                .await?;
        }
    }

    Ok(())
}
