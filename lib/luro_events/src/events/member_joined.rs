use luro_core::{Data, Error};
use luro_utilities::{discod_event_log_channel_defined, event_embed, guild_accent_colour};
use poise::serenity_prelude::{Context, Member};

/// A Serenity listener for the [poise::Event::GuildMemberAddition] type
pub async fn member_joined(ctx: &Context, user_data: &Data, accent_colour: [u8; 3], member: &Member) -> Result<(), Error> {
    if let Some(alert_channel) = discod_event_log_channel_defined(&member.guild_id, user_data, ctx).await {
        let guild = member.guild_id.to_guild_cached(ctx);

        let mut embed = event_embed(guild_accent_colour(accent_colour, guild), Some(&member.user), None).await;
        embed.title("Member Joined").description(format!(
            "The user {} ({}) just joined the server!",
            member.display_name(),
            member.user.id.0
        ));

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
