use luro_core::{Data, Error};
use luro_utilities::{discod_event_log_channel_defined, event_embed, guild_accent_colour};
use poise::serenity_prelude::{Context, Mentionable, Message, MessageUpdateEvent};
use tracing::debug;

/// A Serenity listener for the [poise::Event::MessageUpdate] type
pub async fn message_updated(
    ctx: &Context,
    user_data: &Data,
    accent_colour: [u8; 3],
    old_if_available: &Option<Message>,
    new: &Option<Message>,
    event: &MessageUpdateEvent
) -> Result<(), Error> {
    // Discard if the message is from a bot
    if let Some(author) = &event.author {
        if author.bot {
            debug!("Message edited was by a bot, so ignoring");
            return Ok(());
        }
    }

    if let Some(guild_id) = event.guild_id && event.content.is_some() {
        if let Some(alert_channel) = discod_event_log_channel_defined(&guild_id, user_data, ctx).await {
            let guild = guild_id.to_guild_cached(ctx);
            let mut embed = if let Some(new_message) = new {
                let mut embed = event_embed(guild_accent_colour(accent_colour, guild), None, Some(&new_message.author)).await;

                if new_message.content.is_empty() {
                    embed.description("<THE NEW MESSAGE DOES NOT HAVE A BODY>");
                } else {
                    embed.field("New Message", &new_message.content, false);
                    embed.description(&new_message.content);
                };
                embed.url(new_message.link());

                embed
            } else if let Some(old_message) = old_if_available {
                let mut embed = event_embed(guild_accent_colour(accent_colour, guild), None, Some(&old_message.author)).await;

                if old_message.content.is_empty() {
                    embed.description("<THE OLD MESSAGE DOES NOT HAVE A BODY>");
                } else {
                    embed.field("Old Message", &old_message.content, false);
                    embed.description(&old_message.content);
                }

                embed
            } else {
                match &event.author {
                    Some(message_user) => {
                        event_embed(guild_accent_colour(accent_colour, guild), None, Some(message_user)).await
                    }
                    None => event_embed(guild_accent_colour(accent_colour, guild), None, None).await
                }
            };

            embed.title("Message Edited");
            embed.field("Channel", event.channel_id.mention(), true);
            embed.field("Message ID", event.id, true);
            if let Some(message_content) = &event.content {
                embed.description(message_content);
            }

            if let Some(attachments) = &event.attachments {
                if let Some(first_attachment) = attachments.first() {
                    embed.image(&first_attachment.url);
                }
            }

            alert_channel
                .send_message(ctx, |message| {
                    message.add_embed(|e| {
                        *e = embed;
                        e
                    })
                })
                .await?;
        }
    }
    Ok(())
}
