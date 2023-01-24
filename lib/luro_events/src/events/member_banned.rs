use luro_core::{Data, Error};
use luro_utilities::{event_embed, guild_accent_colour, moderator_actions_log_channel_defined};
use poise::serenity_prelude::{Context, GuildId, User};

/// A Serenity listener for the [poise::Event::GuildBanAddition] type
pub async fn member_banned(
    ctx: &Context,
    user_data: &Data,
    accent_colour: [u8; 3],
    guild_id: &GuildId,
    banned_user: &User
) -> Result<(), Error> {
    let mut embed = event_embed(
        guild_accent_colour(accent_colour, guild_id.to_guild_cached(ctx)),
        None,
        Some(banned_user)
    )
    .await;
    embed
        .title("Member Banned")
        .description(format!("The user {} ({}) just got banned!", banned_user, banned_user.id.0));

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
