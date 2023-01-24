use crate::functions::deleted_message_formatted;
use luro_core::{Data, Error};
use luro_utilities::discod_event_log_channel_defined;
use poise::serenity_prelude::{ChannelId, Context, GuildId, MessageId};

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
            let message = deleted_message_formatted(
                ctx,
                accent_colour,
                &alert_channel,
                user_data,
                deleted_message_id.0,
                channel_id,
                false
            )
            .await;

            alert_channel
                .send_message(ctx, |m| {
                    *m = message;
                    m
                })
                .await?;
            return Ok(());
        }
    }

    Ok(())
}
