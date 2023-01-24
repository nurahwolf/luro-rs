use crate::functions::event_embed;
use luro_core::{Data, Error};
use luro_utilities::{guild_accent_colour, moderator_actions_log_channel_defined};
use poise::serenity_prelude::{Context, GuildId, User};

/// A Serenity listener for the [poise::Event::GuildBanRemoval] type
pub async fn member_unbanned(
    ctx: &Context,
    user_data: &Data,
    accent_colour: [u8; 3],
    guild_id: &GuildId,
    unbanned_user: &User
) -> Result<(), Error> {
    let mut embed = event_embed(
        guild_accent_colour(accent_colour, guild_id.to_guild_cached(ctx)),
        None,
        Some(unbanned_user)
    )
    .await;
    embed.title("Member Unbanned").description(format!(
        "The user {} ({}) just got unbanned!",
        unbanned_user, unbanned_user.id.0
    ));

    if let Some(alert_channel) = moderator_actions_log_channel_defined(guild_id, user_data, ctx).await {
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
