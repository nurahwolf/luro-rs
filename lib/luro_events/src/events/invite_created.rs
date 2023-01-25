use luro_core::{Data, Error};
use luro_utilities::{discod_event_log_channel_defined, event_embed, guild_accent_colour};
use poise::serenity_prelude::{Context, InviteCreateEvent};

/// A Serenity listener for the [poise::Event::InviteCreate] type
pub async fn invite_create(
    ctx: &Context,
    user_data: &Data,
    accent_colour: [u8; 3],
    invite: &InviteCreateEvent
) -> Result<(), Error> {
    if let Some(guild_id) = invite.guild_id {
        let guild = guild_id.to_guild_cached(ctx);
        let mut embed = if let Some(invite_user) = &invite.inviter {
            let mut embed = event_embed(guild_accent_colour(accent_colour, guild), None, Some(invite_user)).await;
            embed.description(format!(
                "The invite {} just got created by user {} - {}!",
                invite.code, invite_user.name, invite_user.id
            ));
            embed
        } else {
            let mut embed = event_embed(guild_accent_colour(accent_colour, guild), None, None).await;
            embed.description(format!("The invite {} just got created by an unknown user!", invite.code));
            embed
        };

        embed.title("Invite Created");

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
