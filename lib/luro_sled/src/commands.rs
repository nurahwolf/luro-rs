use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{CacheHttp, CreateEmbed, Message, User};

use luro_core::{Context, Error};

use crate::{add_discord_message, get_discord_message, total_messages_by_user};

/// Add a message to the database
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn add(ctx: Context<'_>, #[description = "Message to add to DB"] msg: Message) -> Result<(), Error> {
    match add_discord_message(&ctx.data().database, msg.clone()) {
        Ok(_) => {
            ctx.say(format!("**Added message!**\nID: {}\nMessage:\n{}", &msg.id.0, &msg.content))
                .await?
        }
        Err(err) => ctx.say(format!("We had a fucky wucky!!{err}")).await?
    };

    Ok(())
}

/// Get a message from the database
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn get(
    ctx: Context<'_>,
    #[description = "Message ID to get"] message_id: String,
    #[description = "Hide advanced information"]
    #[flag]
    hide: bool
) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;

    match message_id.parse::<u64>() {
        Ok(parsed_message_id) => {
            let luro_message = get_discord_message(&ctx.data().database, parsed_message_id);
            let message_resolved = ctx.serenity_context().http.get_message(luro_message.channel_id, luro_message.message_id).await;
            let mut embed = CreateEmbed::default();

            embed.description(&luro_message.message_content);
            embed.color(guild_accent_colour(accent_colour, ctx.guild()));
            embed.footer(|footer| footer.text("This message was fetched from the database, so most likely no longer exists"));

            if let Ok(message_user) = ctx.http().get_user(luro_message.user_id).await {
                embed.author(|author| {
                    author
                        .name(&message_user.name)
                        .icon_url(&message_user.avatar_url().unwrap_or_default())
                });
            }

            if !hide {
                embed.field("Message ID", &luro_message.message_id, true);
                embed.field("Channel ID", &luro_message.channel_id, true);
                embed.field("User ID", &luro_message.user_id, true);

                if let Some(guild_id) = &luro_message.guild_id && message_resolved.is_err() {
                    embed.field("Guild ID", guild_id, true);
                }
            }

            if let Ok(message_resolved) = message_resolved {
                embed.footer(|footer| footer.text("This message was fully resolved, so it still exists in Discord"));
                embed.author(|author| {
                    author
                    .name(&message_resolved.author.name)
                    .icon_url(&message_resolved.author.avatar_url().unwrap_or_default())
                        .url(&message_resolved.link())
                });

                if let Some(guild) = message_resolved.guild(ctx) {
                    embed.footer(|footer| footer.icon_url(guild.icon_url().unwrap_or_default()).text(format!("{} - This message was fully resolved, so it still exists in Discord", guild.name)));
                } else {
                    if let Some(guild_id) = &luro_message.guild_id && !hide {
                        embed.field("Guild ID", guild_id, true);
                    }
                }
            };

            ctx.send(|builder| {
                builder.embed(|e| {
                    *e = embed;
                    e
                })
            })
            .await?;
        }
        Err(err) => {
            ctx.say(format!("Had a fucky wucky (you probably didn't pass just a number)\n{err}"))
                .await?;
        }
    };

    Ok(())
}

/// Total messages sent by a user
#[poise::command(prefix_command, slash_command, category = "Database")]
pub async fn total(ctx: Context<'_>, #[description = "User ID to get"] user: User) -> Result<(), Error> {
    let message_total = total_messages_by_user(&ctx.data().database, user.id.0);
    let accent_colour = ctx.data().config.read().await.accent_colour;

    ctx.send(|builder| {
        builder.embed(|embed| {
            embed
                .author(|author| author.name(&user.name).icon_url(&user.avatar_url().unwrap_or_default()))
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .description(format!("**Total messages sent by user {}**\n{}", &user, message_total))
        })
    })
    .await?;

    Ok(())
}
