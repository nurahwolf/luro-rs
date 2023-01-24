use luro_core::{Data, Error};
use luro_utilities::{discod_event_log_channel_defined, event_embed, guild_accent_colour};
use poise::serenity_prelude::{ChannelCategory, Context};

/// A Serenity listener for the [poise::Event::CategoryCreate] type
pub async fn category_create(
    ctx: &Context,
    user_data: &Data,
    accent_colour: [u8; 3],
    category: &ChannelCategory
) -> Result<(), Error> {
    let mut embed = event_embed(
        guild_accent_colour(accent_colour, category.guild_id.to_guild_cached(ctx)),
        None,
        None
    )
    .await;
    embed
        .title("Category Created")
        .description(format!("The category {} just got created", category.name()));

    if let Some(alert_channel) = discod_event_log_channel_defined(&category.guild_id, user_data, ctx).await {
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
