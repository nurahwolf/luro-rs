use crate::{utils::guild_accent_colour, Context, Error};

use poise::serenity_prelude::{CacheHttp, Channel, CreateEmbed};

/// Get a message. Note, this gets the message directly, NOT from the database!
#[poise::command(prefix_command, slash_command, category = "Guild")]
pub async fn message(
    ctx: Context<'_>,
    #[description = "The channel ID in which the message is present"] channel_query: Channel,
    #[description = "The message to get"] message_query: String
) -> Result<(), Error> {
    let message_id_resolved = match message_query.parse::<u64>() {
        Ok(parsed_message_id) => parsed_message_id,
        Err(err) => {
            ctx.say(format!("Had a fucky wucky (you probably didn't pass just a number)\n{err}")).await?;
            return Ok(());
        }
    };

    let message = match ctx.http().get_message(channel_query.id().0, message_id_resolved).await {
        Ok(ok) => ok,
        Err(err) => {
            ctx.say(format!("Failed to get the message for the following reason:\n{err}")).await?;
            return Ok(());
        }
    };

    let guild = message.guild(ctx);
    let member = message.member(ctx).await;

    // Create an embed for the data we wish to show, filling it with key data
    let mut embed = CreateEmbed::default();
    embed.colour(guild_accent_colour(ctx.data().config.lock().unwrap().accent_colour, guild.clone()));
    if message.content.is_empty() {
        embed.description("　");
    } else {
        embed.description(message.content);
    }

    // Get member profile picture, otherwise fall back to user
    if let Ok(member) = &member {
        embed.thumbnail(member.avatar_url().unwrap_or_default());
    } else {
        embed.thumbnail(message.author.avatar_url().unwrap_or_default());
    };

    // Set author
    if let Ok(member) = &member {
        embed.author(|author| author.icon_url(&member.avatar_url().unwrap_or_default()).name(&member.display_name()))
    } else {
        embed.author(|author| author.icon_url(&message.author.avatar_url().unwrap_or_default()).name(&message.author.name))
    };

    // Set Fields
    if let Ok(member) = &member {
        embed.field("Member", member, true);
    } else {
        embed.field("User", &message.author, true);
    };

    if let Some(guild) = &guild {
        embed.field("Guild", &guild.name, true);
    } else {
        embed.field("Channel", format!("Channel: <#{}>", &message.channel_id), true);
    };

    ctx.send(|builder| {
        builder.embed(|e| {
            *e = embed;
            e
        });
        builder
    })
    .await?;

    Ok(())
}
