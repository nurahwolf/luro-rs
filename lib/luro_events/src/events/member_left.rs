use crate::functions::event_embed;
use luro_core::{Data, Error};
use luro_utilities::{discod_event_log_channel_defined, guild_accent_colour};
use poise::serenity_prelude::{Context, GuildId, User};

/// A Serenity listener for the [poise::Event::GuildMemberRemoval] type
pub async fn member_left(
    ctx: &Context,
    user_data: &Data,
    accent_colour: [u8; 3],
    guild_id: &GuildId,
    user: &User
) -> Result<(), Error> {
    if let Some(alert_channel) = discod_event_log_channel_defined(guild_id, user_data, ctx).await {
        let mut embed = event_embed(
            guild_accent_colour(accent_colour, alert_channel.guild(ctx)),
            Some(user),
            None
        )
        .await;
        embed
            .title("Member Left")
            .description(format!("The user {} ({}) just left the server!", user, user.id.0));

        alert_channel
            .send_message(ctx, |message| {
                message.add_embed(|e| {
                    *e = embed;
                    e
                })
            })
            .await?;
    };
    Ok(())
}
