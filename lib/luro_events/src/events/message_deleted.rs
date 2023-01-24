use luro_core::{Data, Error};
use luro_sled::get_discord_message;
use luro_utilities::{discod_event_log_channel_defined, event_embed, guild_accent_colour};
use poise::serenity_prelude::{ChannelId, Context, CreateEmbed, GuildId, MessageId};

/// A Serenity listener for the [poise::Event::MessageDelete] type
pub async fn message_deleted(
    ctx: &Context,
    user_data: &Data,
    accent_colour: [u8; 3],
    channel_id: &ChannelId,
    deleted_message_id: &MessageId,
    guild_id: &Option<GuildId>
) -> Result<(), Error> {
    if let Some(guild_id) = guild_id {
        if let Some(alert_channel) = discod_event_log_channel_defined(guild_id, user_data, ctx).await {
            let embed = match get_discord_message(&user_data.database, deleted_message_id.0) {
                Ok(luro_message) => {
                    // If the message content is empty, there is no point in returning a response.
                    if luro_message.message_content.is_empty() {
                        return Ok(());
                    };

                    let mut embed = match &ctx.http.get_user(luro_message.user_id).await {
                        Ok(user) => {
                            event_embed(guild_accent_colour(accent_colour, alert_channel.guild(ctx)), Some(user), None).await
                        }
                        Err(_) => event_embed(guild_accent_colour(accent_colour, alert_channel.guild(ctx)), None, None).await
                    };

                    embed.title("Message Deleted");
                    embed.description(&luro_message.message_content);
                    embed.color(guild_accent_colour(accent_colour, alert_channel.guild(ctx)));
                    embed.footer(|footer| {
                        footer.text("This message was fetched from the database, so most likely no longer exists")
                    });
                    embed.field("Message ID", luro_message.message_id, true);
                    embed.field("Channel ID", luro_message.channel_id, true);
                    embed.field("User ID", luro_message.user_id, true);
                    embed
                }
                Err(_) => {
                    let mut embed = CreateEmbed::default();
                    embed
                        .title("Message Deleted")
                        .description(format!(
                            "The message with ID {deleted_message_id} just got deleted in channel {channel_id}!"
                        ))
                        .color(guild_accent_colour(accent_colour, alert_channel.guild(ctx)));
                    embed
                }
            };

            alert_channel
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        *e = embed;
                        e
                    })
                })
                .await?;
            return Ok(());
        }
    }

    Ok(())
}
