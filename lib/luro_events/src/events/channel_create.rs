use crate::functions::event_embed;
use luro_core::{Data, Error};
use luro_utilities::{discod_event_log_channel_defined, guild_accent_colour};
use poise::serenity_prelude::{Context, GuildChannel, Mentionable};

/// A Serenity listener for the [poise::Event::ChannelCreate] type
pub async fn channel_create(
    ctx: &Context,
    user_data: &Data,
    accent_colour: [u8; 3],
    channel: &GuildChannel
) -> Result<(), Error> {
    let mut embed = event_embed(guild_accent_colour(accent_colour, channel.guild(ctx)), None, None).await;
    embed.title("Channel Created");
    embed.description(format!("The channel {} just got created", channel.mention()));

    if let Some(alert_channel) = discod_event_log_channel_defined(&channel.guild_id, user_data, ctx).await {
        alert_channel
            .send_message(ctx, |message| {
                message.add_embed(|e| {
                    *e = embed;
                    e
                })
            })
            .await?;
    }
    Ok(())
}
