use poise::serenity_prelude::CacheHttp;

use crate::{database::get_discord_message, functions::guild_accent_colour::guild_accent_colour, Context, Error};

/// Get a message from the database
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn db_get(ctx: Context<'_>, #[description = "Message ID to get"] message_id: String) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;

    match message_id.parse::<u64>() {
        Ok(parsed_message_id) => {
            let luro_message = get_discord_message(&ctx.data().database, parsed_message_id);
            let message = match ctx.http().get_message(luro_message.channel_id, luro_message.message_id).await {
                Ok(message) => message,
                Err(_) => {
                    ctx.say(format!(
                        "I found that message, but failed to resolve their user.\n**Channel ID:** {}\n**Message ID:** {}\n**User ID:** {}\n**Message Content:** {}",
                        luro_message.channel_id, luro_message.message_id, luro_message.user_id, luro_message.message_content
                    ))
                    .await?;
                    return Ok(());
                },
            };

            ctx.send(|builder| {
                builder.embed(|embed| {
                    embed
                        .author(|author| author.name(&message.author.name).icon_url(&message.author.avatar_url().unwrap_or_default()))
                        .url(message.link())
                        .color(guild_accent_colour(accent_colour, ctx.guild()))
                        .description(luro_message.message_content);

                    if let Some(guild) = message.guild(ctx) {
                        embed.footer(|footer| footer.icon_url(guild.icon_url().unwrap_or_default()).text(guild.name));
                    };

                    embed
                })
            })
            .await?;
        }
        Err(err) => {
            ctx.say(format!("Had a fucky wucky (you probably didn't pass just a number)\n{err}")).await?;
        }
    };

    Ok(())
}
