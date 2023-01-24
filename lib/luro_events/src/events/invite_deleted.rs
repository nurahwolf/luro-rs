use luro_core::{Data, Error};
use luro_utilities::{discod_event_log_channel_defined, event_embed, guild_accent_colour};
use poise::serenity_prelude::{Context, InviteDeleteEvent};

/// A Serenity listener for the [poise::Event::InviteCreate] type
pub async fn invite_deleted(
    ctx: &Context,
    user_data: &Data,
    accent_colour: [u8; 3],
    invite: &InviteDeleteEvent
) -> Result<(), Error> {
    if let Some(guild_id) = invite.guild_id {
        let guild = guild_id.to_guild_cached(ctx);
        let mut embed = event_embed(guild_accent_colour(accent_colour, guild), None, None).await;
        embed.title("Invite Deleted");
        embed.description(format!("The invite {} just got deleted!", invite.code));

        if let Some(alert_channel) = discod_event_log_channel_defined(&guild_id, user_data, ctx).await {
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
