use poise::serenity_prelude::CacheHttp;

use crate::{database::get_discord_message, Context, Error, functions::guild_accent_colour::guild_accent_colour};

/// Get a message from the database
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn db_get(ctx: Context<'_>, #[description = "Message ID to get"] message_id: String) -> Result<(), Error> {
    match message_id.parse::<u64>() {
        Ok(parsed_message_id) => {
            let luro_message = get_discord_message(&ctx.data().database, parsed_message_id);

            if let Ok(user) = ctx.http().get_user(luro_message.user_id).await {
                ctx.send(|builder| {
                    builder.embed(|embed| {
                        embed
                            .author(|author| author.name(&user.name).icon_url(&user.avatar_url().unwrap_or_default()))
                            .color(guild_accent_colour(ctx.data().config.lock().unwrap().accent_colour, ctx.guild()))
                            .description(luro_message.message_content)
                    })
                })
                .await?;
            } else {
                ctx.say(format!(
                    "I found that message, but failed to resolve their user.\n**Channel ID:** {}\n**Message ID:** {}\n**User ID:** {}\n**Message Content:** {}",
                    luro_message.channel_id, luro_message.message_id, luro_message.user_id, luro_message.message_content
                ))
                .await?;
            };
        }
        Err(err) => {
            ctx.say(format!("Had a fucky wucky (you probably didn't pass just a number)\n{err}")).await?;
        }
    };

    Ok(())
}
