use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{Message, User};

use luro_core::{Context, Error};

use crate::{add_discord_message, total_messages_by_user, get_message_formatted};

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
    match message_id.parse::<u64>() {
        Ok(parsed_message_id) => {
            let embed = match get_message_formatted(ctx, parsed_message_id, hide).await {
                Ok(embed) => embed,
                Err(err) => {
                    ctx.say(err.to_string()).await?;
                    return Ok(());
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
